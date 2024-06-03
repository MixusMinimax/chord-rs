/*
 * Copyright (c) 2024 Maximilian Barmetler <https://barmetler.com>
 *
 * Use of this source code is governed by an MIT-style
 * license that can be found in the LICENSE file or at
 * https://opensource.org/licenses/MIT.
 */

use std::net::{SocketAddrV4, SocketAddrV6};
use std::sync::Arc;
use std::time::Duration;

use serde::{Deserialize, Serialize};
use shaku::{Component, Interface};

pub trait ConfigProvider: Interface {
    fn get_config(&self) -> Arc<Config>;
}

#[derive(Component)]
#[shaku(interface = ConfigProvider)]
pub struct DefaultConfigProvider {
    config: Arc<Config>,
}

impl ConfigProvider for DefaultConfigProvider {
    fn get_config(&self) -> Arc<Config> {
        self.config.clone()
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub virtual_nodes: u32,
    pub peer_interfaces: Vec<PeerInterface>,
    pub client_config: ClientConfig,
}

#[derive(Clone, Eq, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct ClientConfig {
    pub connect_timeout: Duration,
    pub request_timeout: Duration,
    pub keep_alive_timeout: Duration,
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum PeerInterface {
    GrpcIpV4(SocketAddrV4),
    GrpcIpV6(SocketAddrV6),
}
