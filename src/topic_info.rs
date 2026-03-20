use re_types_core::{AsComponents, ComponentDescriptor, SerializedComponentBatch, try_serialize_field};
use rerun::components::Text;

/// Custom archetype for ROS 2 topic metadata.
///
/// Logged statically at `/rewire/topics/{topic_name}` by the bridge when a topic is subscribed.
/// Queried by the custom Topics SpaceView in the rewire viewer.
pub struct ROS2TopicInfo {
    topic_name: Option<SerializedComponentBatch>,
    type_name: Option<SerializedComponentBatch>,
    status: Option<SerializedComponentBatch>,
}

impl ROS2TopicInfo {
    /// Creates a new topic info with status "active".
    pub fn new(topic_name: &str, type_name: &str) -> Self {
        Self {
            topic_name: try_serialize_field::<Text>(
                Self::descriptor_topic_name(),
                [Text::from(topic_name)],
            ),
            type_name: try_serialize_field::<Text>(
                Self::descriptor_type_name(),
                [Text::from(type_name)],
            ),
            status: try_serialize_field::<Text>(
                Self::descriptor_status(),
                [Text::from("active")],
            ),
        }
    }

    /// Overrides the status field.
    pub fn with_status(mut self, status: &str) -> Self {
        self.status = try_serialize_field::<Text>(
            Self::descriptor_status(),
            [Text::from(status)],
        );
        self
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
        [&self.topic_name, &self.type_name, &self.status]
            .into_iter()
            .flatten()
            .cloned()
            .collect()
    }
}
