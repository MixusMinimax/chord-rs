/*
 * Copyright (c) 2024 Maximilian Barmetler <https://barmetler.com>
 *
 * Use of this source code is governed by an MIT-style
 * license that can be found in the LICENSE file or at
 * https://opensource.org/licenses/MIT.
 */

use std::ops::Bound;
use std::sync::Arc;

use cached::{Cached, TimedSizedCache};
use futures::future::BoxFuture;
use futures::FutureExt;
use thiserror::Error;
use tokio::sync::{Mutex, RwLock};
use tonic::{async_trait, Code, Status};

use chord_types::finger_table::FingerTable;
use chord_types::node_info::NodeInfo;

use crate::convert::ConversionError;
use crate::node_client_factory::NodeClientFactory;
use crate::util::looping_range::LoopingRange;

pub type DynNode = dyn Node + Send + Sync;
pub type BoxedNode = Box<DynNode>;

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct FindSuccessorParameters {
    pub id: u64,
    pub iterate: bool,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum FindSuccessorResult {
    Successor(NodeInfo),
    ClosestPrecedingNode(NodeInfo),
}

#[async_trait]
pub trait Node {
    fn id(&self) -> u64;

    async fn find_successor(
        &self,
        parameters: FindSuccessorParameters,
    ) -> Result<FindSuccessorResult, NodeError>;

    async fn get_predecessor(&self) -> Result<NodeInfo, NodeError>;
}

pub struct NodeImpl {
    pub id: u64,
    finger_table: RwLock<FingerTable>,
    node_statuses: Mutex<TimedSizedCache<NodeInfo, BoxFuture<'static, NodeStatus>>>,
    grpc_node_client_factory: Arc<dyn NodeClientFactory>,
}

impl NodeImpl {
    pub fn new(id: u64, client_factory: Arc<dyn NodeClientFactory>) -> Self {
        Self {
            id,
            finger_table: Default::default(),
            node_statuses: Mutex::new(TimedSizedCache::with_size_and_lifespan(1024, 60)),
            grpc_node_client_factory: client_factory,
        }
    }
}

enum NodeStatus {
    Alive,
    Dead,
}

impl NodeImpl {
    fn get_node(&self, node_info: &NodeInfo) -> BoxedNode {
        self.grpc_node_client_factory.create_node_client(node_info)
    }

    async fn check_node(&self, node_info: NodeInfo) -> NodeStatus {
        self.node_statuses
            .lock()
            .await
            .cache_get_or_set_with(node_info, || {
                let node = self.get_node(&node_info);
                async move {
                    match node.get_predecessor().await {
                        Ok(_) => NodeStatus::Alive,
                        Err(_) => NodeStatus::Dead,
                    };
                    NodeStatus::Alive
                }
                .boxed()
            })
            .await
    }
}

#[async_trait]
impl Node for NodeImpl {
    fn id(&self) -> u64 {
        self.id
    }

    async fn find_successor(
        &self,
        FindSuccessorParameters { id, .. }: FindSuccessorParameters,
    ) -> Result<FindSuccessorResult, NodeError> {
        let finger_table = self.finger_table.read().await;
        // find direct successor
        for successor in finger_table.get_successors() {
            if (
                Bound::Excluded(self.id),
                Bound::Included(successor.node_info.id),
            )
                .contains_looping(&id)
            {
                match self.check_node(successor.node_info).await {
                    NodeStatus::Alive => {
                        return Ok(FindSuccessorResult::Successor(successor.node_info))
                    }
                    NodeStatus::Dead => {
                        log::debug!("successor {} is dead", successor.node_info.id);
                    }
                }
            }
        }
        // find the closest preceding node
        todo!();
    }

    async fn get_predecessor(&self) -> Result<NodeInfo, NodeError> {
        let finger_table = self.finger_table.read().await;
        for predecessor in finger_table.get_predecessors() {
            match self.check_node(predecessor.node_info).await {
                NodeStatus::Alive => return Ok(predecessor.node_info),
                NodeStatus::Dead => {
                    log::debug!("predecessor {} is dead", predecessor.node_info.id);
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
