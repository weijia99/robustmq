// Copyright 2023 RobustMQ Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::{
    handler::{cache::CacheManager, command::Command},
    security::AuthDriver,
    server::{
        connection::NetworkConnectionType,
        connection_manager::ConnectionManager,
        packet::{RequestPackage, ResponsePackage},
        tcp::{
            handler::handler_process, response::response_process, tcp_server::acceptor_process,
            tls_server::acceptor_tls_process,
        },
    },
    subscribe::subscribe_manager::SubscribeManager,
};
use clients::poll::ClientPool;
use common_base::config::broker_mqtt::broker_mqtt_conf;
use log::info;
use std::sync::Arc;
use storage_adapter::storage::StorageAdapter;
use tokio::{
    net::TcpListener,
    sync::{broadcast, mpsc},
};

pub async fn start_tcp_server<S>(
    sucscribe_manager: Arc<SubscribeManager>,
    cache_manager: Arc<CacheManager>,
    connection_manager: Arc<ConnectionManager>,
    message_storage_adapter: Arc<S>,
    client_poll: Arc<ClientPool>,
    stop_sx: broadcast::Sender<bool>,
    auth_driver: Arc<AuthDriver>,
) where
    S: StorageAdapter + Sync + Send + 'static + Clone,
{
    let conf = broker_mqtt_conf();
    let command = Command::new(
        cache_manager.clone(),
        message_storage_adapter.clone(),
        sucscribe_manager.clone(),
        client_poll.clone(),
        connection_manager.clone(),
        auth_driver.clone(),
    );

    let mut server = TcpServer::<S>::new(
        command.clone(),
        conf.tcp_thread.accept_thread_num,
        conf.tcp_thread.handler_thread_num,
        conf.tcp_thread.response_thread_num,
        stop_sx.clone(),
        connection_manager.clone(),
        sucscribe_manager.clone(),
        cache_manager.clone(),
        client_poll.clone(),
    );
    server.start(conf.network.tcp_port).await;

    let mut server = TcpServer::<S>::new(
        command,
        conf.tcp_thread.accept_thread_num,
        conf.tcp_thread.handler_thread_num,
        conf.tcp_thread.response_thread_num,
        stop_sx.clone(),
        connection_manager,
        sucscribe_manager.clone(),
        cache_manager,
        client_poll,
    );
    server.start_tls(conf.network.tcps_port).await;
}

// U: codec: encoder + decoder
// S: message storage adapter
struct TcpServer<S> {
    command: Command<S>,
    connection_manager: Arc<ConnectionManager>,
    cache_manager: Arc<CacheManager>,
    subscribe_manager: Arc<SubscribeManager>,
    client_poll: Arc<ClientPool>,
    accept_thread_num: usize,
    handler_process_num: usize,
    response_process_num: usize,
    stop_sx: broadcast::Sender<bool>,
    network_connection_type: NetworkConnectionType,
}

impl<S> TcpServer<S>
where
    S: StorageAdapter + Clone + Send + Sync + 'static,
{
    pub fn new(
        command: Command<S>,
        accept_thread_num: usize,
        handler_process_num: usize,
        response_process_num: usize,
        stop_sx: broadcast::Sender<bool>,
        connection_manager: Arc<ConnectionManager>,
        subscribe_manager: Arc<SubscribeManager>,
        cache_manager: Arc<CacheManager>,
        client_poll: Arc<ClientPool>,
    ) -> Self {
        Self {
            command,
            subscribe_manager,
            cache_manager,
            client_poll,
            connection_manager,
            accept_thread_num,
            handler_process_num,
            response_process_num,
            stop_sx,
            network_connection_type: NetworkConnectionType::TCP,
        }
    }

    pub async fn start(&mut self, port: u32) {
        let listener = match TcpListener::bind(format!("0.0.0.0:{}", port)).await {
            Ok(tl) => tl,
            Err(e) => {
                panic!("{}", e.to_string());
            }
        };
        let (request_queue_sx, request_queue_rx) = mpsc::channel::<RequestPackage>(1000);
        let (response_queue_sx, response_queue_rx) = mpsc::channel::<ResponsePackage>(1000);

        let arc_listener = Arc::new(listener);

        acceptor_process(
            self.accept_thread_num.clone(),
            self.connection_manager.clone(),
            self.stop_sx.clone(),
            arc_listener.clone(),
            request_queue_sx,
            self.cache_manager.clone(),
            self.network_connection_type.clone(),
        )
        .await;

        handler_process(
            self.handler_process_num.clone(),
            request_queue_rx,
            self.connection_manager.clone(),
            response_queue_sx,
            self.stop_sx.clone(),
            self.command.clone(),
        )
        .await;

        response_process(
            self.response_process_num,
            self.connection_manager.clone(),
            self.cache_manager.clone(),
            self.subscribe_manager.clone(),
            response_queue_rx,
            self.client_poll.clone(),
            self.stop_sx.clone(),
        )
        .await;

        self.network_connection_type = NetworkConnectionType::TCP;
        info!("MQTT TCP Server started successfully, listening port: {port}");
    }

    pub async fn start_tls(&mut self, port: u32) {
        let listener = match TcpListener::bind(format!("0.0.0.0:{}", port)).await {
            Ok(tl) => tl,
            Err(e) => {
                panic!("{}", e.to_string());
            }
        };
        let (request_queue_sx, request_queue_rx) = mpsc::channel::<RequestPackage>(1000);
        let (response_queue_sx, response_queue_rx) = mpsc::channel::<ResponsePackage>(1000);

        let arc_listener = Arc::new(listener);

        acceptor_tls_process(
            self.accept_thread_num,
            arc_listener.clone(),
            self.stop_sx.clone(),
            self.network_connection_type.clone(),
            self.connection_manager.clone(),
            request_queue_sx,
        )
        .await;

        handler_process(
            self.handler_process_num.clone(),
            request_queue_rx,
            self.connection_manager.clone(),
            response_queue_sx,
            self.stop_sx.clone(),
            self.command.clone(),
        )
        .await;

        response_process(
            self.response_process_num,
            self.connection_manager.clone(),
            self.cache_manager.clone(),
            self.subscribe_manager.clone(),
            response_queue_rx,
            self.client_poll.clone(),
            self.stop_sx.clone(),
        )
        .await;
        self.network_connection_type = NetworkConnectionType::TCPS;
        info!("MQTT TCP TLS Server started successfully, listening port: {port}");
    }
}
