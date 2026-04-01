use std::collections::HashMap;
use std::time::{Duration, Instant};

const DEFAULT_STALENESS: Duration = Duration::from_secs(5);

/// Whether a bridge is idle (no subscribed topics) or actively streaming.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BridgeState {
    #[default]
    Idle,
    Active,
}

impl From<i32> for BridgeState {
    fn from(value: i32) -> Self {
        match value {
            2 => Self::Active,
            _ => Self::Idle,
        }
    }
}

struct BridgeEntry {
    last_seen: Instant,
    state: BridgeState,
}

/// Tracks heartbeats from connected bridge instances.
///
/// Each bridge sends periodic beats identified by a unique string ID.
/// The tracker automatically prunes stale entries when queried for status,
/// using a configurable staleness threshold (default: 5 seconds).
///
/// # Example
///
/// ```
/// use rewire_extras::{HeartbeatTracker, BridgeState};
///
/// let mut tracker = HeartbeatTracker::default();
/// assert_eq!(tracker.status(), (false, 0, BridgeState::Idle));
///
/// tracker.beat("bridge-001", BridgeState::Active);
/// let (connected, count, state) = tracker.status();
/// assert!(connected);
/// assert_eq!(count, 1);
/// assert_eq!(state, BridgeState::Active);
/// ```
pub struct HeartbeatTracker {
    bridges: HashMap<String, BridgeEntry>,
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
    /// If the bridge already exists, its timestamp and state are updated.
    /// Otherwise a new entry is created.
    pub fn beat(&mut self, bridge_id: &str, state: BridgeState) {
        self.bridges.insert(
            bridge_id.to_owned(),
            BridgeEntry {
                last_seen: Instant::now(),
                state,
            },
        );
    }

    /// Returns `(connected, bridge_count, aggregate_state)` after pruning stale entries.
    ///
    /// A bridge is considered connected if its last beat is within the
    /// staleness threshold. `connected` is `true` when at least one bridge
    /// is alive. The aggregate state is [`BridgeState::Active`] when any
    /// bridge is active, [`BridgeState::Idle`] otherwise.
    pub fn status(&mut self) -> (bool, usize, BridgeState) {
        self.prune();
        let count = self.bridges.len();
        let state = if self
            .bridges
            .values()
            .any(|e| e.state == BridgeState::Active)
        {
            BridgeState::Active
        } else {
            BridgeState::Idle
        };
        (count > 0, count, state)
    }

    fn prune(&mut self) {
        let staleness = self.staleness;
        self.bridges
            .retain(|_, entry| entry.last_seen.elapsed() < staleness);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_tracker_reports_disconnected() {
        let mut tracker = HeartbeatTracker::default();
        let (connected, count, state) = tracker.status();
        assert!(!connected);
        assert_eq!(count, 0);
        assert_eq!(state, BridgeState::Idle);
    }

    #[test]
    fn single_beat_reports_connected() {
        let mut tracker = HeartbeatTracker::default();
        tracker.beat("abc123", BridgeState::Idle);
        let (connected, count, _) = tracker.status();
        assert!(connected);
        assert_eq!(count, 1);
    }

    #[test]
    fn multiple_bridges_counted() {
        let mut tracker = HeartbeatTracker::default();
        tracker.beat("bridge-a", BridgeState::Idle);
        tracker.beat("bridge-b", BridgeState::Idle);
        tracker.beat("bridge-c", BridgeState::Idle);
        let (connected, count, _) = tracker.status();
        assert!(connected);
        assert_eq!(count, 3);
    }

    #[test]
    fn duplicate_beats_do_not_increase_count() {
        let mut tracker = HeartbeatTracker::default();
        tracker.beat("bridge-a", BridgeState::Idle);
        tracker.beat("bridge-a", BridgeState::Idle);
        tracker.beat("bridge-a", BridgeState::Idle);
        let (_, count, _) = tracker.status();
        assert_eq!(count, 1);
    }

    #[test]
    fn stale_bridges_pruned() {
        let mut tracker = HeartbeatTracker::with_staleness(Duration::from_millis(50));
        tracker.beat("bridge-old", BridgeState::Active);
        std::thread::sleep(Duration::from_millis(100));
        let (connected, count, state) = tracker.status();
        assert!(!connected);
        assert_eq!(count, 0);
        assert_eq!(state, BridgeState::Idle);
    }

    #[test]
    fn mix_of_stale_and_fresh() {
        let mut tracker = HeartbeatTracker::with_staleness(Duration::from_millis(100));
        tracker.beat("bridge-old", BridgeState::Idle);
        std::thread::sleep(Duration::from_millis(150));
        tracker.beat("bridge-new", BridgeState::Idle);
        let (connected, count, _) = tracker.status();
        assert!(connected);
        assert_eq!(count, 1);
    }

    #[test]
    fn rebeat_revives_stale_bridge() {
        let mut tracker = HeartbeatTracker::with_staleness(Duration::from_millis(50));
        tracker.beat("bridge-a", BridgeState::Idle);
        std::thread::sleep(Duration::from_millis(100));
        assert_eq!(tracker.status().1, 0);
        tracker.beat("bridge-a", BridgeState::Idle);
        assert_eq!(tracker.status().1, 1);
    }

    #[test]
    fn default_staleness_is_five_seconds() {
        let tracker = HeartbeatTracker::default();
        assert_eq!(tracker.staleness, Duration::from_secs(5));
    }

    #[test]
    fn single_active_bridge_reports_active() {
        let mut tracker = HeartbeatTracker::default();
        tracker.beat("bridge-a", BridgeState::Active);
        let (_, _, state) = tracker.status();
        assert_eq!(state, BridgeState::Active);
    }

    #[test]
    fn mixed_bridges_reports_active() {
        let mut tracker = HeartbeatTracker::default();
        tracker.beat("bridge-a", BridgeState::Idle);
        tracker.beat("bridge-b", BridgeState::Active);
        let (_, _, state) = tracker.status();
        assert_eq!(state, BridgeState::Active);
    }

    #[test]
    fn all_idle_reports_idle() {
        let mut tracker = HeartbeatTracker::default();
        tracker.beat("bridge-a", BridgeState::Idle);
        tracker.beat("bridge-b", BridgeState::Idle);
        let (_, _, state) = tracker.status();
        assert_eq!(state, BridgeState::Idle);
    }

    #[test]
    fn unspecified_proto_state_treated_as_idle() {
        assert_eq!(BridgeState::from(0), BridgeState::Idle);
        assert_eq!(BridgeState::from(1), BridgeState::Idle);
        assert_eq!(BridgeState::from(2), BridgeState::Active);
        assert_eq!(BridgeState::from(99), BridgeState::Idle);
    }
}
