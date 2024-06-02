/*
 * Copyright (c) 2024 Maximilian Barmetler <https://barmetler.com>
 *
 * Use of this source code is governed by an MIT-style
 * license that can be found in the LICENSE file or at
 * https://opensource.org/licenses/MIT.
 */

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use shaku::{Component, Interface};

use crate::node::DynNode;

pub trait NodeManager: Interface {
    fn initialize(&self, nodes: HashMap<u64, Arc<DynNode>>);

    fn get_node(&self, id: u64) -> Option<Arc<DynNode>>;
}

#[derive(Component)]
#[shaku(interface = NodeManager)]
pub struct NodeManagerImpl {
    #[shaku(default)]
    nodes: RwLock<HashMap<u64, Arc<DynNode>>>,
}

impl NodeManager for NodeManagerImpl {
    fn initialize(&self, nodes: HashMap<u64, Arc<DynNode>>) {
        *self.nodes.write().unwrap() = nodes;
    }

    fn get_node(&self, id: u64) -> Option<Arc<DynNode>> {
        self.nodes.read().unwrap().get(&id).cloned()
    }
}
