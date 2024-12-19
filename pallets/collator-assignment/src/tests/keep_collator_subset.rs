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

use {
    crate::{assignment::Assignment, tests::Test},
    sp_runtime::Perbill,
    tp_traits::FullRotationMode,
};

#[test]
fn keep_subset_keep_50_percent() {
    let mut collators = vec![1, 2, 3, 4, 5];
    let mut shuffle_count = 0;

    let mut shuffle = |_collators: &mut Vec<u64>| {
        shuffle_count += 1;
    };

    let full_rotation_mode = FullRotationMode::KeepPerbill {
        percentage: Perbill::from_percent(50),
    };
    let max_collators = 5;
    Assignment::<Test>::keep_collator_subset(
        Some(&mut collators),
        full_rotation_mode,
        max_collators,
        Some(&mut shuffle),
    );

    // 50% of 5 is 2
    assert_eq!(collators.len(), 2);
    assert_eq!(shuffle_count, 1);
}

#[test]
fn keep_subset_keep_2_collators() {
    let mut collators = vec![1, 2, 3, 4, 5];
    let mut shuffle_count = 0;

    let mut shuffle = |_collators: &mut Vec<u64>| {
        shuffle_count += 1;
    };

    let full_rotation_mode = FullRotationMode::KeepCollators { keep: 2 };
    let max_collators = 5;
    Assignment::<Test>::keep_collator_subset(
        Some(&mut collators),
        full_rotation_mode,
        max_collators,
        Some(&mut shuffle),
    );

    assert_eq!(collators.len(), 2);
    assert_eq!(shuffle_count, 1);
}

#[test]
fn keep_subset_rotate_all() {
    let mut collators = vec![1, 2, 3, 4, 5];
    let mut shuffle_count = 0;

    let mut shuffle = |_collators: &mut Vec<u64>| {
        shuffle_count += 1;
    };

    let full_rotation_mode = FullRotationMode::RotateAll;
    let max_collators = 5;
    Assignment::<Test>::keep_collator_subset(
        Some(&mut collators),
        full_rotation_mode,
        max_collators,
        Some(&mut shuffle),
    );

    assert_eq!(collators.len(), 0);
    assert_eq!(shuffle_count, 0);
}

#[test]
fn keep_subset_keep_all() {
    let mut collators = vec![1, 2, 3, 4, 5];
    let mut shuffle_count = 0;

    let mut shuffle = |_collators: &mut Vec<u64>| {
        shuffle_count += 1;
    };

    let full_rotation_mode = FullRotationMode::KeepAll;
    let max_collators = 5;
    Assignment::<Test>::keep_collator_subset(
        Some(&mut collators),
        full_rotation_mode,
        max_collators,
        Some(&mut shuffle),
    );

    assert_eq!(collators.len(), 5);
    assert_eq!(shuffle_count, 0);
}

#[test]
fn keep_subset_empty_collators() {
    let mut collators = vec![];
    let mut shuffle_count = 0;

    let mut shuffle = |_collators: &mut Vec<u64>| {
        shuffle_count += 1;
    };

    let full_rotation_mode = FullRotationMode::KeepCollators { keep: 2 };
    let max_collators = 5;
    Assignment::<Test>::keep_collator_subset(
        Some(&mut collators),
        full_rotation_mode.clone(),
        max_collators,
        Some(&mut shuffle),
    );
    assert_eq!(collators.len(), 0);

    // Calling this with None does not panic
    Assignment::<Test>::keep_collator_subset(
        None,
        full_rotation_mode,
        max_collators,
        Some(&mut shuffle),
    );
    assert_eq!(shuffle_count, 0);
}

#[test]
fn keep_subset_keep_more_than_max() {
    // Trying to keep more collators than the max keeps all of them and does not panic
    let mut collators = vec![1, 2, 3, 4, 5];
    let mut shuffle_count = 0;

    let mut shuffle = |_collators: &mut Vec<u64>| {
        shuffle_count += 1;
    };

    let full_rotation_mode = FullRotationMode::KeepCollators { keep: 200 };
    let max_collators = 5;
    Assignment::<Test>::keep_collator_subset(
        Some(&mut collators),
        full_rotation_mode.clone(),
        max_collators,
        Some(&mut shuffle),
    );
    assert_eq!(collators.len(), 5);
    assert_eq!(shuffle_count, 0);
}
