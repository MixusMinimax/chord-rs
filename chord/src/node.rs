/*
 * Copyright (c) 2024 Maximilian Barmetler <https://barmetler.com>
 *
 * Use of this source code is governed by an MIT-style
 * license that can be found in the LICENSE file or at
 * https://opensource.org/licenses/MIT.
 */

use cached::{Cached, TimedSizedCache};
use futures::future::BoxFuture;
use futures::FutureExt;
use thiserror::Error;
use tokio::sync::{Mutex, RwLock};
use tonic::{async_trait, Code, Status};
use tonic::codegen::Body;

use chord_types::finger_table::FingerTable;
use chord_types::node_info::NodeInfo;

use crate::convert::ConversionError;

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
    node_statuses: Mutex<TimedSizedCache<NodeInfo, BoxFuture<'static, NodeStatus>>>,
}

impl NodeConstructor for NodeImpl {
    fn new_with_id(id: u64) -> Self {
        Self::new(id)
    }
}

enum NodeStatus {
    Alive,
    Dead,
}

impl NodeImpl {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            finger_table: Default::default(),
            node_statuses: Mutex::new(TimedSizedCache::with_size_and_lifespan(1024, 60)),
        }
    }

    async fn check_node(&self, node_info: NodeInfo) -> NodeStatus {
        self.node_statuses
            .lock()
            .await
            .cache_get_or_set_with(node_info, || {
                async move {
                    // TODO: communicate with node
                    NodeStatus::Alive
                }
                .boxed()
            })
            .await
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
        for predecessor in finger_table.get_predecessors() {
            match self.check_node(predecessor.node_info).await {
                NodeStatus::Alive => return Ok(predecessor.node_info),
                NodeStatus::Dead => {
                    log::debug!("predecessor {} is dead", predecessor.node_info.id)
                }
            };
        }
        todo!()
    }
}

#[derive(Clone, Debug, Error)]
pub enum NodeError {
    #[error("invalid response from node")]
    InvalidResponse(u64),
    #[error(transparent)]
    StatusError(#[from] Status),
    #[error(transparent)]
    ConversionError(#[from] ConversionError),
    #[error("unknown error")]
    Unknown,
}

impl NodeError {
    pub fn invalid_response(id: u64) -> Self {
        NodeError::InvalidResponse(id)
    }

    pub fn status_error(status: impl Into<Status>) -> Self {
        NodeError::StatusError(status.into())
    }

    pub fn unknown() -> Self {
        NodeError::Unknown
    }

    pub fn get_code(&self) -> Code {
        match self {
            NodeError::InvalidResponse(_) => Code::InvalidArgument,
            NodeError::StatusError(_) => Code::Internal,
            NodeError::ConversionError(_) => Code::Internal,
            NodeError::Unknown => Code::Unknown,
        }
    }
}
