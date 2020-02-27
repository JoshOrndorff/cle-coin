//! Service and ServiceFactory implementation. Specialized wrapper over substrate service.

use std::sync::Arc;
use std::time::Duration;
use sc_client::LongestChain;
use runtime::{self, GenesisConfig, opaque::Block, RuntimeApi};
use sc_service::{error::{Error as ServiceError}, AbstractService, Configuration, ServiceBuilder};
use sp_inherents::InherentDataProviders;
use sc_network::{config::DummyFinalityProofRequestBuilder, construct_simple_protocol};
use sc_executor::native_executor_instance;
pub use sc_executor::NativeExecutor;
use sp_consensus_aura::sr25519::{AuthorityPair as AuraPair};
use grandpa::{self, FinalityProofProvider as GrandpaFinalityProofProvider};
use sc_basic_authority;
use crate::pow::Sha3Algorithm;

// Our native executor instance.
native_executor_instance!(
	pub Executor,
	runtime::api::dispatch,
	runtime::native_version,
);

construct_simple_protocol! {
	/// Demo protocol attachment for substrate.
	pub struct NodeProtocol where Block = Block { }
}

/// Starts a `ServiceBuilder` for a full service.
///
/// Use this macro if you don't actually need the full service, but just the builder in order to
/// be able to perform chain operations.
macro_rules! new_full_start {
	($config:expr) => {{
		let inherent_data_providers = sp_inherents::InherentDataProviders::new();

		let builder = sc_service::ServiceBuilder::new_full::<
			runtime::opaque::Block, runtime::RuntimeApi, crate::service::Executor
		>($config)?
			.with_select_chain(|_config, backend| {
				Ok(sc_client::LongestChain::new(backend.clone()))
			})?
			.with_transaction_pool(|config, client, _fetcher| {
				let pool_api = sc_transaction_pool::FullChainApi::new(client.clone());
				let pool = sc_transaction_pool::BasicPool::new(config, pool_api);
				let maintainer = sc_transaction_pool::FullBasicPoolMaintainer::new(pool.pool().clone(), client);
				let maintainable_pool = sp_transaction_pool::MaintainableTransactionPool::new(pool, maintainer);
				Ok(maintainable_pool)
			})?
			.with_import_queue(|_config, client, select_chain, _transaction_pool| {
				let import_queue = sc_consensus_pow::import_queue(
					Box::new(client.clone()),
					client.clone(),
					crate::pow::Sha3Algorithm::new(client.clone()),
					0,
					select_chain,
					inherent_data_providers.clone(),
				)?;

				Ok(import_queue)
			})?;

		(builder, inherent_data_providers)
	}}
}

/// Builds a new service for a full client.
pub fn new_full<C: Send + Default + 'static>(config: Configuration<C, GenesisConfig>)
	-> Result<impl AbstractService, ServiceError>
{
	let is_authority = config.roles.is_authority();
	let name = config.name.clone();

	// sentry nodes announce themselves as authorities to the network
	// and should run the same protocols authorities do, but it should
	// never actively participate in any consensus process.
	let participates_in_consensus = is_authority && !config.sentry_mode;

	let (builder, inherent_data_providers) = new_full_start!(config);

	let service = builder.with_network_protocol(|_| Ok(NodeProtocol::new()))?
		.with_finality_proof_provider(|_client, _backend|
			Ok(Arc::new(()) as _)
		)?
		.build()?;

	if participates_in_consensus {
		let proposer = sc_basic_authority::ProposerFactory {
			client: service.client(),
			transaction_pool: service.transaction_pool(),
		};

		// The number of rounds of mining to try in a single call
		let rounds = 500;

		let can_author_with =
			sp_consensus::CanAuthorWithNativeVersion::new(service.client().executor().clone());

		sc_consensus_pow::start_mine(
			Box::new(service.client().clone()),
			service.client(),
			Sha3Algorithm::new(service.client().clone()),
			proposer,
			None,
			rounds,
			service.network(),
			std::time::Duration::new(2, 0),
			service.select_chain().map(|v| v.clone()),
			inherent_data_providers.clone(),
			can_author_with,
		);
	}
	Ok(service)
}

/// Builds a new service for a light client.
pub fn new_light<C: Send + Default + 'static>(config: Configuration<C, GenesisConfig>)
	-> Result<impl AbstractService, ServiceError>
{
	let inherent_data_providers = InherentDataProviders::new();

	ServiceBuilder::new_light::<Block, RuntimeApi, Executor>(config)?
		.with_select_chain(|_config, backend| {
			Ok(LongestChain::new(backend.clone()))
		})?
		.with_transaction_pool(|config, client, fetcher| {
			let fetcher = fetcher
				.ok_or_else(|| "Trying to start light transaction pool without active fetcher")?;
			let pool_api = sc_transaction_pool::LightChainApi::new(client.clone(), fetcher.clone());
			let pool = sc_transaction_pool::BasicPool::new(config, pool_api);
			let maintainer = sc_transaction_pool::LightBasicPoolMaintainer::with_defaults(pool.pool().clone(), client, fetcher);
			let maintainable_pool = sp_transaction_pool::MaintainableTransactionPool::new(pool, maintainer);
			Ok(maintainable_pool)
		})?
		.with_import_queue_and_fprb(|_config, client, _backend, _fetcher, select_chain, _tx_pool| {
			let finality_proof_request_builder =
				Box::new(DummyFinalityProofRequestBuilder::default()) as Box<_>;

			let import_queue = sc_consensus_pow::import_queue(
				Box::new(client.clone()),
				client.clone(),
				Sha3Algorithm::new(client.clone()),
				0,
				select_chain,
				inherent_data_providers.clone(),
			)?;

			Ok((import_queue, finality_proof_request_builder))
		})?
		.with_network_protocol(|_| Ok(NodeProtocol::new()))?
		.with_finality_proof_provider(|_client, _backend|
			Ok(Arc::new(()) as _)
		)?
		.build()
}
