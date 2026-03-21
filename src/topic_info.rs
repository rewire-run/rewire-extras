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
    publisher_counts: Option<SerializedComponentBatch>,
    subscriber_counts: Option<SerializedComponentBatch>,
}

/// Per-topic metadata passed to [`ROS2TopicInfo::new`].
pub struct TopicMeta<'a> {
    pub name: &'a str,
    pub type_name: &'a str,
    pub publishers: usize,
    pub subscribers: usize,
}

impl ROS2TopicInfo {
    pub fn new(topics: &[TopicMeta<'_>]) -> Self {
        let names: Vec<Text> = topics.iter().map(|t| Text::from(t.name)).collect();
        let types: Vec<Text> = topics.iter().map(|t| Text::from(t.type_name)).collect();
        let statuses: Vec<Text> = topics.iter().map(|_| Text::from("active")).collect();
        let pubs: Vec<Text> = topics.iter().map(|t| Text::from(t.publishers.to_string())).collect();
        let subs: Vec<Text> = topics.iter().map(|t| Text::from(t.subscribers.to_string())).collect();

        Self {
            topic_names: try_serialize_field::<Text>(Self::descriptor_topic_name(), names),
            type_names: try_serialize_field::<Text>(Self::descriptor_type_name(), types),
            statuses: try_serialize_field::<Text>(Self::descriptor_status(), statuses),
            publisher_counts: try_serialize_field::<Text>(Self::descriptor_publisher_count(), pubs),
            subscriber_counts: try_serialize_field::<Text>(Self::descriptor_subscriber_count(), subs),
        }
    }

    pub fn descriptor_topic_name() -> ComponentDescriptor {
        ComponentDescriptor::partial("rewire.ROS2TopicInfo:topic_name")
            .with_archetype("rewire.ROS2TopicInfo".into())
    }

    pub fn descriptor_type_name() -> ComponentDescriptor {
        ComponentDescriptor::partial("rewire.ROS2TopicInfo:type_name")
            .with_archetype("rewire.ROS2TopicInfo".into())
    }

    pub fn descriptor_status() -> ComponentDescriptor {
        ComponentDescriptor::partial("rewire.ROS2TopicInfo:status")
            .with_archetype("rewire.ROS2TopicInfo".into())
    }

    pub fn descriptor_publisher_count() -> ComponentDescriptor {
        ComponentDescriptor::partial("rewire.ROS2TopicInfo:publisher_count")
            .with_archetype("rewire.ROS2TopicInfo".into())
    }

    pub fn descriptor_subscriber_count() -> ComponentDescriptor {
        ComponentDescriptor::partial("rewire.ROS2TopicInfo:subscriber_count")
            .with_archetype("rewire.ROS2TopicInfo".into())
    }
}

impl AsComponents for ROS2TopicInfo {
    fn as_serialized_batches(&self) -> Vec<SerializedComponentBatch> {
        [
            &self.topic_names,
            &self.type_names,
            &self.statuses,
            &self.publisher_counts,
            &self.subscriber_counts,
        ]
        .into_iter()
        .flatten()
        .cloned()
        .collect()
    }
}
