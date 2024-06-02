/*
 * Copyright (c) 2024 Maximilian Barmetler <https://barmetler.com>
 *
 * Use of this source code is governed by an MIT-style
 * license that can be found in the LICENSE file or at
 * https://opensource.org/licenses/MIT.
 */

use std::sync::Arc;

use shaku::{Component, Interface};

use crate::node::{BoxedNode, NodeImpl};
use crate::node_client_factory::NodeClientFactory;

pub trait NodeFactory: Interface {
    fn create_node(&self, id: u64) -> BoxedNode;
}

#[derive(Component)]
#[shaku(interface = NodeFactory)]
pub struct DefaultNodeFactory {
    #[shaku(inject)]
    client_factory: Arc<dyn NodeClientFactory>,
}

impl NodeFactory for DefaultNodeFactory {
    /// Create a new node with a random id.
    ///
    /// Uses [rand::thread_rng] to generate a random id.
    fn create_node(&self, id: u64) -> BoxedNode {
        Box::new(NodeImpl::new(id, self.client_factory.clone()))
    }
}
