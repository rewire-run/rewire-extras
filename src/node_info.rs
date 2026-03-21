use re_types_core::{AsComponents, ComponentDescriptor, SerializedComponentBatch, try_serialize_field};
use rerun::components::Text;

/// Custom archetype for ROS 2 node metadata.
///
/// Logged at `/rewire/nodes` as a batch — each field is a column with one entry per node.
/// Re-logged whenever the set of discovered nodes changes.
pub struct ROS2NodeInfo {
    node_names: Option<SerializedComponentBatch>,
    publisher_counts: Option<SerializedComponentBatch>,
    subscriber_counts: Option<SerializedComponentBatch>,
    transports: Option<SerializedComponentBatch>,
}

/// Per-node metadata passed to [`ROS2NodeInfo::new`].
pub struct NodeMeta<'a> {
    pub name: &'a str,
    pub publisher_count: usize,
    pub subscriber_count: usize,
    pub transport: &'a str,
}

impl ROS2NodeInfo {
    pub fn new(nodes: &[NodeMeta<'_>]) -> Self {
        let names: Vec<Text> = nodes.iter().map(|n| Text::from(n.name)).collect();
        let pubs: Vec<Text> = nodes.iter().map(|n| Text::from(n.publisher_count.to_string())).collect();
        let subs: Vec<Text> = nodes.iter().map(|n| Text::from(n.subscriber_count.to_string())).collect();
        let transports: Vec<Text> = nodes.iter().map(|n| Text::from(n.transport)).collect();

        Self {
            node_names: try_serialize_field::<Text>(Self::descriptor_node_name(), names),
            publisher_counts: try_serialize_field::<Text>(Self::descriptor_publisher_count(), pubs),
            subscriber_counts: try_serialize_field::<Text>(Self::descriptor_subscriber_count(), subs),
            transports: try_serialize_field::<Text>(Self::descriptor_transport(), transports),
        }
    }

    pub fn descriptor_node_name() -> ComponentDescriptor {
        ComponentDescriptor::partial("rewire.ROS2NodeInfo:node_name")
            .with_archetype("rewire.ROS2NodeInfo".into())
    }

    pub fn descriptor_publisher_count() -> ComponentDescriptor {
        ComponentDescriptor::partial("rewire.ROS2NodeInfo:publisher_count")
            .with_archetype("rewire.ROS2NodeInfo".into())
    }

    pub fn descriptor_subscriber_count() -> ComponentDescriptor {
        ComponentDescriptor::partial("rewire.ROS2NodeInfo:subscriber_count")
            .with_archetype("rewire.ROS2NodeInfo".into())
    }

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
