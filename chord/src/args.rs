/*
 * Copyright (c) 2024 Maximilian Barmetler <https://barmetler.com>
 *
 * Use of this source code is governed by an MIT-style
 * license that can be found in the LICENSE file or at
 * https://opensource.org/licenses/MIT.
 */

/*
 * Make sure not to use imports in this module, as it is included in the `build.rs` script for
 * generating completions.
 */

use std::net::SocketAddr;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// The number of virtual nodes to use.
    #[arg(short = 'n', long, default_value = "1")]
    pub virtual_nodes: u32,

    /// The addresses to bind the grpc interfaces to.
    ///
    /// Supports both IPv4 and IPv6 addresses.
    #[arg(
        short = 'a',
        long = "address",
        value_name = "ADDRESS",
        value_delimiter = ','
    )]
    pub socket_addresses: Vec<SocketAddr>,
}
