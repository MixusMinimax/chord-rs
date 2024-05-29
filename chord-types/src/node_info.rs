/*
 * Copyright (c) 2024 Maximilian Barmetler <https://barmetler.com>
 *
 * Use of this source code is governed by an MIT-style
 * license that can be found in the LICENSE file or at
 * https://opensource.org/licenses/MIT.
 */

use std::net::IpAddr;

use uuid::Uuid;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct NodeInfo {
    pub id: Uuid,
    pub address: IpAddr,
    pub port: u16,
}
