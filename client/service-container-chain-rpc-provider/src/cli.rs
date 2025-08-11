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

use node_common::chain_spec::Extensions;
use sc_cli::ChainSpec;

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type OrchestratorChainSpec = sc_service::GenericChainSpec<Extensions>;

pub struct EmbededOrchestratorCli(pub cumulus_client_cli::RunCmd);

fn load_spec(path: &str) -> std::result::Result<Box<dyn ChainSpec>, String> {
    Ok(Box::new(OrchestratorChainSpec::from_json_file(
        std::path::PathBuf::from(path),
    )?))
}

impl sc_cli::SubstrateCli for EmbededOrchestratorCli {
    fn impl_name() -> String {
        "Orchestrator embeded node".into()
    }

    fn impl_version() -> String {
        env!("SUBSTRATE_CLI_IMPL_VERSION").into()
    }

    fn description() -> String {
        "Orchestrator embeded node".into()
    }

    fn author() -> String {
        env!("CARGO_PKG_AUTHORS").into()
    }

    fn support_url() -> String {
        "https://github.com/paritytech/cumulus/issues/new".into()
    }

    fn copyright_start_year() -> i32 {
        2020
    }

    fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
        load_spec(id)
    }
}

impl sc_cli::DefaultConfigurationValues for EmbededOrchestratorCli {
    fn p2p_listen_port() -> u16 {
        30334
    }

    fn rpc_listen_port() -> u16 {
        9945
    }

    fn prometheus_listen_port() -> u16 {
        9616
    }
}

impl sc_cli::CliConfiguration<Self> for EmbededOrchestratorCli {
    fn shared_params(&self) -> &sc_cli::SharedParams {
        self.0.base.shared_params()
    }

    fn import_params(&self) -> Option<&sc_cli::ImportParams> {
        self.0.base.import_params()
    }

    fn network_params(&self) -> Option<&sc_cli::NetworkParams> {
        self.0.base.network_params()
    }

    fn keystore_params(&self) -> Option<&sc_cli::KeystoreParams> {
        self.0.base.keystore_params()
    }

    fn base_path(&self) -> sc_cli::Result<Option<sc_service::BasePath>> {
        Ok(self
            .shared_params()
            .base_path()?)
    }

    fn rpc_addr(&self, default_listen_port: u16) -> sc_cli::Result<Option<Vec<sc_cli::RpcEndpoint>>> {
        self.0.base.rpc_addr(default_listen_port)
    }

    fn prometheus_config(
        &self,
        default_listen_port: u16,
        chain_spec: &Box<dyn ChainSpec>,
    ) -> sc_cli::Result<Option<sc_service::config::PrometheusConfig>> {
        self.0
            .base
            .prometheus_config(default_listen_port, chain_spec)
    }

    fn init<F>(&self, _support_url: &String, _impl_version: &String, _logger_hook: F) -> sc_cli::Result<()>
    where
        F: FnOnce(&mut sc_cli::LoggerBuilder),
    {
        unreachable!("PolkadotCli is never initialized; qed");
    }

    fn chain_id(&self, is_dev: bool) -> sc_cli::Result<String> {
        self.0.base.chain_id(is_dev)
    }

    fn role(&self, is_dev: bool) -> sc_cli::Result<sc_service::Role> {
        self.0.base.role(is_dev)
    }

    fn transaction_pool(&self, is_dev: bool) -> sc_cli::Result<sc_service::config::TransactionPoolOptions> {
        self.0.base.transaction_pool(is_dev)
    }

    fn trie_cache_maximum_size(&self) -> sc_cli::Result<Option<usize>> {
        self.0.base.trie_cache_maximum_size()
    }

    fn rpc_methods(&self) -> sc_cli::Result<sc_service::config::RpcMethods> {
        self.0.base.rpc_methods()
    }

    fn rpc_max_connections(&self) -> sc_cli::Result<u32> {
        self.0.base.rpc_max_connections()
    }

    fn rpc_cors(&self, is_dev: bool) -> sc_cli::Result<Option<Vec<String>>> {
        self.0.base.rpc_cors(is_dev)
    }

    fn default_heap_pages(&self) -> sc_cli::Result<Option<u64>> {
        self.0.base.default_heap_pages()
    }

    fn force_authoring(&self) -> sc_cli::Result<bool> {
        self.0.base.force_authoring()
    }

    fn disable_grandpa(&self) -> sc_cli::Result<bool> {
        self.0.base.disable_grandpa()
    }

    fn max_runtime_instances(&self) -> sc_cli::Result<Option<usize>> {
        self.0.base.max_runtime_instances()
    }

    fn announce_block(&self) -> sc_cli::Result<bool> {
        self.0.base.announce_block()
    }

    fn telemetry_endpoints(
        &self,
        chain_spec: &Box<dyn ChainSpec>,
    ) -> sc_cli::Result<Option<sc_telemetry::TelemetryEndpoints>> {
        self.0.base.telemetry_endpoints(chain_spec)
    }

    fn node_name(&self) -> sc_cli::Result<String> {
        self.0.base.node_name()
    }
}
