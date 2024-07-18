// Copyright (C) Moondance Labs Ltd.
// This file is part of Tanssi.

// Tanssi is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Tanssi is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Tanssi.  If not, see <http://www.gnu.org/licenses/>

#![cfg(test)]

use {
    crate::common::*,
    frame_support::assert_ok,
    sp_std::vec,
    starlight_runtime::{
        CollatorConfiguration, ContainerRegistrar, TanssiAuthorityMapping, TanssiInvulnerables,
    },
};

mod common;

const UNIT: Balance = 1_000_000_000_000_000_000;

struct TestConfig {
    backed_and_concluding: BTreeMap<u32, u32>,
    fill_claimqueue: bool,
}

fn make_inherent_data(
    TestConfig {
        backed_and_concluding,
        fill_claimqueue,
    }: TestConfig,
) -> Bench<Test> {
    let extra_cores = elastic_paras
        .values()
        .map(|count| *count as usize)
        .sum::<usize>()
        .saturating_sub(elastic_paras.len() as usize);
    let total_cores = dispute_sessions.len() + backed_and_concluding.len() + extra_cores;

    let builder = BenchBuilder::<Test>::new()
        .set_backed_and_concluding_paras(backed_and_concluding.clone())
        .set_fill_claimqueue(fill_claimqueue);
}

/// Paras inherent `enter` scenario builder.
pub(crate) struct  TestBuilder<T: paras_inherent::Config> {
	/// Active validators. Validators should be declared prior to all other setup.
	validators: Option<IndexedVec<ValidatorIndex, ValidatorId>>,
	/// Starting block number; we expect it to get incremented on session setup.
	block_number: BlockNumberFor<T>,
	/// Starting session; we expect it to get incremented on session setup.
	session: SessionIndex,
	/// Session we want the scenario to take place in. We will roll to this session.
	target_session: u32,
	/// Optionally set the max validators per core; otherwise uses the configuration value.
	max_validators_per_core: Option<u32>,
	/// Optionally set the max validators; otherwise uses the configuration value.
	max_validators: Option<u32>,
	/// Optionally set the number of dispute statements for each candidate.
	dispute_statements: BTreeMap<u32, u32>,
	/// Session index of for each dispute. Index of slice corresponds to a core,
	/// which is offset by the number of entries for `backed_and_concluding_paras`. I.E. if
	/// `backed_and_concluding_paras` has 3 entries, the first index of `dispute_sessions`
	/// will correspond to core index 3. There must be one entry for each core with a dispute
	/// statement set.
	dispute_sessions: Vec<u32>,
	/// Map from para id to number of validity votes. Core indices are generated based on
	/// `elastic_paras` configuration. Each para id in `elastic_paras` gets the
	/// specified amount of consecutive cores assigned to it. If a para id is not present
	/// in `elastic_paras` it get assigned to a single core.
	backed_and_concluding_paras: BTreeMap<u32, u32>,
	/// Map from para id (seed) to number of chained candidates.
	elastic_paras: BTreeMap<u32, u8>,
	/// Make every candidate include a code upgrade by setting this to `Some` where the interior
	/// value is the byte length of the new code.
	code_upgrade: Option<u32>,
	/// Specifies whether the claimqueue should be filled.
	fill_claimqueue: bool,
	/// Cores which should not be available when being populated with pending candidates.
	unavailable_cores: Vec<u32>,
	_phantom: sp_std::marker::PhantomData<T>,
}

#[allow(dead_code)]
impl<T: paras_inherent::Config> BenchBuilder<T> {
	/// Create a new `BenchBuilder` with some opinionated values that should work with the rest
	/// of the functions in this implementation.
	pub(crate) fn new() -> Self {
		BenchBuilder {
			validators: None,
			block_number: Zero::zero(),
			session: SessionIndex::from(0u32),
			target_session: 2u32,
			max_validators_per_core: None,
			max_validators: None,
			dispute_statements: BTreeMap::new(),
			dispute_sessions: Default::default(),
			backed_and_concluding_paras: Default::default(),
			elastic_paras: Default::default(),
			code_upgrade: None,
			fill_claimqueue: true,
			unavailable_cores: vec![],
			_phantom: sp_std::marker::PhantomData::<T>,
		}
	}

    /// Build a scenario for testing or benchmarks.
	///
	/// Note that this API only allows building scenarios where the `backed_and_concluding_paras`
	/// are mutually exclusive with the cores for disputes. So
	/// `backed_and_concluding_paras.len() + dispute_sessions.len()` must be less than the max
	/// number of cores.
	pub(crate) fn build(self) -> ParachainsInherentData {
		// Make sure relevant storage is cleared. This is just to get the asserts to work when
		// running tests because it seems the storage is not cleared in between.
		#[allow(deprecated)]
		inclusion::PendingAvailability::<T>::remove_all(None);

		// We don't allow a core to have both disputes and be marked fully available at this block.
		let max_cores = self.max_cores() as usize;

		let extra_cores = self
			.elastic_paras
			.values()
			.map(|count| *count as usize)
			.sum::<usize>()
			.saturating_sub(self.elastic_paras.len() as usize);

		let used_cores =
			self.dispute_sessions.len() + self.backed_and_concluding_paras.len() + extra_cores;

		assert!(used_cores <= max_cores);
		let fill_claimqueue = self.fill_claimqueue;

		// NOTE: there is an n+2 session delay for these actions to take effect.
		// We are currently in Session 0, so these changes will take effect in Session 2.
		Self::setup_para_ids(used_cores - extra_cores);
		configuration::ActiveConfig::<T>::mutate(|c| {
			c.scheduler_params.num_cores = used_cores as u32;
		});

		let validator_ids = Self::generate_validator_pairs(self.max_validators());
		let target_session = SessionIndex::from(self.target_session);
		let builder = self.setup_session(target_session, validator_ids, used_cores, extra_cores);

		let bitfields = builder.create_availability_bitfields(
			&builder.backed_and_concluding_paras,
			&builder.elastic_paras,
			used_cores,
		);
		let backed_candidates = builder.create_backed_candidates(
			&builder.backed_and_concluding_paras,
			&builder.elastic_paras,
			builder.code_upgrade,
		);

		let disputes = builder.create_disputes(
			builder.backed_and_concluding_paras.len() as u32,
			(used_cores - extra_cores) as u32,
			builder.dispute_sessions.as_slice(),
		);
		let mut disputed_cores = (builder.backed_and_concluding_paras.len() as u32..
			((used_cores - extra_cores) as u32))
			.into_iter()
			.map(|idx| (idx, 0))
			.collect::<BTreeMap<_, _>>();

		let mut all_cores = builder.backed_and_concluding_paras.clone();
		all_cores.append(&mut disputed_cores);

		assert_eq!(inclusion::PendingAvailability::<T>::iter().count(), used_cores - extra_cores);

		// Mark all the used cores as occupied. We expect that there are
		// `backed_and_concluding_paras` that are pending availability and that there are
		// `used_cores - backed_and_concluding_paras ` which are about to be disputed.
		let now = frame_system::Pallet::<T>::block_number() + One::one();

		let mut core_idx = 0u32;
		let elastic_paras = &builder.elastic_paras;
		// Assign potentially multiple cores to same parachains,
		let cores = all_cores
			.iter()
			.flat_map(|(para_id, _)| {
				(0..elastic_paras.get(&para_id).cloned().unwrap_or(1))
					.map(|_para_local_core_idx| {
						let ttl = configuration::ActiveConfig::<T>::get().scheduler_params.ttl;
						// Load an assignment into provider so that one is present to pop
						let assignment =
							<T as scheduler::Config>::AssignmentProvider::get_mock_assignment(
								CoreIndex(core_idx),
								ParaId::from(*para_id),
							);
						core_idx += 1;
						CoreOccupied::Paras(ParasEntry::new(assignment, now + ttl))
					})
					.collect::<Vec<CoreOccupied<_>>>()
			})
			.collect::<Vec<CoreOccupied<_>>>();

		scheduler::AvailabilityCores::<T>::set(cores);

		core_idx = 0u32;
		if fill_claimqueue {
			let cores = all_cores
				.keys()
				.flat_map(|para_id| {
					(0..elastic_paras.get(&para_id).cloned().unwrap_or(1))
						.filter_map(|_para_local_core_idx| {
							let ttl = configuration::ActiveConfig::<T>::get().scheduler_params.ttl;
							// Load an assignment into provider so that one is present to pop
							let assignment =
								<T as scheduler::Config>::AssignmentProvider::get_mock_assignment(
									CoreIndex(core_idx),
									ParaId::from(*para_id),
								);

							let entry = (
								CoreIndex(core_idx),
								[ParasEntry::new(assignment, now + ttl)].into(),
							);
							let res = if builder.unavailable_cores.contains(&core_idx) {
								None
							} else {
								Some(entry)
							};
							core_idx += 1;
							res
						})
						.collect::<Vec<(CoreIndex, VecDeque<ParasEntry<_>>)>>()
				})
				.collect::<BTreeMap<CoreIndex, VecDeque<ParasEntry<_>>>>();

			scheduler::ClaimQueue::<T>::set(cores);
		}

		Bench::<T> {
			data: ParachainsInherentData {
				bitfields,
				backed_candidates,
				disputes,
				parent_header: Self::header(builder.block_number),
			},
			_session: target_session,
			_block_number: builder.block_number,
		}
	}
}

#[test]
fn test_core_assignation_goes_to_paras() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
        ])
        .with_config(pallet_configuration::HostConfiguration {
            max_collators: 2,
            min_orchestrator_collators: 0,
            max_orchestrator_collators: 0,
            collators_per_container: 2,
            ..Default::default()
        })
        .with_para_ids(vec![(1000, empty_genesis_data(), u32::MAX, u32::MAX).into()])
        .build()
        .execute_with(|| {
            run_to_block(2);
            let payload: cumulus_primitives_core::relay_chain::AvailabilityBitfield =
                AvailabilityBitfield(bitvec![u8, bitvec::order::Lsb0; 1u8; 32]);
            set_new_inherent_data(cumulus_primitives_core::relay_chain::InherentData {
                bitfields: vec![payload],
                backed_candidates: vec![cumulus_primitives_core::relay_chain::BackedCandidate ::new(

                )
            })

        });
}
