use std::time::Duration;

/// The performance measurement structure.
#[derive(Debug, Copy, Clone, Default)]
pub struct Performance {
    /// Holds the last measurement.
    pub tmp: Duration,
    /// Holds the first measurement in a set.
    pub beg: Duration,
    /// Holds the last measurement in a set.
    pub end: Duration,
    /// Holds the summation of durations.
    pub sum: Duration,
    /// Holds the average of the durations.
    pub avg: Duration,
    /// Holds the minimum of the durations.
    pub min: Duration,
    /// Holds the number of measurements.
    pub num: Duration,
    /// Holds the maximum of the durations.
    pub max: Duration,
}
