use re_types_core::{AsComponents, ComponentDescriptor, SerializedComponentBatch, try_serialize_field};
use rerun::components::Text;

/// Custom archetype for ROS 2 topic metadata.
///
/// Logged at `/rewire/topics` as a batch — each field is a column with one entry per topic.
/// Re-logged whenever the set of subscribed topics changes.
pub struct ROS2TopicInfo {
    topic_names: Option<SerializedComponentBatch>,
    type_names: Option<SerializedComponentBatch>,
    statuses: Option<SerializedComponentBatch>,
}

impl ROS2TopicInfo {
    /// Creates topic info from parallel slices of topic names and type names.
    /// All topics default to "active" status.
    pub fn new(topics: &[(&str, &str)]) -> Self {
        let names: Vec<Text> = topics.iter().map(|(n, _)| Text::from(*n)).collect();
        let types: Vec<Text> = topics.iter().map(|(_, t)| Text::from(*t)).collect();
        let statuses: Vec<Text> = topics.iter().map(|_| Text::from("active")).collect();

        Self {
            topic_names: try_serialize_field::<Text>(Self::descriptor_topic_name(), names),
            type_names: try_serialize_field::<Text>(Self::descriptor_type_name(), types),
            statuses: try_serialize_field::<Text>(Self::descriptor_status(), statuses),
        }
    }

    /// Descriptor for the topic_name component.
    pub fn descriptor_topic_name() -> ComponentDescriptor {
        ComponentDescriptor::partial("rewire.ROS2TopicInfo:topic_name")
            .with_archetype("rewire.ROS2TopicInfo".into())
    }

    /// Descriptor for the type_name component.
    pub fn descriptor_type_name() -> ComponentDescriptor {
        ComponentDescriptor::partial("rewire.ROS2TopicInfo:type_name")
            .with_archetype("rewire.ROS2TopicInfo".into())
    }

    /// Descriptor for the status component.
    pub fn descriptor_status() -> ComponentDescriptor {
        ComponentDescriptor::partial("rewire.ROS2TopicInfo:status")
            .with_archetype("rewire.ROS2TopicInfo".into())
    }
}

impl AsComponents for ROS2TopicInfo {
    fn as_serialized_batches(&self) -> Vec<SerializedComponentBatch> {
        [&self.topic_names, &self.type_names, &self.statuses]
            .into_iter()
            .flatten()
            .cloned()
            .collect()
    }
}
