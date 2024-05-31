/*
 * Copyright (c) 2024 Maximilian Barmetler <https://barmetler.com>
 *
 * Use of this source code is governed by an MIT-style
 * license that can be found in the LICENSE file or at
 * https://opensource.org/licenses/MIT.
 */

pub fn init_logging() {
    let config_str = include_str!("log.toml");
    let config = toml::from_str(config_str).expect("Failed to parse log config");
    log4rs::init_raw_config(config).unwrap();
}
