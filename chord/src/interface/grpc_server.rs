/*
 * Copyright (c) 2024 Maximilian Barmetler <https://barmetler.com>
 *
 * Use of this source code is governed by an MIT-style
 * license that can be found in the LICENSE file or at
 * https://opensource.org/licenses/MIT.
 */

use std::net::SocketAddr;
use std::sync::Arc;

use async_trait::async_trait;
use log::info;
use shaku::{Component, Interface};
use tokio_util::sync::CancellationToken;
use tonic::{Request, Response, Status};
use tonic::transport::{Error, Server};

use crate::api::com::barmetler::chord::{
    FindSuccessorRequest, FindSuccessorResponse, GetPredecessorRequest, GetPredecessorResponse,
};
use crate::api::com::barmetler::chord::node_service_server::{NodeService, NodeServiceServer};
use crate::node_grpc_service::NodeGrpcServiceComponent;

#[async_trait]
pub trait GrpcServer: Interface {
    async fn run(
        &self,
        socket_addr: SocketAddr,
        shutdown: CancellationToken,
    ) -> Result<(), tonic::transport::Error>;
}

#[derive(Component)]
#[shaku(interface = GrpcServer)]
pub struct GrpcServerImpl {
    #[shaku(inject)]
    node_grpc_service: Arc<dyn NodeGrpcServiceComponent>,
}

#[async_trait]
impl GrpcServer for GrpcServerImpl {
    async fn run(&self, socket_addr: SocketAddr, shutdown: CancellationToken) -> Result<(), Error> {
        Server::builder()
            .add_service(NodeServiceServer::new(NodeServiceWrapper(
                self.node_grpc_service.clone(),
            )))
            .serve_with_shutdown(socket_addr, async {
                info!("Server started on {}", socket_addr);
                shutdown.cancelled().await;
                info!("Shutting down server on {}...", socket_addr);
            })
            .await
    }
}

struct NodeServiceWrapper(Arc<dyn NodeService>);

#[async_trait]
impl NodeService for NodeServiceWrapper {
    async fn find_successor(
        &self,
        request: Request<FindSuccessorRequest>,
    ) -> Result<Response<FindSuccessorResponse>, Status> {
        self.0.find_successor(request).await
    }

    async fn get_predecessor(
        &self,
        request: Request<GetPredecessorRequest>,
    ) -> Result<Response<GetPredecessorResponse>, Status> {
        self.0.get_predecessor(request).await
    }
}
