/*
 * Copyright (c) 2024 Maximilian Barmetler <https://barmetler.com>
 *
 * Use of this source code is governed by an MIT-style
 * license that can be found in the LICENSE file or at
 * https://opensource.org/licenses/MIT.
 */

use clap::Parser;

use args::Args;

mod api;
mod args;

fn main() {
    let args = Args::parse();

    println!("{:?}", args);
}
