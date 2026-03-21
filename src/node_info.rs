use re_sdk_types::components::Text;
use re_types_core::{
    try_serialize_field, AsComponents, ComponentDescriptor, SerializedComponentBatch,
};

/// Custom Rerun archetype for ROS 2 node metadata.
///
/// Logged at `/rewire/nodes` as a batch where each field is a column with one
/// entry per node. Re-logged whenever the set of discovered nodes changes.
///
/// All fields are stored as [`Text`] components with custom
/// [`ComponentDescriptor`]s under the `rewire.ROS2NodeInfo` archetype namespace.
///
/// # Example
///
/// ```
/// use rewire_extras::{ROS2NodeInfo, NodeMeta};
///
/// let nodes = vec![
///     NodeMeta { name: "/talker", publisher_count: 1, subscriber_count: 0, transport: "dds" },
/// ];
/// let info = ROS2NodeInfo::new(&nodes);
/// let batches = re_types_core::AsComponents::as_serialized_batches(&info);
/// assert_eq!(batches.len(), 4);
/// ```
pub struct ROS2NodeInfo {
    node_names: Option<SerializedComponentBatch>,
    publisher_counts: Option<SerializedComponentBatch>,
    subscriber_counts: Option<SerializedComponentBatch>,
    transports: Option<SerializedComponentBatch>,
}

/// Per-node metadata passed to [`ROS2NodeInfo::new`].
///
/// # Fields
///
/// - `name` — fully qualified node name (e.g., `"/talker"`)
/// - `publisher_count` — number of topics this node publishes
/// - `subscriber_count` — number of topics this node subscribes to
/// - `transport` — discovery transport layer (e.g., `"dds"`, `"zenoh"`)
pub struct NodeMeta<'a> {
    pub name: &'a str,
    pub publisher_count: usize,
    pub subscriber_count: usize,
    pub transport: &'a str,
}

impl ROS2NodeInfo {
    /// Creates a new [`ROS2NodeInfo`] from a slice of node metadata.
    ///
    /// Each field is serialized as a [`Text`] column. Counts are converted
    /// to strings for batch-oriented storage.
    pub fn new(nodes: &[NodeMeta<'_>]) -> Self {
        let names: Vec<Text> = nodes.iter().map(|n| Text::from(n.name)).collect();
        let pubs: Vec<Text> = nodes
            .iter()
            .map(|n| Text::from(n.publisher_count.to_string()))
            .collect();
        let subs: Vec<Text> = nodes
            .iter()
            .map(|n| Text::from(n.subscriber_count.to_string()))
            .collect();
        let transports: Vec<Text> = nodes.iter().map(|n| Text::from(n.transport)).collect();

        Self {
            node_names: try_serialize_field::<Text>(Self::descriptor_node_name(), names),
            publisher_counts: try_serialize_field::<Text>(Self::descriptor_publisher_count(), pubs),
            subscriber_counts: try_serialize_field::<Text>(
                Self::descriptor_subscriber_count(),
                subs,
            ),
            transports: try_serialize_field::<Text>(Self::descriptor_transport(), transports),
        }
    }

    /// Returns the [`ComponentDescriptor`] for the node name column.
    pub fn descriptor_node_name() -> ComponentDescriptor {
        ComponentDescriptor::partial("rewire.ROS2NodeInfo:node_name")
            .with_archetype("rewire.ROS2NodeInfo".into())
    }

    /// Returns the [`ComponentDescriptor`] for the publisher count column.
    pub fn descriptor_publisher_count() -> ComponentDescriptor {
        ComponentDescriptor::partial("rewire.ROS2NodeInfo:publisher_count")
            .with_archetype("rewire.ROS2NodeInfo".into())
    }

    /// Returns the [`ComponentDescriptor`] for the subscriber count column.
    pub fn descriptor_subscriber_count() -> ComponentDescriptor {
        ComponentDescriptor::partial("rewire.ROS2NodeInfo:subscriber_count")
            .with_archetype("rewire.ROS2NodeInfo".into())
    }

    /// Returns the [`ComponentDescriptor`] for the transport column.
    pub fn descriptor_transport() -> ComponentDescriptor {
        ComponentDescriptor::partial("rewire.ROS2NodeInfo:transport")
            .with_archetype("rewire.ROS2NodeInfo".into())
    }
}

impl AsComponents for ROS2NodeInfo {
    fn as_serialized_batches(&self) -> Vec<SerializedComponentBatch> {
        [
            &self.node_names,
            &self.publisher_counts,
            &self.subscriber_counts,
            &self.transports,
        ]
        .into_iter()
        .flatten()
        .cloned()
        .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_serializes_all_columns() {
        let nodes = vec![
            NodeMeta {
                name: "/talker",
                publisher_count: 1,
                subscriber_count: 0,
                transport: "dds",
            },
            NodeMeta {
                name: "/listener",
                publisher_count: 0,
                subscriber_count: 1,
                transport: "zenoh",
            },
        ];
        let info = ROS2NodeInfo::new(&nodes);
        let batches = info.as_serialized_batches();
        assert_eq!(batches.len(), 4);
    }

    #[test]
    fn empty_nodes_produces_empty_columns() {
        let info = ROS2NodeInfo::new(&[]);
        let batches = info.as_serialized_batches();
        for batch in &batches {
            assert_eq!(batch.array.len(), 0);
        }
    }

    #[test]
    fn descriptors_have_archetype_annotation() {
        let desc = ROS2NodeInfo::descriptor_node_name();
        assert!(format!("{:?}", desc).contains("ROS2NodeInfo"));
    }

    #[test]
    fn all_descriptors_are_distinct() {
        let descs = [
            ROS2NodeInfo::descriptor_node_name(),
            ROS2NodeInfo::descriptor_publisher_count(),
            ROS2NodeInfo::descriptor_subscriber_count(),
            ROS2NodeInfo::descriptor_transport(),
        ];
        for (i, a) in descs.iter().enumerate() {
            for (j, b) in descs.iter().enumerate() {
                if i != j {
                    assert_ne!(a, b);
                }
            }
        }
    }
}
