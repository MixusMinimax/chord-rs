/*
 * Copyright (c) 2024 Maximilian Barmetler <https://barmetler.com>
 *
 * Use of this source code is governed by an MIT-style
 * license that can be found in the LICENSE file or at
 * https://opensource.org/licenses/MIT.
 */

use async_trait::async_trait;

use chord_types::node_info::NodeInfo;

use crate::node::BoxedNode;

#[async_trait]
pub trait NodeClientFactory {
    async fn create_node_client(&self, node_info: &NodeInfo) -> BoxedNode;
}

pub struct GrpcNodeClientFactory {}

impl GrpcNodeClientFactory {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl NodeClientFactory for GrpcNodeClientFactory {
    async fn create_node_client(&self, node_info: &NodeInfo) -> BoxedNode {
        todo!()
    }
}
