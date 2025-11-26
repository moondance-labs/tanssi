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
// along with Tanssi.  If not, see <http://www.gnu.org/licenses/>.

use frame_support::__private::Get;
use frontier_template_emulated_chain::FrontierTemplateParaPallet;
use simple_template_emulated_chain::SimpleTemplateParaPallet;
use xcm::latest::NetworkId;
use xcm_emulator::{Chain, TestExt};

mod erc20_token_transfer_from_container_to_eth;
mod foreign_eth_token_to_container_chain_transfer;
mod native_container_chain_token_to_eth_transfer;
mod native_eth_token_to_container_chain_transfer;
mod reserver_transfers_polkadot_xcm;
mod xcm_message_exporter;

// TODO: when pallet_parameters supports genesis config, add this to genesis and remove this function
pub fn set_templates_relay_param_to_starlight() {
    use starlight_system_emulated_network::{
        FrontierTemplatePara as FrontierTemplate, SimpleTemplatePara as SimpleTemplate,
    };
    pub const STARLIGHT_GENESIS_HASH: [u8; 32] =
        hex_literal::hex!["dd6d086f75ec041b66e20c4186d327b23c8af244c534a2418de6574e8c041a60"];

    FrontierTemplate::execute_with(|| {
        let root_origin = <FrontierTemplate as Chain>::RuntimeOrigin::root();

        let a = container_chain_template_frontier_runtime::dynamic_params::xcm_config::RelayNetwork;
        let b = Some(xcm::latest::NetworkId::ByGenesis(STARLIGHT_GENESIS_HASH));
        let asdf = container_chain_template_frontier_runtime::RuntimeParameters::XcmConfig(container_chain_template_frontier_runtime::dynamic_params::xcm_config::Parameters::RelayNetwork(a, b));
        <FrontierTemplate as FrontierTemplateParaPallet>::Parameters::set_parameter(
            root_origin,
            asdf,
        )
        .unwrap();
    });

    SimpleTemplate::execute_with(|| {
        let root_origin = <SimpleTemplate as Chain>::RuntimeOrigin::root();

        let a = container_chain_template_simple_runtime::dynamic_params::xcm_config::RelayNetwork;
        let b = Some(xcm::latest::NetworkId::ByGenesis(STARLIGHT_GENESIS_HASH));
        let asdf = container_chain_template_simple_runtime::RuntimeParameters::XcmConfig(container_chain_template_simple_runtime::dynamic_params::xcm_config::Parameters::RelayNetwork(a, b));
        <SimpleTemplate as SimpleTemplateParaPallet>::Parameters::set_parameter(root_origin, asdf)
            .unwrap();
    });

    let ethereum_network_frontier = FrontierTemplate::execute_with(|| {
        ethereum_chain_id::<container_chain_template_frontier_runtime::EthereumNetwork>()
    });
    let ethereum_network_simple = SimpleTemplate::execute_with(|| {
        ethereum_chain_id::<container_chain_template_simple_runtime::EthereumNetwork>()
    });

    assert_eq!(ethereum_network_frontier, 1);
    assert_eq!(ethereum_network_simple, 1);
}

pub fn ethereum_chain_id<N: Get<NetworkId>>() -> u64 {
    match N::get() {
        NetworkId::Ethereum { chain_id } => chain_id,
        _ => panic!("Expected Ethereum NetworkId"),
    }
}
