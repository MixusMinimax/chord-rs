/*
 * Copyright (c) 2024 Maximilian Barmetler <https://barmetler.com>
 *
 * Use of this source code is governed by an MIT-style
 * license that can be found in the LICENSE file or at
 * https://opensource.org/licenses/MIT.
 */

use uuid::Uuid;

pub struct Node {
    pub id: Uuid,
}

impl Node {
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }
}
