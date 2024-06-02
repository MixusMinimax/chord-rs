/*
 * Copyright (c) 2024 Maximilian Barmetler <https://barmetler.com>
 *
 * Use of this source code is governed by an MIT-style
 * license that can be found in the LICENSE file or at
 * https://opensource.org/licenses/MIT.
 */

use std::sync::{Arc, Mutex, MutexGuard, OnceLock};

use cached::{Cached, SizedCache};
use shaku::{Component, Interface};
use tonic::transport::Channel;

use chord_types::node_info::NodeInfo;

use crate::config::ConfigProvider;
use crate::node::BoxedNode;
use crate::node_grpc_client::NodeGrpcClient;

pub trait NodeClientFactory: Interface {
    fn create_node_client(&self, node_info: &NodeInfo) -> BoxedNode;
}

#[derive(Component)]
#[shaku(interface = NodeClientFactory)]
pub struct GrpcNodeClientFactory {
    #[shaku(inject)]
    config_provider: Arc<dyn ConfigProvider>,

    #[shaku(default)]
    channels: OnceLock<Mutex<SizedCache<u64, Channel>>>,
}

impl GrpcNodeClientFactory {
    fn channels(&self) -> MutexGuard<SizedCache<u64, Channel>> {
        self.channels
            .get_or_init(|| Mutex::new(SizedCache::with_size(1024)))
            .lock()
            .unwrap()
    }

    fn get_channel(&self, node_info: &NodeInfo) -> Channel {
        let mut channels = self.channels();
        channels
            .cache_get_or_set_with(node_info.id, || {
                let ref config = self.config_provider.get_config().client_config;
                Channel::builder(node_info.uri())
                    .connect_timeout(config.connect_timeout)
                    .timeout(config.request_timeout)
                    .keep_alive_timeout(config.keep_alive_timeout)
                    .connect_lazy()
            })
            .clone()
    }
}

impl NodeClientFactory for GrpcNodeClientFactory {
    fn create_node_client(&self, node_info: &NodeInfo) -> BoxedNode {
        let channel = self.get_channel(node_info);
        Box::new(NodeGrpcClient::new(*node_info, channel))
    }
}
