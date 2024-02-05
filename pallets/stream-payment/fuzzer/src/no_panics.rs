mod mock;
use arbitrary::Arbitrary;
use honggfuzz::fuzz;
use mock::*;
use parity_scale_codec::{DecodeLimit, Encode};
use sp_core::{sr25519, Decode, Get, Pair, Public};
use frame_support::weights::constants::WEIGHT_REF_TIME_PER_SECOND;
use frame_support::pallet_prelude::Weight;
use sp_runtime::traits::Dispatchable;

/// The maximum number of blocks per fuzzer input.
/// If set to 0, then there is no limit at all.
/// Feel free to set this to a low number (e.g. 4) when you begin your fuzzing campaign and then set it back to 32 once you have good coverage.
const MAX_BLOCKS_PER_INPUT: usize = 32;

/// The maximum number of extrinsics per block.
/// If set to 0, then there is no limit at all.
/// Feel free to set this to a low number (e.g. 4) when you begin your fuzzing campaign and then set it back to 0 once you have good coverage.
const MAX_EXTRINSICS_PER_BLOCK: usize = 0;

/// Max number of seconds a block should run for.
const MAX_TIME_FOR_BLOCK: u64 = 6;

// We do not skip more than DEFAULT_STORAGE_PERIOD to avoid pallet_transaction_storage from
// panicking on finalize.
const MAX_BLOCK_LAPSE: u32 = 1000;

const NUM_ORIGINS: u32 = 8;

// Extrinsic delimiter: `********`
const DELIMITER: [u8; 8] = [42; 8];

struct Data<'a> {
    data: &'a [u8],
    pointer: usize,
    size: usize,
}

impl<'a> Data<'a> {
    fn size_limit_reached(&self) -> bool {
        !(MAX_BLOCKS_PER_INPUT == 0 || MAX_EXTRINSICS_PER_BLOCK == 0)
            && self.size >= MAX_BLOCKS_PER_INPUT * MAX_EXTRINSICS_PER_BLOCK
    }
}

impl<'a> Iterator for Data<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        if self.data.len() <= self.pointer || self.size_limit_reached() {
            return None;
        }
        let next_delimiter = self.data[self.pointer..]
            .windows(DELIMITER.len())
            .position(|window| window == DELIMITER);
        let next_pointer = match next_delimiter {
            Some(delimiter) => self.pointer + delimiter,
            None => self.data.len(),
        };
        let res = Some(&self.data[self.pointer..next_pointer]);
        self.pointer = next_pointer + DELIMITER.len();
        self.size += 1;
        res
    }
}

#[derive(Debug, Encode, Decode)]
enum ExtrOrPseudo {
    Extr(RuntimeCall),
    Pseudo(FuzzRuntimeCall),
}

#[derive(Debug, Encode, Decode)]
enum FuzzRuntimeCall {
    // TODO: fill this with combinations of extrinsics that are hard to get randomly, such as update_all_streams which
    // would need to read from storage
    NewBlock,
}

fn fuzz_main(data: &[u8]) {
    let iteratable = Data {
        data,
        pointer: 0,
        size: 0,
    };

    // Max weight for a block.
    let max_weight: Weight = Weight::from_parts(WEIGHT_REF_TIME_PER_SECOND * 2, 0);

    let mut block_count = 0;
    let mut extrinsics_in_block = 0;
    let mut extrinsics: Vec<(Option<u32>, usize, ExtrOrPseudo)> = vec![];
    let iterable = iteratable.filter_map(|data| {
        // We have reached the limit of block we want to decode
        if MAX_BLOCKS_PER_INPUT != 0 && block_count >= MAX_BLOCKS_PER_INPUT {
            return None;
        }
        // lapse is u32 (4 bytes), origin is u16 (2 bytes) -> 6 bytes minimum
        let min_data_len = 4 + 2;
        if data.len() <= min_data_len {
            return None;
        }
        let lapse: u32 = u32::from_le_bytes(data[0..4].try_into().unwrap());
        let origin: usize = u16::from_le_bytes(data[4..6].try_into().unwrap()) as usize;
        let mut encoded_extrinsic: &[u8] = &data[6..];

        // If the lapse is in the range [1, MAX_BLOCK_LAPSE] it is valid.
        let maybe_lapse = match lapse {
            1..=MAX_BLOCK_LAPSE => Some(lapse),
            _ => None,
        };
        // We have reached the limit of extrinsics for this block
        if maybe_lapse.is_none()
            && MAX_EXTRINSICS_PER_BLOCK != 0
            && extrinsics_in_block >= MAX_EXTRINSICS_PER_BLOCK
        {
            return None;
        }

        if let Some(mut pseudo_extrinsic) = encoded_extrinsic.strip_prefix(b"\xff\xff\xff\xff") {
            match FuzzRuntimeCall::decode_with_depth_limit(64, &mut pseudo_extrinsic) {
                Ok(decoded_extrinsic) => {
                    if maybe_lapse.is_some() {
                        block_count += 1;
                        extrinsics_in_block = 1;
                    } else {
                        extrinsics_in_block += 1;
                    }
                    // We have reached the limit of block we want to decode
                    if MAX_BLOCKS_PER_INPUT != 0 && block_count >= MAX_BLOCKS_PER_INPUT {
                        return None;
                    }
                    return Some((maybe_lapse, origin, ExtrOrPseudo::Pseudo(decoded_extrinsic)));
                }
                Err(_) => return None,
            }
        }

        match DecodeLimit::decode_with_depth_limit(64, &mut encoded_extrinsic) {
            Ok(decoded_extrinsic) => {
                if maybe_lapse.is_some() {
                    block_count += 1;
                    extrinsics_in_block = 1;
                } else {
                    extrinsics_in_block += 1;
                }
                // We have reached the limit of block we want to decode
                if MAX_BLOCKS_PER_INPUT != 0 && block_count >= MAX_BLOCKS_PER_INPUT {
                    return None;
                }
                Some((maybe_lapse, origin, ExtrOrPseudo::Extr(decoded_extrinsic)))
            }
            Err(_) => None,
        }
    });
    extrinsics.extend(iterable);

    if extrinsics.is_empty() {
        return;
    }

    ExtBuilder::default().with_balances(
        (0u64..(NUM_ORIGINS as u64)).map(|origin| (origin, 1 * DEFAULT_BALANCE)).collect()
    ).build().execute_with(|| {
        let initial_issuance = pallet_balances::TotalIssuance::<Runtime>::get();
        for (maybe_lapse, origin, extrinsic) in extrinsics {
            if let ExtrOrPseudo::Pseudo(pseudo) = &extrinsic {
                continue;
            }

            if let Some(lapse) = maybe_lapse {
                do_lapse(lapse as u64);
            }

            let extrinsic = if let ExtrOrPseudo::Extr(x) = extrinsic { x } else { unreachable!() };
            let origin = RuntimeOrigin::signed((origin % (NUM_ORIGINS as usize)) as u64);
            #[cfg(not(fuzzing))]
            {
                println!("\n    origin:     {:?}", origin);
                println!("    call:       {:?}", extrinsic);
            }
            let _res = extrinsic.dispatch(origin);
            #[cfg(not(fuzzing))]
            println!("    result:     {:?}", _res);
        }
        let final_issuance = pallet_balances::TotalIssuance::<Runtime>::get();

        assert_eq!(initial_issuance, final_issuance);
    })
}

fn main() {
    // Here you can parse `std::env::args and
    // setup / initialize your project

    // You have full control over the loop but
    // you're supposed to call `fuzz` ad vitam aeternam
    loop {
        // The fuzz macro gives an arbitrary object (see `arbitrary crate`)
        // to a closure-like block of code.
        // For performance reasons, it is recommended that you use the native type
        // `&[u8]` when possible.
        // Here, this slice will contain a "random" quantity of "random" data.
        fuzz!(|data: &[u8]| { fuzz_main(data) });
    }
}
