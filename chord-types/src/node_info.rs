/*
 * Copyright (c) 2024 Maximilian Barmetler <https://barmetler.com>
 *
 * Use of this source code is governed by an MIT-style
 * license that can be found in the LICENSE file or at
 * https://opensource.org/licenses/MIT.
 */

use std::net::IpAddr;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct NodeInfo {
    pub id: u64,
    pub address: IpAddr,
    pub port: u16,
}
