use std::collections::BTreeMap;
use crate::mock::*;
use crate::pallet::CollatorContainerChain;

/// Returns a map of collator to assigned para id
pub fn get_test_result() -> TestResult {
    let mut r = TestResult::default();
    let assigned_collators = CollatorContainerChain::<Test>::get();
    
    for (para_id, collators) in assigned_collators.container_chains.iter() {
        if !collators.is_empty() {
            // Assuming that para ids >= 3000 are always parathreads, and the rest always parachains
            if para_id >= ParaId::from(3000u32) {
                r.parathreads_with_collators += 1;
            } else {
                r.parachains_with_collators += 1;
            }
        }
    }
    
    r
}

#[derive(Default)]
struct TestResult {
    parachains_with_collators: u32,
    parathreads_with_collators: u32,
}

fn run_test<F, R>(f: F) -> TestResult
where
    F: FnOnce(&mut Mocks) -> R,
{
    // With 0% max parachains, no parachains are assigned collators
    new_test_ext().execute_with(|| {
        run_to_block(1);

        MockData::mutate(|m| {
            // Default mock values for this test
            m.collators_per_container = 1;
            m.collators_per_parathread = 1;
            m.min_orchestrator_chain_collators = 1;
            m.max_orchestrator_chain_collators = 1;
            // Test-specific mock values
            f(m);
        });

        run_to_block(11);

        get_test_result()
    })
}

/*

Interesting parameters:

num_cores (1-100)
parachain_limit (0% - 100%)
^ can be combined into parachain_cores (0-100)
num_collators (1-100)
num_parachains (0-10)
num_parathreads (0-10)

Interesting outputs:
num_parachains with collators
num_parathreads with collators

Priority based on fee not tested here, needs to be tested somewhere else
Noting that if all parathreads have a high fee and parachains have low fee, still parachains have
priority because of the parachain limit %

These can be constant?

collators_per_parathread = 1
collators_per_container = 1
min_orchestrator_chain_collators = 1
max_orchestrator_chain_collators = 1

 */

mod enough_cores_enough_collators {
    use super::*;

    #[test]
    fn assign_zero_parachains() {
        // With 0% max parachains, no parachains are assigned collators
        let r = run_test(|m| {
            //m.num_cores = 100;
            //m.parachain_limit = 0%;
            m.collators = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
            m.container_chains = vec![1001, 1002, 1003, 1004, 1005];
            m.parathreads = vec![3001, 3002, 3003, 3004, 3005];
        });

        assert_eq!(r.parachains_with_collators, 0);
        assert_eq!(r.parathreads_with_collators, 5);
    }

    #[test]
    fn assign_one_parachain() {
        // With 1% max parachains and 100 cores, only 1 parachain is assigned collators
        let r = run_test(|m| {
            //m.num_cores = 100;
            //m.parachain_limit = 1%;
            m.collators = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
            m.container_chains = vec![1001, 1002, 1003, 1004, 1005];
            m.parathreads = vec![3001, 3002, 3003, 3004, 3005];
        });

        assert_eq!(r.parachains_with_collators, 1);
        assert_eq!(r.parathreads_with_collators, 5);
    }

    #[test]
    fn assign_half_parachains() {
        // With 50% max parachains and 100 cores, all parachains are assigned collators
        let r = run_test(|m| {
            //m.num_cores = 100;
            //m.parachain_limit = 50%;
            m.collators = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
            m.container_chains = vec![1001, 1002, 1003, 1004, 1005];
            m.parathreads = vec![3001, 3002, 3003, 3004, 3005];
        });

        assert_eq!(r.parachains_with_collators, 5);
        assert_eq!(r.parathreads_with_collators, 5);
    }

    #[test]
    fn assign_all_parachains() {
        // With 100% max parachains and 100 cores, all parachains are assigned collators
        let r = run_test(|m| {
            //m.num_cores = 100;
            //m.parachain_limit = 100%;
            m.collators = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
            m.container_chains = vec![1001, 1002, 1003, 1004, 1005];
            m.parathreads = vec![3001, 3002, 3003, 3004, 3005];
        });

        assert_eq!(r.parachains_with_collators, 5);
        assert_eq!(r.parathreads_with_collators, 5);
    }
}


mod five_cores_enough_collators {
    use super::*;

    #[test]
    fn assign_zero_parachains() {
        // With 0% max parachains, no parachains are assigned collators
        let r = run_test(|m| {
            //m.num_cores = 5;
            //m.parachain_limit = 0%;
            m.collators = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
            m.container_chains = vec![1001, 1002, 1003, 1004, 1005];
            m.parathreads = vec![3001, 3002, 3003, 3004, 3005];
        });

        assert_eq!(r.parachains_with_collators, 0);
        assert_eq!(r.parathreads_with_collators, 5);
    }

    #[test]
    fn assign_one_parachain() {
        // With 1% max parachains and 5 cores, no parachains are assigned collators
        let r = run_test(|m| {
            //m.num_cores = 5;
            //m.parachain_limit = 1%;
            m.collators = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
            m.container_chains = vec![1001, 1002, 1003, 1004, 1005];
            m.parathreads = vec![3001, 3002, 3003, 3004, 3005];
        });

        assert_eq!(r.parachains_with_collators, 0);
        assert_eq!(r.parathreads_with_collators, 5);
    }

    #[test]
    fn assign_half_parachains() {
        // With 50% max parachains and 5 cores, 2 parachains are assigned collators
        let r = run_test(|m| {
            //m.num_cores = 100;
            //m.parachain_limit = 50%;
            m.collators = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
            m.container_chains = vec![1001, 1002, 1003, 1004, 1005];
            m.parathreads = vec![3001, 3002, 3003, 3004, 3005];
        });

        assert_eq!(r.parachains_with_collators, 2);
        assert_eq!(r.parathreads_with_collators, 5);
    }
    
    #[test]
    fn assign_parachains_leave_one_core_for_parathreads() {
        // With 99% max parachains and 5 cores, 4 parachains are assigned collators, and the
        // remaining parathreads are all assigned to the last core
        let r = run_test(|m| {
            //m.num_cores = 5;
            //m.parachain_limit = 99%;
            m.collators = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
            m.container_chains = vec![1001, 1002, 1003, 1004, 1005];
            m.parathreads = vec![3001, 3002, 3003, 3004, 3005];
        });

        assert_eq!(r.parachains_with_collators, 4);
        assert_eq!(r.parathreads_with_collators, 5);
    }

    #[test]
    fn assign_all_parachains() {
        // With 100% max parachains and 5 cores, all parachains are assigned collators, but no parathreads
        let r = run_test(|m| {
            //m.num_cores = 5;
            //m.parachain_limit = 100%;
            m.collators = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
            m.container_chains = vec![1001, 1002, 1003, 1004, 1005];
            m.parathreads = vec![3001, 3002, 3003, 3004, 3005];
        });

        assert_eq!(r.parachains_with_collators, 5);
        assert_eq!(r.parathreads_with_collators, 0);
    }
}