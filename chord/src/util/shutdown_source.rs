/*
 * Copyright (c) 2024 Maximilian Barmetler <https://barmetler.com>
 *
 * Use of this source code is governed by an MIT-style
 * license that can be found in the LICENSE file or at
 * https://opensource.org/licenses/MIT.
 */

use tokio::select;
use tokio::signal::ctrl_c;
use tokio::signal::unix::{signal, SignalKind};
use tokio_util::sync::CancellationToken;

pub fn start_shutdown_listener() -> CancellationToken {
    let token = CancellationToken::new();
    let token_clone = token.clone();
    tokio::spawn(async move {
        #[cfg(unix)]
        {
            let sig_int = ctrl_c();
            let mut sig_term = signal(SignalKind::terminate()).unwrap();
            select! {
                e = sig_int => e.unwrap(),
                e = sig_term.recv() => e.unwrap(),
            }
        }
        #[cfg(windows)]
        ctrl_c().await.unwrap();
        token.cancel();
    });
    token_clone
}
