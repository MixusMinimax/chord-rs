/*
 * Copyright (c) 2024 Maximilian Barmetler <https://barmetler.com>
 *
 * Use of this source code is governed by an MIT-style
 * license that can be found in the LICENSE file or at
 * https://opensource.org/licenses/MIT.
 */

use std::collections::HashMap;
use std::sync::Arc;

use uuid::Uuid;

use crate::node::Node;

pub struct NodeGrpcService {}

impl NodeGrpcService {
    pub fn new(nodes: Arc<HashMap<Uuid, Node>>) -> Self {
        Self {}
    }
}
