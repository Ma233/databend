// Copyright 2022 Datafuse Labs.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! `common_storage` will provide storage related types and functions.
//!
//! Databend Query will have three kinds of storage operators, visit
//! [RFC: Cache](https://databend.rs/doc/contributing/rfcs/cache) for
//! more detailed information.
//!
//! - data operator: All data will be persisted until users delete them.
//! - cache operator: Backends could have their GC or background auto eviction logic, which means cache services is non-persist.
//! - temporary operator: Backend will be configured with TTL and timely delete old data.
//!
//! Users can use different operator based on their own needs, for example:
//!
//! - Users table data must be accessed via data operator
//! - Table snapshots, segments cache must be stored accessed via cache operator.
//! - Intermediate data generated by query could be stored by temporary operator.

#![allow(clippy::uninlined_format_args)]
#![feature(no_sanitize)]
#![feature(io_error_other)]

mod config;
pub use config::CacheConfig;
pub use config::ShareTableConfig;
pub use config::StorageAzblobConfig;
pub use config::StorageConfig;
pub use config::StorageFsConfig;
pub use config::StorageFtpConfig;
pub use config::StorageGcsConfig;
pub use config::StorageHdfsConfig;
pub use config::StorageHttpConfig;
pub use config::StorageIpfsConfig;
pub use config::StorageMokaConfig;
pub use config::StorageObsConfig;
pub use config::StorageOssConfig;
pub use config::StorageParams;
pub use config::StorageRedisConfig;
pub use config::StorageS3Config;
pub use config::STORAGE_FTP_DEFAULT_ENDPOINT;
pub use config::STORAGE_GCS_DEFAULT_ENDPOINT;
pub use config::STORAGE_IPFS_DEFAULT_ENDPOINT;
pub use config::STORAGE_S3_DEFAULT_ENDPOINT;

mod operator;
pub use operator::init_operator;
pub use operator::CacheOperator;
pub use operator::DataOperator;

mod metrics;
pub use metrics::StorageMetrics;
pub use metrics::StorageMetricsLayer;

mod runtime_layer;
mod utils;

mod cache;
pub use cache::FuseCachePolicy;

mod column_node;
pub use column_node::ColumnNode;
pub use column_node::ColumnNodes;
