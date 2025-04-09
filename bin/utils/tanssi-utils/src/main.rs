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

use clap::{Parser, Subcommand};
use serde::Deserialize;
use sp_runtime::{traits::Keccak256, AccountId32};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

#[derive(Deserialize, Debug, Clone)]
pub struct RewardData {
    account: AccountId32,
    amount: u32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RewardClaimInput {
    pub(crate) operator_rewards: Vec<RewardData>,
    pub(crate) era: u32,
}

/// A utility to easily create a chain spec definition.
#[derive(Debug, Parser)]
#[command(rename_all = "kebab-case", version, about)]
pub struct TanssiUtils {
    #[command(subcommand)]
    pub command: TanssiUtilsCmd,
}

#[derive(Debug, Subcommand)]
#[command(rename_all = "kebab-case")]
pub enum TanssiUtilsCmd {
    RewardClaimGenerator(RewardClaimGeneratorCmd),
}

/// Create a new chain spec by interacting with the provided runtime wasm blob.
#[derive(Parser, Debug)]
pub struct RewardClaimGeneratorCmd {
    /// The path where the json with the values is
    #[arg(long, short)]
    pub input_path: PathBuf,
}

impl TanssiUtils {
    /// Executes the internal command.
    pub fn run(&self) {
        match &self.command {
            TanssiUtilsCmd::RewardClaimGenerator(cmd) => {
                println!("input path is {:?}", cmd.input_path);
                let rewards =
                    extract_rewards_data_from_file(&cmd.input_path).expect("command fail");
                generate_reward_utils(rewards)
            }
        }
    }
}

/// Extract a set of
fn extract_rewards_data_from_file(reward_path: &Path) -> Result<RewardClaimInput, String> {
    let reader = std::fs::File::open(reward_path).expect("Can open file");
    let reward_input = serde_json::from_reader(&reader).expect("Cant parse reward input from JSON");
    Ok(reward_input)
}

fn generate_reward_utils(reward_input: RewardClaimInput) {
    let mut total_points = 0;
    let individual_rewards: BTreeMap<_, _> = reward_input
        .operator_rewards
        .into_iter()
        .map(|data| {
            total_points += data.amount;
            (data.account, data.amount)
        })
        .collect();
    let era_rewards = pallet_external_validators_rewards::EraRewardPoints::<AccountId32> {
        total: total_points,
        individual: individual_rewards,
    };
    let utils = era_rewards.generate_era_rewards_utils::<Keccak256>(reward_input.era, None);
    println!("utils are {:?}", utils);
}

fn main() {
    // Parses the options
    let cmd = TanssiUtils::parse();
    cmd.run();
}
