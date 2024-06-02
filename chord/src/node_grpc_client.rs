/*
 * Copyright (c) 2024 Maximilian Barmetler <https://barmetler.com>
 *
 * Use of this source code is governed by an MIT-style
 * license that can be found in the LICENSE file or at
 * https://opensource.org/licenses/MIT.
 */

use async_trait::async_trait;
use tonic::Request;
use tonic::transport::Channel;

use chord_types::node_info::NodeInfo;

use crate::api::com::barmetler::chord::{FindSuccessorRequest, GetPredecessorRequest};
use crate::api::com::barmetler::chord::node_service_client::NodeServiceClient;
use crate::convert::TryToDomain;
use crate::node::{Node, NodeError};

pub struct NodeGrpcClient {
    node_info: NodeInfo,
    channel: Channel,
}

impl NodeGrpcClient {
    pub fn new(node_info: NodeInfo, channel: Channel) -> Self {
        Self {
            node_info: node_info.to_owned(),
            channel,
        }
    }
}

impl NodeGrpcClient {
    fn client(&self) -> NodeServiceClient<Channel> {
        NodeServiceClient::new(self.channel.clone())
    }
}

#[async_trait]
impl Node for NodeGrpcClient {
    fn id(&self) -> u64 {
        self.node_info.id
    }

    async fn find_successor(&self, id: u64) -> Result<NodeInfo, NodeError> {
        Ok(self
            .client()
            .find_successor(Request::new(FindSuccessorRequest {
                node_id: self.node_info.id.to_string(),
                id: id.to_string(),
            }))
            .await?
            .into_inner()
            .node
            .ok_or(NodeError::invalid_response(self.node_info.id))?
            .try_to_domain()?)
    }

    async fn get_predecessor(&self) -> Result<NodeInfo, NodeError> {
        Ok(self
            .client()
            .get_predecessor(Request::new(GetPredecessorRequest {
                node_id: self.node_info.id.to_string(),
            }))
            .await?
            .into_inner()
            .node
            .ok_or(NodeError::invalid_response(self.node_info.id))?
            .try_to_domain()?)
    }
}
