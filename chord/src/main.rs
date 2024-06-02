/*
 * Copyright (c) 2024 Maximilian Barmetler <https://barmetler.com>
 *
 * Use of this source code is governed by an MIT-style
 * license that can be found in the LICENSE file or at
 * https://opensource.org/licenses/MIT.
 */

#![feature(trait_upcasting)]

use std::collections::HashMap;
use std::iter::repeat_with;
use std::sync::Arc;

use clap::Parser;
use log::{info, warn};
use rand::random;
use shaku::{HasComponent, module};

use args::Args;
use node_factory::DefaultNodeFactory;

use crate::config::{Config, DefaultConfigProvider, DefaultConfigProviderParameters};
use crate::interface::grpc_server::{GrpcServer, GrpcServerImpl};
use crate::logging::init_logging;
use crate::node_client_factory::GrpcNodeClientFactory;
use crate::node_factory::NodeFactory;
use crate::node_grpc_service::NodeGrpcService;
use crate::node_manager::{NodeManager, NodeManagerImpl};
use crate::util::shutdown_source::start_shutdown_listener;

mod api;
mod args;
mod config;
mod convert;
mod interface;
mod logging;
mod node;
mod node_client_factory;
mod node_factory;
mod node_grpc_client;
mod node_grpc_service;
mod node_manager;
mod util;

#[tokio::main]
async fn main() {
    init_logging();

    let args = Args::parse();

    let cancellation = start_shutdown_listener();

    let program = Program::builder()
        .with_component_parameters::<DefaultConfigProvider>(DefaultConfigProviderParameters {
            config: Arc::new(Config {
                virtual_nodes: args.virtual_nodes,
                socket_addresses: Vec::new(),
            }),
        })
        .build();

    let factory: Arc<dyn NodeFactory> = program.resolve();
    let node_manager: Arc<dyn NodeManager> = program.resolve();

    let nodes: HashMap<_, _> = repeat_with(|| factory.create_node(random()))
        .map(|node| (node.id(), Arc::from(node)))
        .take(args.virtual_nodes as usize)
        .collect();
    node_manager.initialize(nodes);

    let mut tasks = Vec::new();

    // Start grpc interfaces
    let socket_addresses = args.socket_addresses;
    if socket_addresses.is_empty() {
        warn!("No socket addresses provided, not starting grpc interfaces");
    }
    tasks.extend(socket_addresses.into_iter().map(|address| {
        let grpc_server: Arc<dyn GrpcServer> = program.resolve();
        let cancellation = cancellation.clone();
        tokio::spawn(async move {
            grpc_server.run(address, cancellation).await.unwrap();
        })
    }));

    // TODO: start interfaces to communicate with local clients (ethernet, pipes, stdin/stdout, etc.)

    for task in tasks {
        task.await.unwrap();
    }

    info!("All servers shut down.");
}

module! {
    Program {
        components = [
            DefaultConfigProvider,
            DefaultNodeFactory,
            GrpcNodeClientFactory,
            GrpcServerImpl,
            NodeGrpcService,
            NodeManagerImpl,
        ],
        providers = []
    }
}
