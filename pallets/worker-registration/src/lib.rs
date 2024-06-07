//! A shell pallet built with [`frame`].

#![cfg_attr(not(feature = "std"), no_std)]
// #![allow(ambiguous_glob_reexports)]
// #![allow(unused_imports)]

use parity_scale_codec::{
	Decode, Encode, alloc::string::ToString,
};
// use frame_support::{ pallet_prelude::*, ensure};
use frame_system::{
	pallet_prelude::*, WeightInfo
};
use scale_info::{prelude::vec::Vec, prelude::{string::String, format}, TypeInfo};
// use sp_runtime::{
// 	RuntimeDebug,
// };
use sp_core::crypto::KeyTypeId;
use frame_system::{
	self as system,
	offchain::{
		AppCrypto, CreateSignedTransaction, SendSignedTransaction, SendUnsignedTransaction,
		SignedPayload, Signer, SigningTypes, SubmitTransaction,
	},
	pallet_prelude::BlockNumberFor,
};
use sp_runtime::{
	offchain::{
		http,
		storage::{MutateStorageError, StorageRetrievalError, StorageValueRef},
		Duration,
	},
	traits::Zero,
	transaction_validity::{InvalidTransaction, TransactionValidity, ValidTransaction},
	RuntimeDebug, BoundedVec,
};
use sp_io::offchain_index;
use serde::{Deserialize, Deserializer};
use sp_std::str;

pub type ClusterId = u64;
pub type TaskId = u64;

#[derive(PartialEq, Eq, Clone, Decode, Encode, TypeInfo, Debug)]
pub enum WorkerStatusType {
	Active,
	Pending,
	Completed,
	Inactive,
}

#[derive(PartialEq, Eq, Clone, Decode, Encode, TypeInfo, Debug)]
pub enum TaskStatusType {
	Pending,
	Completed,
	Expired,
}

#[derive(PartialEq, Eq, Clone, Decode, Encode, TypeInfo, Debug)]
pub enum TaskType {
	Docker,
}

#[derive(Default, PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode, TypeInfo)]
pub struct Ip {
	pub ipv4: Option<Vec<u32>>,
	pub ipv6: Option<Vec<u32>>,
}

#[derive(PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode, TypeInfo)]
pub struct Worker<AccountId, BlockNumber> {
	pub id: ClusterId,
	pub account: AccountId,
	pub start_block: BlockNumber,
	pub name: Vec<u8>,
	pub ip: Ip,
	pub port: u32,
	pub status: bool,
}


#[derive(PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode, TypeInfo)]
pub struct TaskInfo<AccountId, BlockNumber> {
	pub task_owner: AccountId,
	pub create_block: BlockNumber,
	pub metadata: Vec<u8>,
	pub assigned_worker: ClusterId,
	pub time_elapsed: Option<BlockNumber>,
	pub average_cpu_percentage_use: Option<u8>,
	pub task_type: TaskType,
	// pub completed_hash: Option<Hash>
}

pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"ping");

pub mod crypto {
	use super::KEY_TYPE;
	use sp_core::sr25519::Signature as Sr25519Signature;
	use sp_runtime::{
		traits::Verify,
		MultiSignature, MultiSigner,
	};
	use scale_info::prelude::string::String;
	use sp_application_crypto::{app_crypto, sr25519};

	app_crypto!(sr25519, KEY_TYPE);
	pub struct ClusterStatusAuthId;

	// used for offchain worker transaction signing
	impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for ClusterStatusAuthId {
		type RuntimeAppPublic = Public;
		type GenericSignature = sp_core::sr25519::Signature;
		type GenericPublic = sp_core::sr25519::Public;
	}

	// implemented for mock runtime in test
	impl frame_system::offchain::AppCrypto<<Sr25519Signature as Verify>::Signer, Sr25519Signature>
		for ClusterStatusAuthId
	{
		type RuntimeAppPublic = Public;
		type GenericSignature = sp_core::sr25519::Signature;
		type GenericPublic = sp_core::sr25519::Public;
	}
}
const ONCHAIN_TX_KEY: &[u8] = b"cluster::storage::tx";

#[derive(Debug, Deserialize, Encode, Decode, Default)]
struct IndexingData(Vec<u8>, u64);

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

// Http response from verifying a cluster
#[derive(Debug, Deserialize)]
struct VerifyClusterJSONResponse {
    deployment_status: bool,
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	
	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: CreateSignedTransaction<Call<Self>> + frame_system::Config {
		/// Pallet event
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;
		/// Authority ID used for offchain worker
		type AuthorityId: AppCrypto<Self::Public, Self::Signature>;
	}

	#[pallet::type_value]
    pub fn DefaultForm1() -> ClusterId {
        0
    }

	/// Id of the next cluster of worker to be registered
    #[pallet::storage]
    #[pallet::getter(fn get_next_cluster_id)]
    pub type NextClusterId<T: Config> = StorageValue<_, ClusterId, ValueQuery, DefaultForm1>;


	/// user's Worker information
	#[pallet::storage]
	#[pallet::getter(fn get_worker_accounts)]
	pub type WorkerAccounts<T: Config> = 
		StorageMap<_, Identity, T::AccountId, ClusterId, OptionQuery>;

	#[pallet::storage]
    #[pallet::getter(fn task_status)]
    pub type TaskStatus<T: Config> = StorageMap<_, Twox64Concat, TaskId, TaskStatusType, OptionQuery>;
	
	#[pallet::storage]
    #[pallet::getter(fn task_allocations)]
    pub type TaskAllocations<T: Config> = StorageMap<_, Twox64Concat, TaskId, T::AccountId, OptionQuery>;

	#[pallet::storage]
    #[pallet::getter(fn task_owners)]
    pub type TaskOwners<T: Config> = StorageMap<_, Twox64Concat, TaskId, T::AccountId, OptionQuery>;

	#[pallet::storage]
    #[pallet::getter(fn next_task_id)]
    pub type NextTaskId<T: Config> = StorageValue<_, TaskId, ValueQuery>;

	/// Task Information
	#[pallet::storage]
	#[pallet::getter(fn get_tasks)]
	pub type Tasks<T: Config> = 
		StorageMap<_, Identity, TaskId, TaskInfo<T::AccountId, BlockNumberFor<T>>, OptionQuery>;

	/// Worker Cluster information
	#[pallet::storage]
	#[pallet::getter(fn get_worker_clusters)]
	pub type WorkerClusters<T: Config> = 
		StorageMap<_, Identity, ClusterId, Worker<T::AccountId, BlockNumberFor<T>>, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		WorkerRegistered{ creator: T::AccountId },
		WorkerUnRegistered{ creator: T::AccountId, cluster_id: ClusterId },
		TaskScheduled {
            worker: T::AccountId,
			owner: T::AccountId,
            task_id: TaskId,
            task: String,
			assigned_ip: String,
        },
		ConnectionEstablished{ cluster_id: ClusterId },
		SubmittedCompletedTask{ task_id: TaskId },
		VerifiedCompletedTask{ task_id: TaskId },
	}

	/// Pallet Errors
	#[pallet::error]
	pub enum Error<T> {
		WorkerRegisterMissingIp,
		WorkerRegisterMissingPort,
		ClusterExists,
		ClusterDoesNotExists,
		NoWorkersAvailable,
		InvalidOwnerOfCluster,
		WorkerClusterNotRegistered,
		UnassignedTaskId,
		InvalidTaskOwner,
		RequirePendingTask,
	}

	// The pallet's hooks for offchain worker
	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn offchain_worker(block_number: BlockNumberFor<T>) {
			log::info!("From: Worker Registration Pallet offchain workers.");

			let signer = Signer::<T, T::AuthorityId>::any_account();
			if !signer.can_sign() {
				log::error!("Offchain::worker-registration: No local accounts available");
				return
			}

			// Import `frame_system` and retrieve a block hash of the parent block.
			let parent_hash = <system::Pallet<T>>::block_hash(block_number - 1u32.into());
			log::debug!("ffchain::worker-registration: Current block: {:?} (parent hash: {:?})", block_number, parent_hash);

			// Reading back the off-chain indexing value. It is exactly the same as reading from
			// ocw local storage for any pings to cluster.
			let ping_key = Self::derived_key(block_number, "ping");
			let oci_mem_ping = StorageValueRef::persistent(&ping_key);
			log::info!("Offchain::worker-registration: ping_key info: {:?}", ping_key.clone());
			log::info!("Offchain::worker-registration: oci_mem_ping info: {:?}", oci_mem_ping.get::<IndexingData>()); 

			if let Ok(Some(data)) = oci_mem_ping.get::<IndexingData>() {
				log::info!("Offchain::worker-registration: off-chain indexing cluster status data: {:?}, {:?}",
					str::from_utf8(&data.0).unwrap_or("error"), data.1);
					if let Some(cluster) = Self::get_worker_clusters(data.1){
						log::info!("Offchain::worker-registration: cluster info: {:?}", cluster);

						let response: VerifyClusterJSONResponse = Self::fetch_cluster_status(cluster.ip, cluster.port).unwrap_or_else(|e| {
							log::error!("Offchain::worker-registration: fetch_response error: {:?}", e);
							VerifyClusterJSONResponse {
								deployment_status: false,
							}
						});

						// assign cluster_id to cluster
						// let response: String = Self::asssign_cluster_id(cluster.ip, cluster.port).unwrap_or_else(|e| {
						// 	log::error!("fetch_response error: {:?}", e);
						// 	"Failed".into()
						// });

						log::info!("Offchain::worker-registration: Response: {:?}", response);
						// Use response to submit info to blockchain about cluster

						// Using `send_signed_transaction` associated type we create and submit a transaction
						// representing the call, we've just created.
						// Submit signed will return a vector of results for all accounts that were found in the
						// local keystore with expected `KEY_TYPE`.
						let results = signer.send_signed_transaction(|_account| {
							// Received price is wrapped into a call to `submit_price` public function of this
							// pallet. This means that the transaction, when executed, will simply call that
							// function passing `price` as an argument.
							Call::verify_connection { worker_index: data.1, deployment_status: response.deployment_status }
						});

						for (acc, res) in &results {
							match res {
								Ok(()) => log::info!("Offchain::worker-registration: [{:?}] Submitted cluster info for cluder id {}", acc.id, data.1),
								Err(e) => log::error!("Offchain::worker-registration: [{:?}] Failed to submit transaction: {:?}", acc.id, e),
							}
						}
					} else {
						log::info!("Offchain::worker-registration: no cluster retrieved.");
				};
			} else {
				log::info!("Offchain::worker-registration: no off-chain indexing data retrieved for cluster pings.");
			}

			// // Reading back the off-chain indexing value.
			// // ocw local storage for task completed verifications.
			// let task_key = Self::derived_key(block_number, "task");
			// let oci_mem_task = StorageValueRef::persistent(&task_key);

			// if let Ok(Some(data)) = oci_mem_task.get::<IndexingData>() {
			// 	log::info!("off-chain indexing task data: {:?}, {:?}",
			// 		str::from_utf8(&data.0).unwrap_or("error"), data.1);
			// 		if let Some(task) = Self::get_tasks(data.1){
			// 			log::info!("task info: {:?}", task);
			// 			if let Some(cluster) = Self::get_worker_clusters(task.assigned_worker) {

			// 				let response: String = Self::confirm_task_completion(cluster.ip, cluster.port).unwrap_or_else(|e| {
			// 					log::error!("fetch_response error: {:?}", e);
			// 					"Failed".into()
			// 				});
			// 				log::info!("Response: {}", response.clone());
			// 				// Use response to submit info to blockchain about cluster
	
			// 				// Using `send_signed_transaction` associated type we create and submit a transaction
			// 				// representing the call, we've just created.
			// 				// Submit signed will return a vector of results for all accounts that were found in the
			// 				// local keystore with expected `KEY_TYPE`.
			// 				let results = signer.send_signed_transaction(|_account| {
			// 					// Received price is wrapped into a call to `submit_price` public function of this
			// 					// pallet. This means that the transaction, when executed, will simply call that
			// 					// function passing `price` as an argument.
			// 					Call::verify_completed_task { task_id: data.1, response: response.clone() } //TODO
			// 				});
	
			// 				for (acc, res) in &results {
			// 					match res {
			// 						Ok(()) => log::info!("[{:?}] Submitted cluster info for cluder id {}", acc.id, data.1),
			// 						Err(e) => log::error!("[{:?}] Failed to submit transaction: {:?}", acc.id, e),
			// 					}
			// 				}
			// 			};
			// 		} else {
			// 			log::info!("no cluster retrieved.");
			// 	};
			// } else {
			// 	log::info!("no off-chain indexing data retrieved.");
			// }

		}
	}

	#[pallet::call]
	impl<T:Config> Pallet<T> {
		/// Worker cluster registration
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn worker_register(
			origin: OriginFor<T>,
			name: Vec<u8>,
			ip: Ip,
			port: u32,
		) -> DispatchResult {
			let creator = ensure_signed(origin)?;

			//check ip
			ensure!(ip.ipv4.is_some() || ip.ipv6.is_some(), Error::<T>::WorkerRegisterMissingIp);
			ensure!(port > 0, Error::<T>::WorkerRegisterMissingPort);
			
			//check cluster
			ensure!(WorkerAccounts::<T>::contains_key(creator.clone()) == false, 
			Error::<T>::ClusterExists);

			let cid = NextClusterId::<T>::get();

			let cluster = Worker {
				id: cid.clone(),
				account: creator.clone(),
				start_block: <frame_system::Pallet<T>>::block_number(),
				name: name.clone(),
				ip: ip.clone(),
				port: port.clone(),
				status: false,
			};

			// update storage
			WorkerAccounts::<T>::insert(creator.clone(), cid.clone());
			WorkerClusters::<T>::insert(cid.clone(), cluster);
			NextClusterId::<T>::mutate(|id| *id += 1);

			// update data from offchain worker on cluster healthcheck and metadata

			// Off-chain indexing allowing on-chain extrinsics to write to off-chain storage predictably
			// so it can be read in off-chain worker context. As off-chain indexing is called in on-chain
			// context, if it is agreed upon by the blockchain consensus mechanism, then it is expected
			// to run predicably by all nodes in the network.
			//
			// From an on-chain perspective, this is write-only and cannot be read back.
			//
			// The value is written in byte form, so we need to encode/decode it when writting/reading
			// a number to/from this memory space.
			
			let key = Self::derived_key(frame_system::Pallet::<T>::block_number(), "ping");
			log::info!("Offchain worker key: {:?}", key.clone());
			
			let data: IndexingData = IndexingData(b"registered_cluster_ping".to_vec(), cid);
			offchain_index::set(&key, &data.encode());

			// Emit an event.
			Self::deposit_event(Event::WorkerRegistered { creator });
	
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn task_scheduler(
			origin: OriginFor<T>,
			task_data: String,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
		
			ensure!(WorkerAccounts::<T>::iter().next().is_some(), Error::<T>::NoWorkersAvailable);

			let task_id = NextTaskId::<T>::get();
			NextTaskId::<T>::put(task_id.wrapping_add(1));
		
			// Select one worker randomly.
			let workers = WorkerAccounts::<T>::iter().collect::<Vec<_>>();
			let random_index = (sp_io::hashing::blake2_256(&task_data.as_bytes())[0] as usize) % workers.len();
			let selected_worker = workers[random_index].0.clone();
		
			let cluster_id = Self::get_worker_accounts(selected_worker.clone()).ok_or(Error::<T>::WorkerClusterNotRegistered)?;
			let task_info = TaskInfo {
				task_owner: who.clone(),
				create_block: <frame_system::Pallet<T>>::block_number(),
				metadata: task_data.clone().as_bytes().to_vec(),
				assigned_worker: cluster_id,
				time_elapsed: None,
				average_cpu_percentage_use: None,
				task_type: TaskType::Docker,
				// completed_hash: None,
			};


			// Assign task to worker and set task owner.
			TaskAllocations::<T>::insert(task_id, selected_worker.clone());
			TaskOwners::<T>::insert(task_id, who.clone());
			Tasks::<T>::insert(task_id, task_info);
			TaskStatus::<T>::insert(task_id, TaskStatusType::Pending);
		
			let cluster_id = WorkerAccounts::<T>::get(selected_worker.clone()).ok_or(Error::<T>::WorkerClusterNotRegistered)?;
			let cluster = WorkerClusters::<T>::get(cluster_id).ok_or(Error::<T>::WorkerClusterNotRegistered)?;

			let string_ip = match cluster.ip {
				Ip { ipv4: Some(i), ..}  => i.iter().map(|n| n.to_string()).collect::<Vec<_>>().join("."),
				Ip { ipv6: Some(i), .. }  => i.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(":"),
				_=> {
					format!("localhost")
				}
			};
			// Emit an event.
			Self::deposit_event(Event::TaskScheduled {
				worker: selected_worker,
				owner: who,
				task_id, 
				task: task_data,
				assigned_ip: string_ip,
			});
			Ok(())
		}
		
		#[pallet::call_index(2)]
		#[pallet::weight({0})]
		/// Submit updates worker cluster status and information from successful connection.
		pub fn verify_connection(origin: OriginFor<T>, worker_index: ClusterId, deployment_status: bool) -> DispatchResult {
			// Retrieve the signer and check it is valid.
			let who = ensure_signed(origin)?;

			WorkerClusters::<T>::try_mutate(worker_index, |worker| -> DispatchResult {
				let cluster_info = worker.as_mut().ok_or(Error::<T>::WorkerClusterNotRegistered)?;

				cluster_info.status = deployment_status;

				// update worker's info
				WorkerClusters::<T>::insert(worker_index, cluster_info);

				// Emit an event.
				Self::deposit_event(Event::ConnectionEstablished { cluster_id: worker_index });
				Ok(())
			})?;
			// Return a successful DispatchResult
			Ok(())
		}


		//TODO: update implementation
		#[pallet::call_index(3)]
		#[pallet::weight({0})]
		/// Submit completed task
		pub fn submit_completed_task(
			origin: OriginFor<T>,
			task_id: TaskId,
		) -> DispatchResult
		{
			let who = ensure_signed(origin)?;
			let task_owner = TaskOwners::<T>::get(task_id).ok_or(Error::<T>::UnassignedTaskId)?;
			ensure!(task_owner == who, Error::<T>::InvalidTaskOwner);
			ensure!(TaskStatus::<T>::get(task_id) == Some(TaskStatusType::Pending), Error::<T>::RequirePendingTask);

			let key = Self::derived_key(frame_system::Pallet::<T>::block_number(), "task");
			let data: IndexingData = IndexingData(b"scheduled_task".to_vec(), task_id.into());
			offchain_index::set(&key, &data.encode());
			// Emit an event.
			Self::deposit_event(Event::SubmittedCompletedTask { task_id });
			Ok(())
		} 

		//TODO: update implementation
		#[pallet::call_index(4)]
		#[pallet::weight({0})]
		/// Submit completed task
		pub fn verify_completed_task(
			origin: OriginFor<T>,
			task_id: TaskId,
			response: String,
		) -> DispatchResult
		{
			let who = ensure_signed(origin)?;
			let task_owner = TaskOwners::<T>::get(task_id).ok_or(Error::<T>::UnassignedTaskId)?;
			// ensure!(task_owner == who, Error::<T>::InvalidTaskOwner);

			ensure!(TaskStatus::<T>::get(task_id) == Some(TaskStatusType::Pending), Error::<T>::RequirePendingTask);
			TaskStatus::<T>::insert(task_id, TaskStatusType::Completed);

			// Emit an event.
			Self::deposit_event(Event::VerifiedCompletedTask { task_id });
			Ok(())
		} 

		#[pallet::call_index(5)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn unregister_worker(
			origin: OriginFor<T>,
			cluster_id: ClusterId,
		) -> DispatchResult {
			let creator = ensure_signed(origin)?;
			
			//check cluster
			ensure!(WorkerAccounts::<T>::get(creator.clone()) == Some(cluster_id), 
			Error::<T>::InvalidOwnerOfCluster);
			ensure!(WorkerClusters::<T>::get(cluster_id) != None, 
			Error::<T>::ClusterDoesNotExists);
	
			// update storage
			WorkerClusters::<T>::remove(cluster_id);
			WorkerAccounts::<T>::remove(creator.clone());
	
			// Emit an event.
			Self::deposit_event(Event::WorkerUnRegistered { creator, cluster_id });
	
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		#[deny(clippy::clone_double_ref)]
		fn derived_key(block_number: BlockNumberFor<T>, extend: &str) -> Vec<u8> {
			block_number.using_encoded(|encoded_bn| {
				ONCHAIN_TX_KEY
					.iter()
					.chain(b"/".iter())
					.chain(encoded_bn)
					// .chain(extend.as_bytes().to_vec().iter())
					.copied()
					.collect::<Vec<u8>>()
			})
		}
		/// Fetches the current cluster status response from remote URL and returns it as a string.
		fn fetch_cluster_status(ip: Ip, port: u32) -> Result<VerifyClusterJSONResponse, http::Error> {
			let body = Self::http_call(ip, port, "cluster-status")?;
			match serde_json::from_slice::<VerifyClusterJSONResponse>(&body) {
				Ok(response) => {
					log::info!("Deserialized object: {:?}", response);
					Ok(response)
				}
				Err(e) => {
					log::info!("Failed to deserialize JSON: {}", e);
					Err(http::Error::Unknown)
				}
				_ => {Err(http::Error::Unknown)}
			}
		}
		fn http_call(ip: Ip, port: u32, route: &str) -> Result<Vec<u8>, http::Error> {
			// We want to keep the offchain worker execution time reasonable, so we set a hard-coded
			// deadline to 3s to complete the external call.
			// You can also wait idefinitely for the response, however you may still get a timeout
			// coming from the host machine.
			let deadline = sp_io::offchain::timestamp().add(Duration::from_millis(3_000));
			// Initiate an external HTTP GET request.
			// This is using high-level wrappers from `sp_runtime`, for the low-level calls that
			// you can find in `sp_io`. The API is trying to be similar to `reqwest`, but
			// since we are running in a custom WASM execution environment we can't simply
			// import the library here.
			let string_ip = match ip {
				Ip { ipv4: Some(i), ..}  => i.iter().map(|n| n.to_string()).collect::<Vec<_>>().join("."),
				Ip { ipv6: Some(i), .. }  => i.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(":"),
				_=> {
					format!("localhost")
				}
			};
			
			let url = format!("http://{}:{}/{}", string_ip, port, route);
			let request = http::Request::get(&url);
			// We set the deadline for sending of the request, note that awaiting response can
			// have a separate deadline. Next we send the request, before that it's also possible
			// to alter request headers or stream body content in case of non-GET requests.
			let pending = request.deadline(deadline).send().map_err(|_| http::Error::IoError)?;

			// The request is already being processed by the host, we are free to do anything
			// else in the worker (we can send multiple concurrent requests too).
			// At some point however we probably want to check the response though,
			// so we can block current thread and wait for it to finish.
			// Note that since the request is being driven by the host, we don't have to wait
			// for the request to have it complete, we will just not read the response.
			let response = pending.try_wait(deadline).map_err(|_| http::Error::DeadlineReached)??;
			// Let's check the status code before we proceed to reading the response.
			if response.code != 200 {
				log::warn!("Unexpected status code: {}", response.code);
				return Err(http::Error::Unknown)
			}

			// Next we want to fully read the response body and collect it to a vector of bytes.
			// Note that the return object allows you to read the body in chunks as well
			// with a way to control the deadline.
			let body = response.body().collect::<Vec<u8>>();

			// Create a str slice from the body.
			let body_str = sp_std::str::from_utf8(&body).map_err(|_| {
				log::warn!("No UTF8 body");
				http::Error::Unknown
			})?;

			log::info!("fetch_response: {}", body_str);

			let response: String = body_str.to_string();

			match response.len() {
				0 => Err(http::Error::Unknown),
				_ => Ok(body),
			}
		}
		// fn confirm_task_completion(ip: Ip, port: u32) -> Result<String, http::Error> {
		// 	// fetch existing tasks
		// 	Self::http_call(ip, port, "task")
		// 	// if task is pending, call remote http url to fetch job status
		// 	// if complete or error, update job status
		// 	// default: if exceed an interval set job status to failed
		// }
	}
	
}
