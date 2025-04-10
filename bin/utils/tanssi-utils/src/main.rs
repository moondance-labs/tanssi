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
use serde::{Deserialize, Deserializer};
use snowbridge_outbound_queue_merkle_tree::merkle_proof;
use sp_runtime::{traits::Keccak256, AccountId32};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

#[derive(Deserialize, Debug, Clone)]
pub struct RewardData {
    #[serde(deserialize_with = "hex_to_account_id32")]
    account: AccountId32,
    amount: u32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RewardClaimInput {
    pub(crate) operator_rewards: Vec<RewardData>,
    pub(crate) era: u32,
}

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

#[derive(Parser, Debug)]
pub struct RewardClaimGeneratorCmd {
    /// The path where the json containing the values is located.
    #[arg(long, short)]
    pub input_path: PathBuf,
}

impl TanssiUtils {
    /// Executes the internal command.
    pub fn run(&self) {
        match &self.command {
            TanssiUtilsCmd::RewardClaimGenerator(cmd) => {
                println!("\nInput path is: {:?}\n", cmd.input_path);
                let rewards =
                    extract_rewards_data_from_file(&cmd.input_path).expect("command fail");
                generate_reward_utils(rewards)
            }
        }
    }
}

// Helper function to deserialize hex strings into AccountId32.
// Example: 0x040404...
fn hex_to_account_id32<'de, D>(deserializer: D) -> Result<AccountId32, D::Error>
where
    D: Deserializer<'de>,
{
    let hex_str: String = Deserialize::deserialize(deserializer)?;
    let hex_trimmed = hex_str.strip_prefix("0x").unwrap_or(hex_str.as_str());
    let bytes = hex::decode(hex_trimmed).map_err(serde::de::Error::custom)?;
    let mut array = [0u8; 32];
    array.copy_from_slice(&bytes);
    Ok(AccountId32::from(array))
}

/// Extract a set of rewards information from a JSON file.
fn extract_rewards_data_from_file(reward_path: &Path) -> Result<RewardClaimInput, String> {
    let reader = std::fs::File::open(reward_path).expect("Can open file");
    let reward_input = serde_json::from_reader(&reader).expect("Cant parse reward input from JSON");
    Ok(reward_input)
}

fn generate_reward_utils(reward_input: RewardClaimInput) {
    let era_index = reward_input.era;
    let mut total_points = 0;
    let individual_rewards: BTreeMap<_, _> = reward_input
        .operator_rewards
        .clone()
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

    let mut show_general_info = true;
    reward_input.operator_rewards.iter().for_each(|reward| {
        if let Some(account_utils) = era_rewards
            .generate_era_rewards_utils::<Keccak256>(era_index, Some(reward.account.clone()))
        {
            // Only show the general info once
            if show_general_info {
                println!("=== Era Rewards Utils: Overall info ===\n");
                println!("Era index       : {:?}", era_index);
                println!("Merkle Root     : {:?}", account_utils.rewards_merkle_root);
                println!("Total Points    : {}", account_utils.total_points);
                println!("Leaves:");
                for (i, leaf) in account_utils.leaves.iter().enumerate() {
                    println!("  [{}] {:?}", i, leaf);
                }
                show_general_info = false;

                println!("\n=== Merkle Proofs ===");
            }

            let merkle_proof = account_utils
                .leaf_index
                .map(|index| merkle_proof::<Keccak256, _>(account_utils.leaves.into_iter(), index));

            if let Some(proof) = merkle_proof {
                println!(
                    "\nMerkle proof for account {:?} in era {:?}: \n",
                    reward.account, era_index
                );
                println!("   - Root: {:?}", proof.root);
                println!("   - Proof: {:?}", proof.proof);
                println!("   - Number of leaves: {:?}", proof.number_of_leaves);
                println!("   - Leaf index: {:?}", proof.leaf_index);
                println!("   - Leaf: {:?}", proof.leaf);
            } else {
                println!("No proof generated for account {:?}", reward.account);
            }
        } else {
            println!("No utils generated for account {:?}", reward.account);
        };
    });
}

fn main() {
    // Parses the options
    let cmd = TanssiUtils::parse();
    cmd.run();
}
