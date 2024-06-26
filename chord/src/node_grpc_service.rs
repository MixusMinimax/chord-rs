/*
 * Copyright (c) 2024 Maximilian Barmetler <https://barmetler.com>
 *
 * Use of this source code is governed by an MIT-style
 * license that can be found in the LICENSE file or at
 * https://opensource.org/licenses/MIT.
 */

use std::num::ParseIntError;
use std::sync::Arc;

use async_trait::async_trait;
use shaku::{Component, Interface};
use thiserror::Error;
use tonic::{Code, Request, Response, Status};

use crate::api::com::barmetler::chord::{
    find_successor_response, FindSuccessorRequest, FindSuccessorResponse, GetPredecessorRequest,
    GetPredecessorResponse,
};
use crate::api::com::barmetler::chord::node_service_server::NodeService;
use crate::convert::ToProto;
use crate::node::{DynNode, FindSuccessorParameters, FindSuccessorResult, NodeError};
use crate::node_manager::NodeManager;

pub trait NodeGrpcServiceComponent: NodeService + Interface {}

#[derive(Component)]
#[shaku(interface = NodeGrpcServiceComponent)]
pub struct NodeGrpcService {
    #[shaku(inject)]
    node_manager: Arc<dyn NodeManager>,
}

impl NodeGrpcServiceComponent for NodeGrpcService {}

impl NodeGrpcService {
    fn find_node(&self, id: u64) -> Result<Arc<DynNode>, NodeServiceError> {
        self.node_manager
            .get_node(id)
            .ok_or(NodeServiceError::node_not_found(id))
    }

    fn find_node_by_id_string(
        &self,
        id: impl AsRef<str>,
    ) -> Result<Arc<DynNode>, NodeServiceError> {
        let id = id_from_string(id)?;
        self.find_node(id)
    }
}

#[async_trait]
impl NodeService for NodeGrpcService {
    async fn find_successor(
        &self,
        request: Request<FindSuccessorRequest>,
    ) -> Result<Response<FindSuccessorResponse>, Status> {
        let request = request.into_inner();
        let node = self.find_node_by_id_string(request.node_id)?;
        let id = id_from_string(request.id)?;
        let result = node
            .find_successor(FindSuccessorParameters {
                id,
                ..Default::default()
            })
            .await
            .map_err(NodeServiceError::from)?;
        Ok(Response::new(FindSuccessorResponse {
            node: Some(match result {
                FindSuccessorResult::Successor(node) => {
                    find_successor_response::Node::Successor(node.to_proto())
                }
                FindSuccessorResult::ClosestPrecedingNode(node) => {
                    find_successor_response::Node::ClosestPrecedingNode(node.to_proto())
                }
            }),
        }))
    }

    async fn get_predecessor(
        &self,
        request: Request<GetPredecessorRequest>,
    ) -> Result<Response<GetPredecessorResponse>, Status> {
        let request = request.into_inner();
        let node = self.find_node_by_id_string(request.node_id)?;
        let node_info = node
            .get_predecessor()
            .await
            .map_err(NodeServiceError::from)?;
        Ok(Response::new(GetPredecessorResponse {
            node: Some(node_info.to_proto()),
        }))
    }
}

#[derive(Clone, Debug, Error)]
pub enum NodeServiceError {
    #[error("node not found: {0}")]
    NodeNotFound(u64),
    #[error("invalid id string: {id_string}")]
    InvalidIdString {
        id_string: String,
        #[source]
        source: ParseIntError,
    },
    #[error(transparent)]
    NodeError(#[from] NodeError),
    #[error("unknown error")]
    Unknown,
}

impl NodeServiceError {
    pub fn node_not_found(id: u64) -> Self {
        NodeServiceError::NodeNotFound(id)
    }

    pub fn invalid_id_string(id: impl Into<String>, source: ParseIntError) -> Self {
        NodeServiceError::InvalidIdString {
            id_string: id.into(),
            source,
        }
    }

    pub fn unknown() -> Self {
        NodeServiceError::Unknown
    }
}

impl From<NodeServiceError> for Status {
    fn from(value: NodeServiceError) -> Self {
        let message = value.to_string();
        let code = match value {
            NodeServiceError::NodeNotFound(_) => Code::NotFound,
            NodeServiceError::InvalidIdString { .. } => Code::InvalidArgument,
            NodeServiceError::NodeError(node_error) => node_error.get_code(),
            NodeServiceError::Unknown => Code::Unknown,
        };
        Status::new(code, message)
    }
}

fn id_from_string(id: impl AsRef<str>) -> Result<u64, NodeServiceError> {
    id.as_ref()
        .parse()
        .map_err(|e| NodeServiceError::invalid_id_string(id.as_ref(), e))
}
