use std::collections::HashMap;
use std::time::{Duration, Instant};

const DEFAULT_STALENESS: Duration = Duration::from_secs(5);

/// Tracks heartbeats from connected bridge instances.
///
/// Each bridge sends periodic beats identified by a unique string ID.
/// The tracker automatically prunes stale entries when queried for status,
/// using a configurable staleness threshold (default: 5 seconds).
///
/// # Example
///
/// ```
/// use rewire_extras::HeartbeatTracker;
///
/// let mut tracker = HeartbeatTracker::default();
/// assert_eq!(tracker.status(), (false, 0));
///
/// tracker.beat("bridge-001");
/// let (connected, count) = tracker.status();
/// assert!(connected);
/// assert_eq!(count, 1);
/// ```
pub struct HeartbeatTracker {
    bridges: HashMap<String, Instant>,
    staleness: Duration,
}

impl Default for HeartbeatTracker {
    fn default() -> Self {
        Self {
            bridges: HashMap::new(),
            staleness: DEFAULT_STALENESS,
        }
    }
}

impl HeartbeatTracker {
    /// Creates a tracker with a custom staleness threshold.
    ///
    /// Bridges that haven't sent a beat within this duration are considered
    /// disconnected and pruned on the next [`status`](Self::status) call.
    pub fn with_staleness(staleness: Duration) -> Self {
        Self {
            bridges: HashMap::new(),
            staleness,
        }
    }

    /// Records a heartbeat from the given bridge.
    ///
    /// If the bridge already exists, its timestamp is updated. Otherwise a
    /// new entry is created.
    pub fn beat(&mut self, bridge_id: &str) {
        self.bridges.insert(bridge_id.to_owned(), Instant::now());
    }

    /// Returns `(connected, bridge_count)` after pruning stale entries.
    ///
    /// A bridge is considered connected if its last beat is within the
    /// staleness threshold. `connected` is `true` when at least one bridge
    /// is alive.
    pub fn status(&mut self) -> (bool, usize) {
        self.prune();
        let count = self.bridges.len();
        (count > 0, count)
    }

    fn prune(&mut self) {
        let staleness = self.staleness;
        self.bridges
            .retain(|_, last_seen| last_seen.elapsed() < staleness);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_tracker_reports_disconnected() {
        let mut tracker = HeartbeatTracker::default();
        let (connected, count) = tracker.status();
        assert!(!connected);
        assert_eq!(count, 0);
    }

    #[test]
    fn single_beat_reports_connected() {
        let mut tracker = HeartbeatTracker::default();
        tracker.beat("abc123");
        let (connected, count) = tracker.status();
        assert!(connected);
        assert_eq!(count, 1);
    }

    #[test]
    fn multiple_bridges_counted() {
        let mut tracker = HeartbeatTracker::default();
        tracker.beat("bridge-a");
        tracker.beat("bridge-b");
        tracker.beat("bridge-c");
        let (connected, count) = tracker.status();
        assert!(connected);
        assert_eq!(count, 3);
    }

    #[test]
    fn duplicate_beats_do_not_increase_count() {
        let mut tracker = HeartbeatTracker::default();
        tracker.beat("bridge-a");
        tracker.beat("bridge-a");
        tracker.beat("bridge-a");
        let (_, count) = tracker.status();
        assert_eq!(count, 1);
    }

    #[test]
    fn stale_bridges_pruned() {
        let mut tracker = HeartbeatTracker::with_staleness(Duration::from_millis(50));
        tracker.beat("bridge-old");
        std::thread::sleep(Duration::from_millis(100));
        let (connected, count) = tracker.status();
        assert!(!connected);
        assert_eq!(count, 0);
    }

    #[test]
    fn mix_of_stale_and_fresh() {
        let mut tracker = HeartbeatTracker::with_staleness(Duration::from_millis(100));
        tracker.beat("bridge-old");
        std::thread::sleep(Duration::from_millis(150));
        tracker.beat("bridge-new");
        let (connected, count) = tracker.status();
        assert!(connected);
        assert_eq!(count, 1);
    }

    #[test]
    fn rebeat_revives_stale_bridge() {
        let mut tracker = HeartbeatTracker::with_staleness(Duration::from_millis(50));
        tracker.beat("bridge-a");
        std::thread::sleep(Duration::from_millis(100));
        assert_eq!(tracker.status().1, 0);
        tracker.beat("bridge-a");
        assert_eq!(tracker.status().1, 1);
    }

    #[test]
    fn default_staleness_is_five_seconds() {
        let tracker = HeartbeatTracker::default();
        assert_eq!(tracker.staleness, Duration::from_secs(5));
    }
}
