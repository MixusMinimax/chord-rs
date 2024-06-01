/*
 * Copyright (c) 2024 Maximilian Barmetler <https://barmetler.com>
 *
 * Use of this source code is governed by an MIT-style
 * license that can be found in the LICENSE file or at
 * https://opensource.org/licenses/MIT.
 */

use std::convert::Infallible;

use async_trait::async_trait;
use thiserror::Error;

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

pub trait ToDomain<T> {
    fn to_domain(&self) -> T;
}

pub trait TryToDomain<T> {
    type Error: std::error::Error;

    fn try_to_domain(&self) -> Result<T, Self::Error>;
}

#[async_trait]
pub trait ToDomainAsync<T> {
    async fn to_domain(&self) -> T;
}

#[async_trait]
pub trait TryToDomainAsync<T> {
    type Error: std::error::Error;

    async fn try_to_domain(&self) -> Result<T, Self::Error>;
}

impl<T, F: ToDomain<T>> TryToDomain<T> for F {
    type Error = Infallible;

    fn try_to_domain(&self) -> Result<T, Self::Error> {
        Ok(self.to_domain())
    }
}

#[async_trait]
impl<T, F: ToDomainAsync<T> + Send + Sync> TryToDomainAsync<T> for F {
    type Error = Infallible;

    async fn try_to_domain(&self) -> Result<T, Self::Error> {
        Ok(self.to_domain().await)
    }
}

#[async_trait]
impl<T, F: ToDomain<T> + Send + Sync> ToDomainAsync<T> for F {
    async fn to_domain(&self) -> T {
        self.to_domain()
    }
}

#[derive(Debug, Clone, Error)]
pub enum ConversionError {
    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error(transparent)]
    AddrParseError(#[from] std::net::AddrParseError),
    #[error("conversion failed: {0}")]
    ConversionFailed(String),
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

impl TryToDomain<NodeInfo> for NodeInfoMsg {
    type Error = ConversionError;

    fn try_to_domain(&self) -> Result<NodeInfo, Self::Error> {
        Ok(NodeInfo {
            id: self.id.parse()?,
            address: self.ip.parse()?,
            port: self.port as u16,
        })
    }
}
