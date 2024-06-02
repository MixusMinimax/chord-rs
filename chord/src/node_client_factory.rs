/*
 * Copyright (c) 2024 Maximilian Barmetler <https://barmetler.com>
 *
 * Use of this source code is governed by an MIT-style
 * license that can be found in the LICENSE file or at
 * https://opensource.org/licenses/MIT.
 */

use std::sync::Arc;

use async_trait::async_trait;
use shaku::{Component, Interface};

use chord_types::node_info::NodeInfo;

use crate::config::ConfigProvider;
use crate::node::BoxedNode;

#[async_trait]
pub trait NodeClientFactory: Interface {
    async fn create_node_client(&self, node_info: &NodeInfo) -> BoxedNode;
}

#[derive(Component)]
#[shaku(interface = NodeClientFactory)]
pub struct GrpcNodeClientFactory {
    #[shaku(inject)]
    config_provider: Arc<dyn ConfigProvider>,
}

#[async_trait]
impl NodeClientFactory for GrpcNodeClientFactory {
    async fn create_node_client(&self, node_info: &NodeInfo) -> BoxedNode {
        todo!()
    }
}
