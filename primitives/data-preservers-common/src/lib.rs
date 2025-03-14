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

use core::marker::PhantomData;
use frame_support::dispatch::DispatchErrorWithPostInfo;
use frame_support::traits::OriginTrait;
use frame_support::{Deserialize, Serialize};
use parity_scale_codec::MaxEncodedLen;
use primitives::{AccountId, Balance};
use tp_stream_payment_common::StreamId;
use tp_traits::{apply, derive_storage_traits};

pub type StreamConfig = pallet_stream_payment::StreamConfig<
    tp_stream_payment_common::TimeUnit,
    tp_stream_payment_common::AssetId,
    primitives::Balance,
>;

type RuntimeOriginOf<Runtime> = <Runtime as frame_system::Config>::RuntimeOrigin;

tp_traits::alias!(
    pub trait RuntimeConfigs:
        frame_system::Config<AccountId = AccountId> +
        pallet_balances::Config<Balance = Balance> +
        pallet_stream_payment::Config<
            TimeUnit = tp_stream_payment_common::TimeUnit,
            AssetId = tp_stream_payment_common::AssetId,
            Balance = Balance,
            StreamId = tp_stream_payment_common::StreamId,
        > +
        pallet_data_preservers::Config
);

#[apply(derive_storage_traits)]
#[derive(Copy, Serialize, Deserialize, MaxEncodedLen)]
pub enum ProviderRequest {
    Free,
    StreamPayment { config: StreamConfig },
}

#[apply(derive_storage_traits)]
#[derive(Copy, Serialize, Deserialize)]
pub enum AssignerExtra {
    Free,
    StreamPayment { initial_deposit: Balance },
}

#[apply(derive_storage_traits)]
#[derive(Copy, Serialize, Deserialize, MaxEncodedLen)]
pub enum AssignmentWitness {
    Free,
    StreamPayment { stream_id: StreamId },
}

pub struct AssignmentProcessor<Runtime>(PhantomData<Runtime>);

impl<Runtime: RuntimeConfigs> pallet_data_preservers::AssignmentProcessor<AccountId>
    for AssignmentProcessor<Runtime>
{
    /// Providers requests which kind of payment it accepts.
    type ProviderRequest = ProviderRequest;
    /// Extra parameter the assigner provides.
    type AssignerParameter = AssignerExtra;
    /// Represents the successful outcome of the assignment.
    type AssignmentWitness = AssignmentWitness;

    fn try_start_assignment(
        assigner: AccountId,
        provider: AccountId,
        request: &Self::ProviderRequest,
        extra: Self::AssignerParameter,
    ) -> Result<Self::AssignmentWitness, DispatchErrorWithPostInfo> {
        let witness = match (request, extra) {
            (Self::ProviderRequest::Free, Self::AssignerParameter::Free) => {
                Self::AssignmentWitness::Free
            }
            (
                Self::ProviderRequest::StreamPayment { config },
                Self::AssignerParameter::StreamPayment { initial_deposit },
            ) => {
                let stream_id = pallet_stream_payment::Pallet::<Runtime>::open_stream_returns_id(
                    assigner,
                    provider,
                    *config,
                    initial_deposit,
                )?;

                Self::AssignmentWitness::StreamPayment { stream_id }
            }
            _ => Err(
                pallet_data_preservers::Error::<Runtime>::AssignmentPaymentRequestParameterMismatch,
            )?,
        };

        Ok(witness)
    }

    fn try_stop_assignment(
        provider: AccountId,
        witness: Self::AssignmentWitness,
    ) -> Result<(), DispatchErrorWithPostInfo> {
        match witness {
            Self::AssignmentWitness::Free => (),
            Self::AssignmentWitness::StreamPayment { stream_id } => {
                pallet_stream_payment::Pallet::<Runtime>::close_stream(
                    RuntimeOriginOf::<Runtime>::signed(provider),
                    stream_id,
                )?;
            }
        }

        Ok(())
    }

    /// Return the values for a free assignment if it is supported.
    /// This is required to perform automatic migration from old Bootnodes storage.
    fn free_variant_values() -> Option<(
        Self::ProviderRequest,
        Self::AssignerParameter,
        Self::AssignmentWitness,
    )> {
        Some((
            Self::ProviderRequest::Free,
            Self::AssignerParameter::Free,
            Self::AssignmentWitness::Free,
        ))
    }

    // The values returned by the following functions should match with each other.
    #[cfg(feature = "runtime-benchmarks")]
    fn benchmark_provider_request() -> Self::ProviderRequest {
        ProviderRequest::Free
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn benchmark_assigner_parameter() -> Self::AssignerParameter {
        AssignerExtra::Free
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn benchmark_assignment_witness() -> Self::AssignmentWitness {
        AssignmentWitness::Free
    }
}
