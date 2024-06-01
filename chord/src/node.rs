/*
 * Copyright (c) 2024 Maximilian Barmetler <https://barmetler.com>
 *
 * Use of this source code is governed by an MIT-style
 * license that can be found in the LICENSE file or at
 * https://opensource.org/licenses/MIT.
 */

use thiserror::Error;
use tokio::sync::RwLock;
use tonic::{async_trait, Code};

use chord_types::finger_table::FingerTable;
use chord_types::node_info::NodeInfo;

pub type BoxedNode = Box<dyn Node + Send + Sync>;

#[async_trait]
pub trait Node {
    async fn find_successor(&self, id: u64) -> Result<NodeInfo, NodeError>;

    async fn get_predecessor(&self) -> Result<NodeInfo, NodeError>;
}

pub trait NodeConstructor {
    fn new_with_id(id: u64) -> Self;
}

pub struct NodeImpl {
    pub id: u64,
    finger_table: RwLock<FingerTable>,
}

impl NodeImpl {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            finger_table: Default::default(),
        }
    }
}

impl NodeConstructor for NodeImpl {
    fn new_with_id(id: u64) -> Self {
        Self::new(id)
    }
}

#[async_trait]
impl Node for NodeImpl {
    async fn find_successor(&self, id: u64) -> Result<NodeInfo, NodeError> {
        let finger_table = self.finger_table.read().await;
        todo!();
    }

    async fn get_predecessor(&self) -> Result<NodeInfo, NodeError> {
        let finger_table = self.finger_table.read().await;
        todo!()
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Error)]
pub enum NodeError {
    #[error("unknown error")]
    Unknown,
}

impl NodeError {
    pub fn unknown() -> Self {
        NodeError::Unknown
    }

    pub fn get_code(&self) -> Code {
        match self {
            NodeError::Unknown => Code::Unknown,
        }
    }
}
