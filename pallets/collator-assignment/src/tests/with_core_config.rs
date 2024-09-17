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

use crate::assignment::ChainNumCollators;
use crate::mock::*;
use crate::{CoreAllocationConfiguration, Pallet};
use sp_runtime::Perbill;
use tp_traits::ParaId;

fn create_blank_chain_num_collator(id: u32) -> ChainNumCollators {
    ChainNumCollators {
        para_id: ParaId::new(id),
        min_collators: 0,
        max_collators: 0,
    }
}

fn generate_parachains_and_parathreads(
    num_parachains: u32,
    num_parathreds: u32,
) -> (Vec<ChainNumCollators>, Vec<ChainNumCollators>) {
    let mut parachains = vec![];

    for i in 0..num_parachains {
        parachains.push(create_blank_chain_num_collator(i + 1));
    }

    let mut parathreads = vec![];
    for i in 0..num_parathreds {
        parathreads.push(create_blank_chain_num_collator(num_parachains + i + 1));
    }

    (parachains, parathreads)
}

#[test]
fn test_combine_paras_with_core_config() {
    let core_count = 10u32;
    let num_parachains = 7u32;
    let num_parathreads = 5u32;

    let (generated_parachains, generated_parathreads) =
        generate_parachains_and_parathreads(num_parachains, num_parathreads);

    // Table for testing
    // (config, tipping_chains, expected_resulting_chains, expected_to_be_sorted_by_tip, num_collators)
    let table = vec![
        (
            CoreAllocationConfiguration {
                core_count,
                max_parachain_percentage: Perbill::from_percent(0),
            },
            vec![1u32, 2],
            vec![
                create_blank_chain_num_collator(8),
                create_blank_chain_num_collator(9),
                create_blank_chain_num_collator(10),
                create_blank_chain_num_collator(11),
                create_blank_chain_num_collator(12),
            ],
            true,
            10u32,
        ),
        (
            CoreAllocationConfiguration {
                core_count,
                max_parachain_percentage: Perbill::from_percent(1),
            },
            vec![5, 8],
            vec![
                create_blank_chain_num_collator(8),
                create_blank_chain_num_collator(9),
                create_blank_chain_num_collator(10),
                create_blank_chain_num_collator(11),
                create_blank_chain_num_collator(12),
            ],
            true,
            100,
        ),
        (
            CoreAllocationConfiguration {
                core_count,
                max_parachain_percentage: Perbill::from_percent(2),
            },
            vec![1, 6, 9, 10],
            vec![
                create_blank_chain_num_collator(9),
                create_blank_chain_num_collator(10),
                create_blank_chain_num_collator(8),
                create_blank_chain_num_collator(11),
                create_blank_chain_num_collator(12),
            ],
            true,
            10,
        ),
        (
            CoreAllocationConfiguration {
                core_count,
                max_parachain_percentage: Perbill::from_percent(45),
            },
            vec![],
            vec![
                create_blank_chain_num_collator(1),
                create_blank_chain_num_collator(2),
                create_blank_chain_num_collator(3),
                create_blank_chain_num_collator(4),
                create_blank_chain_num_collator(8),
                create_blank_chain_num_collator(9),
                create_blank_chain_num_collator(10),
                create_blank_chain_num_collator(11),
                create_blank_chain_num_collator(12),
            ],
            true,
            100,
        ),
        (
            CoreAllocationConfiguration {
                core_count,
                max_parachain_percentage: Perbill::from_percent(50),
            },
            vec![4, 6, 7, 8, 12],
            vec![
                create_blank_chain_num_collator(4),
                create_blank_chain_num_collator(6),
                create_blank_chain_num_collator(7),
                create_blank_chain_num_collator(8),
                create_blank_chain_num_collator(12),
                create_blank_chain_num_collator(1),
                create_blank_chain_num_collator(2),
                create_blank_chain_num_collator(9),
                create_blank_chain_num_collator(10),
                create_blank_chain_num_collator(11),
            ],
            true,
            10,
        ),
        (
            CoreAllocationConfiguration {
                core_count,
                max_parachain_percentage: Perbill::from_percent(98),
            },
            vec![1, 12],
            vec![
                create_blank_chain_num_collator(1),
                create_blank_chain_num_collator(12),
                create_blank_chain_num_collator(2),
                create_blank_chain_num_collator(3),
                create_blank_chain_num_collator(4),
                create_blank_chain_num_collator(5),
                create_blank_chain_num_collator(6),
                create_blank_chain_num_collator(7),
                create_blank_chain_num_collator(8),
                create_blank_chain_num_collator(9),
                create_blank_chain_num_collator(10),
                create_blank_chain_num_collator(11),
            ],
            true,
            10,
        ),
        (
            CoreAllocationConfiguration {
                core_count,
                max_parachain_percentage: Perbill::from_percent(99),
            },
            vec![1, 12],
            vec![
                create_blank_chain_num_collator(1),
                create_blank_chain_num_collator(2),
                create_blank_chain_num_collator(3),
                create_blank_chain_num_collator(4),
                create_blank_chain_num_collator(5),
                create_blank_chain_num_collator(6),
                create_blank_chain_num_collator(7),
                create_blank_chain_num_collator(8),
                create_blank_chain_num_collator(9),
                create_blank_chain_num_collator(10),
                create_blank_chain_num_collator(11),
                create_blank_chain_num_collator(12),
            ],
            false,
            100,
        ),
        (
            CoreAllocationConfiguration {
                core_count,
                max_parachain_percentage: Perbill::from_percent(100),
            },
            vec![10],
            vec![
                create_blank_chain_num_collator(10),
                create_blank_chain_num_collator(1),
                create_blank_chain_num_collator(2),
                create_blank_chain_num_collator(3),
                create_blank_chain_num_collator(4),
                create_blank_chain_num_collator(5),
                create_blank_chain_num_collator(6),
                create_blank_chain_num_collator(7),
                create_blank_chain_num_collator(8),
                create_blank_chain_num_collator(9),
                create_blank_chain_num_collator(11),
                create_blank_chain_num_collator(12),
            ],
            true,
            10,
        ),
    ];

    new_test_ext().execute_with(|| {
        for (
            config,
            mut tipping_chains,
            result_chains,
            should_be_ordered_by_tip,
            number_of_collators,
        ) in table
        {
            MockData::mutate(|mock_data_config| {
                mock_data_config.chains_that_are_tipping =
                    tipping_chains.drain(..).map(Into::into).collect();
                mock_data_config.apply_tip = !mock_data_config.chains_that_are_tipping.is_empty();
            });

            let (chains, ordered_by_tip) = Pallet::<Test>::order_paras_with_core_config(
                generated_parachains.clone(),
                generated_parathreads.clone(),
                &config,
                1,
                number_of_collators,
                1,
                1,
            );

            assert_eq!(chains, result_chains);
            assert_eq!(ordered_by_tip, should_be_ordered_by_tip);
        }
    });
}
