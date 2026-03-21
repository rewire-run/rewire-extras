use re_types_core::{
    try_serialize_field, AsComponents, ComponentDescriptor, SerializedComponentBatch,
};
use rerun::components::Text;

/// Custom Rerun archetype for ROS 2 topic metadata.
///
/// Logged at `/rewire/topics` as a batch where each field is a column with one
/// entry per topic. Re-logged whenever the set of subscribed topics changes.
///
/// All fields are stored as [`Text`] components with custom
/// [`ComponentDescriptor`]s under the `rewire.ROS2TopicInfo` archetype namespace.
///
/// # Example
///
/// ```
/// use rewire_extras::{ROS2TopicInfo, TopicMeta};
///
/// let topics = vec![
///     TopicMeta { name: "/chatter", type_name: "std_msgs/String", publishers: 1, subscribers: 2 },
///     TopicMeta { name: "/odom", type_name: "nav_msgs/Odometry", publishers: 1, subscribers: 0 },
/// ];
/// let info = ROS2TopicInfo::new(&topics);
/// let batches = re_types_core::AsComponents::as_serialized_batches(&info);
/// assert_eq!(batches.len(), 4);
/// ```
pub struct ROS2TopicInfo {
    topic_names: Option<SerializedComponentBatch>,
    type_names: Option<SerializedComponentBatch>,
    publisher_counts: Option<SerializedComponentBatch>,
    subscriber_counts: Option<SerializedComponentBatch>,
}

/// Per-topic metadata passed to [`ROS2TopicInfo::new`].
///
/// # Fields
///
/// - `name` — fully qualified topic name (e.g., `"/chatter"`)
/// - `type_name` — ROS 2 message type (e.g., `"std_msgs/String"`)
/// - `publishers` — number of publishers on this topic
/// - `subscribers` — number of subscribers on this topic
pub struct TopicMeta<'a> {
    pub name: &'a str,
    pub type_name: &'a str,
    pub publishers: usize,
    pub subscribers: usize,
}

impl ROS2TopicInfo {
    /// Creates a new [`ROS2TopicInfo`] from a slice of topic metadata.
    ///
    /// Each field is serialized as a [`Text`] column. Publisher and subscriber
    /// counts are converted to strings for batch-oriented storage.
    pub fn new(topics: &[TopicMeta<'_>]) -> Self {
        let names: Vec<Text> = topics.iter().map(|t| Text::from(t.name)).collect();
        let types: Vec<Text> = topics.iter().map(|t| Text::from(t.type_name)).collect();
        let pubs: Vec<Text> = topics
            .iter()
            .map(|t| Text::from(t.publishers.to_string()))
            .collect();
        let subs: Vec<Text> = topics
            .iter()
            .map(|t| Text::from(t.subscribers.to_string()))
            .collect();

        Self {
            topic_names: try_serialize_field::<Text>(Self::descriptor_topic_name(), names),
            type_names: try_serialize_field::<Text>(Self::descriptor_type_name(), types),
            publisher_counts: try_serialize_field::<Text>(Self::descriptor_publisher_count(), pubs),
            subscriber_counts: try_serialize_field::<Text>(
                Self::descriptor_subscriber_count(),
                subs,
            ),
        }
    }

    /// Returns the [`ComponentDescriptor`] for the topic name column.
    pub fn descriptor_topic_name() -> ComponentDescriptor {
        ComponentDescriptor::partial("rewire.ROS2TopicInfo:topic_name")
            .with_archetype("rewire.ROS2TopicInfo".into())
    }

    /// Returns the [`ComponentDescriptor`] for the type name column.
    pub fn descriptor_type_name() -> ComponentDescriptor {
        ComponentDescriptor::partial("rewire.ROS2TopicInfo:type_name")
            .with_archetype("rewire.ROS2TopicInfo".into())
    }

    /// Returns the [`ComponentDescriptor`] for the publisher count column.
    pub fn descriptor_publisher_count() -> ComponentDescriptor {
        ComponentDescriptor::partial("rewire.ROS2TopicInfo:publisher_count")
            .with_archetype("rewire.ROS2TopicInfo".into())
    }

    /// Returns the [`ComponentDescriptor`] for the subscriber count column.
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
            &self.publisher_counts,
            &self.subscriber_counts,
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
        let topics = vec![
            TopicMeta {
                name: "/chatter",
                type_name: "std_msgs/String",
                publishers: 1,
                subscribers: 2,
            },
            TopicMeta {
                name: "/odom",
                type_name: "nav_msgs/Odometry",
                publishers: 3,
                subscribers: 0,
            },
        ];
        let info = ROS2TopicInfo::new(&topics);
        let batches = info.as_serialized_batches();
        assert_eq!(batches.len(), 4);
    }

    #[test]
    fn empty_topics_produces_empty_columns() {
        let info = ROS2TopicInfo::new(&[]);
        let batches = info.as_serialized_batches();
        for batch in &batches {
            assert_eq!(batch.array.len(), 0);
        }
    }

    #[test]
    fn descriptors_have_archetype_annotation() {
        let desc = ROS2TopicInfo::descriptor_topic_name();
        assert!(format!("{:?}", desc).contains("ROS2TopicInfo"));
    }

    #[test]
    fn all_descriptors_are_distinct() {
        let descs = [
            ROS2TopicInfo::descriptor_topic_name(),
            ROS2TopicInfo::descriptor_type_name(),
            ROS2TopicInfo::descriptor_publisher_count(),
            ROS2TopicInfo::descriptor_subscriber_count(),
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
