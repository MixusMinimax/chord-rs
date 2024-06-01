/*
 * Copyright (c) 2024 Maximilian Barmetler <https://barmetler.com>
 *
 * Use of this source code is governed by an MIT-style
 * license that can be found in the LICENSE file or at
 * https://opensource.org/licenses/MIT.
 */

use std::convert::Infallible;

use async_trait::async_trait;

use chord_types::node_info::NodeInfo;

use crate::api::com::barmetler::chord::NodeInfo as NodeInfoMsg;

pub trait ToProto<T> {
    fn to_proto(&self) -> T;
}

pub trait TryToProto<T> {
    type Error: std::error::Error;

    fn try_to_proto(&self) -> Result<T, Self::Error>;
}

#[async_trait]
pub trait ToProtoAsync<T> {
    async fn to_proto(&self) -> T;
}

#[async_trait]
pub trait TryToProtoAsync<T> {
    type Error: std::error::Error;

    async fn try_to_proto(&self) -> Result<T, Self::Error>;
}

impl<T, F: ToProto<T>> TryToProto<T> for F {
    type Error = Infallible;

    fn try_to_proto(&self) -> Result<T, Self::Error> {
        Ok(self.to_proto())
    }
}

#[async_trait]
impl<T, F: ToProtoAsync<T> + Send + Sync> TryToProtoAsync<T> for F {
    type Error = Infallible;

    async fn try_to_proto(&self) -> Result<T, Self::Error> {
        Ok(self.to_proto().await)
    }
}

#[async_trait]
impl<T, F: ToProto<T> + Send + Sync> ToProtoAsync<T> for F {
    async fn to_proto(&self) -> T {
        self.to_proto()
    }
}

impl ToProto<NodeInfoMsg> for NodeInfo {
    fn to_proto(&self) -> NodeInfoMsg {
        NodeInfoMsg {
            id: self.id.to_string(),
            ip: self.address.to_string(),
            port: self.port as u32,
        }
    }
}
