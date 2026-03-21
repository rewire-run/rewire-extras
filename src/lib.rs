//! Shared types for the Rewire ecosystem.
//!
//! Provides custom [Rerun](https://rerun.io) archetypes for ROS 2 metadata and
//! a heartbeat tracker for bridge connection detection. Used by both the bridge
//! (logging) and the viewer (querying).
//!
//! # Archetypes
//!
//! - [`ROS2TopicInfo`] — topic metadata (name, type, publisher/subscriber counts)
//! - [`ROS2NodeInfo`] — node metadata (name, pub/sub counts, transport)
//!
//! Both archetypes implement [`re_types_core::AsComponents`] and are logged as
//! batched columns at a single entity path.
//!
//! # Heartbeat
//!
//! - [`HeartbeatTracker`] — tracks bridge connection status via timestamped beats
//!
//! # gRPC
//!
//! - [`proto`] — generated gRPC service definitions for viewer–bridge communication

mod diagnostics_info;
mod heartbeat;
mod node_info;
mod topic_info;

pub use diagnostics_info::{DiagnosticsMeta, ROS2DiagnosticsInfo};
pub use heartbeat::HeartbeatTracker;
pub use node_info::{NodeMeta, ROS2NodeInfo};
pub use topic_info::{ROS2TopicInfo, TopicMeta};

/// Generated gRPC service definitions for viewer–bridge communication.
pub mod proto;
