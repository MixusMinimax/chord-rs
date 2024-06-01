/*
 * Copyright (c) 2024 Maximilian Barmetler <https://barmetler.com>
 *
 * Use of this source code is governed by an MIT-style
 * license that can be found in the LICENSE file or at
 * https://opensource.org/licenses/MIT.
 */

use rand::random;

use crate::node::{Node, NodeConstructor};

pub trait NodeFactory<T: Node> {
    fn create_node(&mut self) -> T;
}

pub struct DefaultNodeFactory;

impl<T: Node + NodeConstructor> NodeFactory<T> for DefaultNodeFactory {
    /// Create a new node with a random id.
    ///
    /// Uses [rand::thread_rng] to generate a random id.
    fn create_node(&mut self) -> T {
        T::new_with_id(random())
    }
}
