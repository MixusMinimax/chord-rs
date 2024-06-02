/*
 * Copyright (c) 2024 Maximilian Barmetler <https://barmetler.com>
 *
 * Use of this source code is governed by an MIT-style
 * license that can be found in the LICENSE file or at
 * https://opensource.org/licenses/MIT.
 */

use std::net::IpAddr;

use http::Uri;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct NodeInfo {
    pub id: u64,
    pub address: IpAddr,
    pub port: u16,
}

impl NodeInfo {
    pub fn uri(&self) -> Uri {
        Uri::builder()
            .scheme("http")
            .authority(format!("{}:{}", self.address, self.port))
            .build()
            .unwrap()
    }
}
