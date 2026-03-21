use re_types_core::{
    try_serialize_field, AsComponents, ComponentDescriptor, SerializedComponentBatch,
};
use re_sdk_types::components::Text;

/// Custom Rerun archetype for per-topic diagnostics data.
///
/// Logged at `/rewire/diagnostics` as a batch where each field is a column with
/// one entry per subscribed topic. Re-logged every second when diagnostics are enabled.
///
/// All fields are stored as [`Text`] components. Numeric values are serialized
/// as strings for batch-oriented storage.
///
/// # Example
///
/// ```
/// use rewire_extras::{ROS2DiagnosticsInfo, DiagnosticsMeta};
///
/// let metrics = vec![
///     DiagnosticsMeta {
///         topic: "/chatter",
///         hz: 10.0,
///         bytes_per_sec: 1024.0,
///         drops: 0,
///         latency_ms: Some(1.5),
///     },
/// ];
/// let info = ROS2DiagnosticsInfo::new(&metrics);
/// let batches = re_types_core::AsComponents::as_serialized_batches(&info);
/// assert_eq!(batches.len(), 5);
/// ```
pub struct ROS2DiagnosticsInfo {
    topic_names: Option<SerializedComponentBatch>,
    hz_values: Option<SerializedComponentBatch>,
    bytes_per_sec_values: Option<SerializedComponentBatch>,
    drop_counts: Option<SerializedComponentBatch>,
    latency_ms_values: Option<SerializedComponentBatch>,
}

/// Per-topic diagnostics metadata passed to [`ROS2DiagnosticsInfo::new`].
///
/// # Fields
///
/// - `topic` — fully qualified topic name (e.g., `"/chatter"`)
/// - `hz` — message frequency in Hz
/// - `bytes_per_sec` — throughput in bytes per second
/// - `drops` — cumulative dropped message count
/// - `latency_ms` — EMA latency in milliseconds, or `None` when sim time is active
///   or no latency data is available
pub struct DiagnosticsMeta<'a> {
    pub topic: &'a str,
    pub hz: f64,
    pub bytes_per_sec: f64,
    pub drops: u64,
    pub latency_ms: Option<f64>,
}

impl ROS2DiagnosticsInfo {
    /// Creates a new [`ROS2DiagnosticsInfo`] from a slice of per-topic diagnostics.
    ///
    /// Each field is serialized as a [`Text`] column. `latency_ms` is stored as
    /// an empty string when `None`.
    pub fn new(metrics: &[DiagnosticsMeta<'_>]) -> Self {
        let topics: Vec<Text> = metrics.iter().map(|m| Text::from(m.topic)).collect();
        let hz: Vec<Text> = metrics
            .iter()
            .map(|m| Text::from(format!("{:.1}", m.hz)))
            .collect();
        let bps: Vec<Text> = metrics
            .iter()
            .map(|m| Text::from(format!("{:.1}", m.bytes_per_sec)))
            .collect();
        let drops: Vec<Text> = metrics
            .iter()
            .map(|m| Text::from(m.drops.to_string()))
            .collect();
        let latency: Vec<Text> = metrics
            .iter()
            .map(|m| Text::from(m.latency_ms.map(|v| format!("{v:.1}")).unwrap_or_default()))
            .collect();

        Self {
            topic_names: try_serialize_field::<Text>(Self::descriptor_topic_name(), topics),
            hz_values: try_serialize_field::<Text>(Self::descriptor_hz(), hz),
            bytes_per_sec_values: try_serialize_field::<Text>(
                Self::descriptor_bytes_per_sec(),
                bps,
            ),
            drop_counts: try_serialize_field::<Text>(Self::descriptor_drops(), drops),
            latency_ms_values: try_serialize_field::<Text>(Self::descriptor_latency_ms(), latency),
        }
    }

    /// Returns the [`ComponentDescriptor`] for the topic name column.
    pub fn descriptor_topic_name() -> ComponentDescriptor {
        ComponentDescriptor::partial("rewire.ROS2DiagnosticsInfo:topic_name")
            .with_archetype("rewire.ROS2DiagnosticsInfo".into())
    }

    /// Returns the [`ComponentDescriptor`] for the Hz column.
    pub fn descriptor_hz() -> ComponentDescriptor {
        ComponentDescriptor::partial("rewire.ROS2DiagnosticsInfo:hz")
            .with_archetype("rewire.ROS2DiagnosticsInfo".into())
    }

    /// Returns the [`ComponentDescriptor`] for the bytes/sec column.
    pub fn descriptor_bytes_per_sec() -> ComponentDescriptor {
        ComponentDescriptor::partial("rewire.ROS2DiagnosticsInfo:bytes_per_sec")
            .with_archetype("rewire.ROS2DiagnosticsInfo".into())
    }

    /// Returns the [`ComponentDescriptor`] for the drops column.
    pub fn descriptor_drops() -> ComponentDescriptor {
        ComponentDescriptor::partial("rewire.ROS2DiagnosticsInfo:drops")
            .with_archetype("rewire.ROS2DiagnosticsInfo".into())
    }

    /// Returns the [`ComponentDescriptor`] for the latency column.
    pub fn descriptor_latency_ms() -> ComponentDescriptor {
        ComponentDescriptor::partial("rewire.ROS2DiagnosticsInfo:latency_ms")
            .with_archetype("rewire.ROS2DiagnosticsInfo".into())
    }
}

impl AsComponents for ROS2DiagnosticsInfo {
    fn as_serialized_batches(&self) -> Vec<SerializedComponentBatch> {
        [
            &self.topic_names,
            &self.hz_values,
            &self.bytes_per_sec_values,
            &self.drop_counts,
            &self.latency_ms_values,
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
        let metrics = vec![
            DiagnosticsMeta {
                topic: "/chatter",
                hz: 10.0,
                bytes_per_sec: 1024.0,
                drops: 0,
                latency_ms: Some(1.5),
            },
            DiagnosticsMeta {
                topic: "/odom",
                hz: 50.0,
                bytes_per_sec: 4096.0,
                drops: 3,
                latency_ms: None,
            },
        ];
        let info = ROS2DiagnosticsInfo::new(&metrics);
        let batches = info.as_serialized_batches();
        assert_eq!(batches.len(), 5);
    }

    #[test]
    fn empty_metrics_produces_empty_columns() {
        let info = ROS2DiagnosticsInfo::new(&[]);
        let batches = info.as_serialized_batches();
        for batch in &batches {
            assert_eq!(batch.array.len(), 0);
        }
    }

    #[test]
    fn descriptors_have_archetype_annotation() {
        let desc = ROS2DiagnosticsInfo::descriptor_topic_name();
        assert!(format!("{:?}", desc).contains("ROS2DiagnosticsInfo"));
    }

    #[test]
    fn all_descriptors_are_distinct() {
        let descs = [
            ROS2DiagnosticsInfo::descriptor_topic_name(),
            ROS2DiagnosticsInfo::descriptor_hz(),
            ROS2DiagnosticsInfo::descriptor_bytes_per_sec(),
            ROS2DiagnosticsInfo::descriptor_drops(),
            ROS2DiagnosticsInfo::descriptor_latency_ms(),
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
