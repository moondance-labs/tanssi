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

use {
    futures::{
        future::BoxFuture,
        stream::{FuturesUnordered, StreamExt},
        FutureExt,
    },
    jsonrpsee::{
        core::{
            client::{Client as JsonRpcClient, ClientT as _, Subscription},
            params::ArrayParams,
            ClientError as JsonRpseeError, JsonValue,
        },
        ws_client::WsClientBuilder,
    },
    sc_rpc_api::chain::ChainApiClient,
    schnellru::{ByLength, LruMap},
    std::sync::Arc,
    tokio::sync::{mpsc, oneshot},
};

const LOG_TARGET: &str = "reconnecting-websocket-client-orchestrator";

type RpcRequestFuture = BoxFuture<'static, Result<(), JsonRpcRequest>>;

/// A Json Rpc/Rpsee request with a oneshot sender to send the request's response.
pub struct JsonRpcRequest {
    pub method: String,
    pub params: ArrayParams,
    pub response_sender: oneshot::Sender<Result<JsonValue, JsonRpseeError>>,
}

pub enum WsClientRequest {
    JsonRpcRequest(JsonRpcRequest),
    RegisterBestHeadListener(mpsc::Sender<dp_core::Header>),
    RegisterImportListener(mpsc::Sender<dp_core::Header>),
    RegisterFinalizationListener(mpsc::Sender<dp_core::Header>),
}

enum ConnectionStatus {
    Connected,
    Disconnected {
        failed_request: Option<JsonRpcRequest>,
    },
}

/// Worker that manage a WebSocket connection and handle disconnects by changing endpoint and
/// retrying pending requests.
///
/// Is first created with [`ReconnectingWsClientWorker::new`], which returns both a
/// [`ReconnectingWsClientWorker`] and an [`mpsc::Sender`] to send the requests.
/// [`ReconnectingWsClientWorker::run`] must the be called and the returned future queued in
/// a tokio executor.
pub struct ReconnectingWsClientWorker {
    urls: Vec<String>,
    active_client: Arc<JsonRpcClient>,
    active_index: usize,

    request_receiver: mpsc::Receiver<WsClientRequest>,

    imported_header_listeners: Vec<mpsc::Sender<dp_core::Header>>,
    finalized_header_listeners: Vec<mpsc::Sender<dp_core::Header>>,
    best_header_listeners: Vec<mpsc::Sender<dp_core::Header>>,
}

struct OrchestratorSubscription {
    import_subscription: Subscription<dp_core::Header>,
    finalized_subscription: Subscription<dp_core::Header>,
    best_subscription: Subscription<dp_core::Header>,
}

/// Connects to a ws server by cycle throught all provided urls from the starting position until
/// each one one was tried. Stops once a connection was succesfully made.
async fn connect_next_available_rpc_server(
    urls: &[String],
    starting_position: usize,
) -> Result<(usize, Arc<JsonRpcClient>), ()> {
    tracing::debug!(target: LOG_TARGET, starting_position, "Connecting to RPC server.");

    for (counter, url) in urls
        .iter()
        .cycle()
        .skip(starting_position)
        .take(urls.len())
        .enumerate()
    {
        let index = (starting_position + counter) % urls.len();

        tracing::info!(
            target: LOG_TARGET,
            index,
            url,
            "Trying to connect to next external orchestrator node.",
        );

        match WsClientBuilder::default().build(&url).await {
            Ok(ws_client) => return Ok((index, Arc::new(ws_client))),
            Err(err) => tracing::debug!(target: LOG_TARGET, url, ?err, "Unable to connect."),
        };
    }
    Err(())
}

impl ReconnectingWsClientWorker {
    /// Create a new worker that will connect to the provided URLs.
    pub async fn new(urls: Vec<String>) -> Result<(Self, mpsc::Sender<WsClientRequest>), ()> {
        if urls.is_empty() {
            return Err(());
        }

        let (active_index, active_client) = connect_next_available_rpc_server(&urls, 0).await?;
        let (request_sender, request_receiver) = mpsc::channel(100);

        Ok((
            Self {
                urls,
                active_client,
                active_index,
                request_receiver,
                best_header_listeners: vec![],
                imported_header_listeners: vec![],
                finalized_header_listeners: vec![],
            },
            request_sender,
        ))
    }

    /// Change RPC server for future requests.
    async fn connect_to_new_rpc_server(&mut self) -> Result<(), ()> {
        let (active_index, active_client) =
            connect_next_available_rpc_server(&self.urls, self.active_index + 1).await?;
        self.active_index = active_index;
        self.active_client = active_client;
        Ok(())
    }

    /// Send the request to the current client. If this connection becomes dead, the returned future
    /// will return the request so it can be sent to another client.
    fn send_request(
        &self,
        JsonRpcRequest {
            method,
            params,
            response_sender,
        }: JsonRpcRequest,
    ) -> RpcRequestFuture {
        let client = self.active_client.clone();
        async move {
            let response = client.request(&method, params.clone()).await;

            // We should only return the original request in case
            // the websocket connection is dead and requires a restart.
            // Other errors should be forwarded to the request caller.
            if let Err(JsonRpseeError::RestartNeeded(_)) = response {
                return Err(JsonRpcRequest {
                    method,
                    params,
                    response_sender,
                });
            }

            if let Err(err) = response_sender.send(response) {
                tracing::debug!(
                    target: LOG_TARGET,
                    ?err,
                    "Recipient no longer interested in request result"
                );
            }

            Ok(())
        }
        .boxed()
    }

    async fn get_subscriptions(&self) -> Result<OrchestratorSubscription, JsonRpseeError> {
        let import_subscription = <JsonRpcClient as ChainApiClient<
            dp_core::BlockNumber,
            dp_core::Hash,
            dp_core::Header,
            dp_core::SignedBlock,
        >>::subscribe_all_heads(&self.active_client)
        .await
        .map_err(|e| {
            tracing::error!(
                target: LOG_TARGET,
                ?e,
                "Unable to open `chain_subscribeAllHeads` subscription."
            );
            e
        })?;

        let best_subscription = <JsonRpcClient as ChainApiClient<
            dp_core::BlockNumber,
            dp_core::Hash,
            dp_core::Header,
            dp_core::SignedBlock,
        >>::subscribe_new_heads(&self.active_client)
        .await
        .map_err(|e| {
            tracing::error!(
                target: LOG_TARGET,
                ?e,
                "Unable to open `chain_subscribeNewHeads` subscription."
            );
            e
        })?;

        let finalized_subscription = <JsonRpcClient as ChainApiClient<
            dp_core::BlockNumber,
            dp_core::Hash,
            dp_core::Header,
            dp_core::SignedBlock,
        >>::subscribe_finalized_heads(&self.active_client)
        .await
        .map_err(|e| {
            tracing::error!(
                target: LOG_TARGET,
                ?e,
                "Unable to open `chain_subscribeFinalizedHeads` subscription."
            );
            e
        })?;

        Ok(OrchestratorSubscription {
            import_subscription,
            best_subscription,
            finalized_subscription,
        })
    }

    /// Handle a reconnection by fnding a new RPC server and sending all pending requests.
    async fn handle_reconnect(
        &mut self,
        pending_requests: &mut FuturesUnordered<RpcRequestFuture>,
        first_failed_request: Option<JsonRpcRequest>,
    ) -> Result<(), String> {
        let mut requests_to_retry = Vec::new();
        if let Some(req) = first_failed_request {
            requests_to_retry.push(req)
        }

        // All pending requests will return an error since the websocket connection is dead.
        // Draining the pending requests should be fast.
        while !pending_requests.is_empty() {
            if let Some(Err(req)) = pending_requests.next().await {
                requests_to_retry.push(req);
            }
        }

        // Connect to new RPC server if possible.
        if self.connect_to_new_rpc_server().await.is_err() {
            return Err("Unable to find valid external RPC server, shutting down.".to_string());
        }

        // Retry requests.
        for req in requests_to_retry.into_iter() {
            pending_requests.push(self.send_request(req));
        }

        // Get subscriptions from new endpoint.
        self.get_subscriptions().await.map_err(|e| {
			format!("Not able to create streams from newly connected RPC server, shutting down. err: {:?}", e)
		})?;

        Ok(())
    }

    pub async fn run(mut self) {
        let mut pending_requests = FuturesUnordered::new();
        let mut connection_status = ConnectionStatus::Connected;

        let Ok(mut subscriptions) = self.get_subscriptions().await else {
            tracing::error!(target: LOG_TARGET, "Unable to fetch subscriptions on initial connection.");
            return;
        };

        let mut imported_blocks_cache = LruMap::new(ByLength::new(40));
        let mut last_seen_finalized_num: dp_core::BlockNumber = 0;

        loop {
            // Handle reconnection.
            if let ConnectionStatus::Disconnected { failed_request } = connection_status {
                if let Err(message) = self
                    .handle_reconnect(&mut pending_requests, failed_request)
                    .await
                {
                    tracing::error!(
                        target: LOG_TARGET,
                        message,
                        "Unable to reconnect, stopping worker."
                    );
                    return;
                }

                connection_status = ConnectionStatus::Connected;
            }

            tokio::select! {
                // New request received.
                req = self.request_receiver.recv() => match req {
                    Some(WsClientRequest::JsonRpcRequest(req)) => {
                        pending_requests.push(self.send_request(req));
                    },
                    Some(WsClientRequest::RegisterBestHeadListener(tx)) => {
                        self.best_header_listeners.push(tx);
                    },
                    Some(WsClientRequest::RegisterImportListener(tx)) => {
                        self.imported_header_listeners.push(tx);
                    },
                    Some(WsClientRequest::RegisterFinalizationListener(tx)) => {
                        self.finalized_header_listeners.push(tx);
                    },
                    None => {
                        tracing::error!(target: LOG_TARGET, "RPC client receiver closed. Stopping RPC Worker.");
                        return;
                    }
                },
                // We poll pending request futures. If one completes with an `Err`, it means the
                // ws client was disconnected and we need to reconnect to a new ws client.
                pending = pending_requests.next(), if !pending_requests.is_empty() => {
                    if let Some(Err(req)) = pending {
                        connection_status = ConnectionStatus::Disconnected { failed_request: Some(req) };
                    }
                },
                import_event = subscriptions.import_subscription.next() => {
                    match import_event {
                        Some(Ok(header)) => {
                            let hash = header.hash();
                            if imported_blocks_cache.peek(&hash).is_some() {
                                tracing::debug!(
                                    target: LOG_TARGET,
                                    number = header.number,
                                    ?hash,
                                    "Duplicate imported block header. This might happen after switching to a new RPC node. Skipping distribution."
                                );
                                continue;
                            }
                            imported_blocks_cache.insert(hash, ());
                            distribute(header, &mut self.imported_header_listeners);
                        },
                        None => {
                            tracing::error!(target: LOG_TARGET, "Subscription closed.");
                            connection_status = ConnectionStatus::Disconnected { failed_request: None};
                        },
                        Some(Err(error)) => {
                            tracing::error!(target: LOG_TARGET, ?error, "Error in RPC subscription.");
                            connection_status = ConnectionStatus::Disconnected { failed_request: None};
                        },
                    }
                },
                best_header_event = subscriptions.best_subscription.next() => {
                    match best_header_event {
                        Some(Ok(header)) => distribute(header, &mut self.best_header_listeners),
                        None => {
                            tracing::error!(target: LOG_TARGET, "Subscription closed.");
                            connection_status = ConnectionStatus::Disconnected { failed_request: None};
                        },
                        Some(Err(error)) => {
                            tracing::error!(target: LOG_TARGET, ?error, "Error in RPC subscription.");
                            connection_status = ConnectionStatus::Disconnected { failed_request: None};
                        },
                    }
                }
                finalized_event = subscriptions.finalized_subscription.next() => {
                    match finalized_event {
                        Some(Ok(header)) if header.number > last_seen_finalized_num => {
                            last_seen_finalized_num = header.number;
                            distribute(header, &mut self.finalized_header_listeners);
                        },
                        Some(Ok(header)) => {
                            tracing::debug!(
                                target: LOG_TARGET,
                                number = header.number,
                                last_seen_finalized_num,
                                "Duplicate finalized block header. This might happen after switching to a new RPC node. Skipping distribution."
                            );
                        },
                        None => {
                            tracing::error!(target: LOG_TARGET, "Subscription closed.");
                            connection_status = ConnectionStatus::Disconnected { failed_request: None};
                        },
                        Some(Err(error)) => {
                            tracing::error!(target: LOG_TARGET, ?error, "Error in RPC subscription.");
                            connection_status = ConnectionStatus::Disconnected { failed_request: None};
                        },
                    }
                }
            }
        }
    }
}

/// Send `value` through all channels contained in `senders`.
/// If no one is listening to the sender, it is removed from the vector.
pub fn distribute<T: Clone + Send>(value: T, senders: &mut Vec<mpsc::Sender<T>>) {
    senders.retain_mut(|e| {
        match e.try_send(value.clone()) {
            // Receiver has been dropped, remove Sender from list.
            Err(mpsc::error::TrySendError::Closed(_)) => false,
            // Channel is full. This should not happen.
            // TODO: Improve error handling here
            // https://github.com/paritytech/cumulus/issues/1482
            Err(error) => {
                tracing::error!(target: LOG_TARGET, ?error, "Event distribution channel has reached its limit. This can lead to missed notifications.");
                true
            },
            _ => true,
        }
    });
}
