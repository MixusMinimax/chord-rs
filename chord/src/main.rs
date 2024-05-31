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
use uuid::Uuid;

use args::Args;

use crate::node::Node;
use crate::node_grpc_service::NodeGrpcService;

mod api;
mod args;
mod node;
mod node_grpc_service;

fn main() {
    let args = Args::parse();

    let nodes: HashMap<_, _> = repeat_with(|| Node::new(Uuid::new_v4()))
        .map(|node| (node.id, node))
        .take(args.virtual_nodes as usize)
        .collect();
    let nodes = Arc::new(nodes);

    let node_grpc_service = NodeGrpcService::new(nodes);
}
