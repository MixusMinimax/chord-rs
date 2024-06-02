/*
 * Copyright (c) 2024 Maximilian Barmetler <https://barmetler.com>
 *
 * Use of this source code is governed by an MIT-style
 * license that can be found in the LICENSE file or at
 * https://opensource.org/licenses/MIT.
 */

use std::sync::Arc;

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

pub struct Config {
    pub virtual_nodes: u32,
    pub socket_addresses: Vec<String>,
}

pub enum PeerInterface {
    IpV4,
}
