/*
 * Copyright (c) 2024 Maximilian Barmetler <https://barmetler.com>
 *
 * Use of this source code is governed by an MIT-style
 * license that can be found in the LICENSE file or at
 * https://opensource.org/licenses/MIT.
 */

use std::collections::HashMap;
use std::iter::repeat_with;
use std::sync::Arc;

use clap::Parser;
use log::{info, warn};
use tokio::select;
use tokio::signal::ctrl_c;
use tokio::signal::unix::{signal, SignalKind};
use tonic::transport::Server;

use args::Args;

use crate::api::com::barmetler::chord::node_service_server::NodeServiceServer;
use crate::logging::init_logging;
use crate::node::{BoxedNode, NodeImpl};
use crate::node_factory::NodeFactory;
use crate::node_grpc_service::NodeGrpcService;

mod api;
mod args;
mod convert;
mod logging;
mod looping_range;
mod node;
mod node_client_factory;
mod node_factory;
mod node_grpc_client;
mod node_grpc_service;

#[tokio::main]
async fn main() {
    init_logging();

    let args = Args::parse();

    let mut factory = node_factory::DefaultNodeFactory;

    let nodes: HashMap<_, _> = repeat_with(|| factory.create_node())
        .map(|node: NodeImpl| (node.id, Box::new(node) as BoxedNode))
        .take(args.virtual_nodes as usize)
        .collect();
    let nodes = Arc::new(nodes);

    let node_grpc_service = Arc::new(NodeGrpcService::new(nodes.clone()));

    let mut tasks = Vec::new();

    // Start grpc interfaces
    let socket_addresses = args.socket_addresses;
    if socket_addresses.is_empty() {
        warn!("No socket addresses provided, not starting grpc interfaces");
    }
    tasks.extend(socket_addresses.into_iter().map(|address| {
        let node_grpc_service = node_grpc_service.clone();
        tokio::spawn(async move {
            Server::builder()
                .add_service(NodeServiceServer::from_arc(node_grpc_service))
                .serve_with_shutdown(address, async {
                    info!("Server started on {}", address);
                    #[cfg(unix)]
                    {
                        let sig_int = ctrl_c();
                        let mut sig_term = signal(SignalKind::terminate()).unwrap();
                        select! {
                            e = sig_int => e.unwrap(),
                            e = sig_term.recv() => e.unwrap(),
                        }
                    }
                    #[cfg(windows)]
                    ctrl_c().await.unwrap();
                    info!("Shutting down server on {}...", address);
                })
                .await
                .unwrap();
        })
    }));

    // TODO: start interfaces to communicate with local clients (ethernet, pipes, stdin/stdout, etc.)

    for task in tasks {
        task.await.unwrap();
    }

    info!("All servers shut down.");
}
