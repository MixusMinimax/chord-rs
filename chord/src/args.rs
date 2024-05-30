/*
 * Copyright (c) 2024 Maximilian Barmetler <https://barmetler.com>
 *
 * Use of this source code is governed by an MIT-style
 * license that can be found in the LICENSE file or at
 * https://opensource.org/licenses/MIT.
 */

use std::net::{Ipv4Addr, Ipv6Addr};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// The number of virtual nodes to use.
    #[arg(long, default_value = "1")]
    pub virtual_nodes: u32,

    #[arg(long)]
    pub ipv4addr: Option<Ipv4Addr>,

    #[arg(long)]
    pub ipv4port: Option<u16>,

    #[arg(long)]
    pub ipv6addr: Option<Ipv6Addr>,

    #[arg(long)]
    pub ipv6port: Option<u16>,
}
