use libopenvx_sys::vx_perf_t;
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

impl From<vx_perf_t> for Performance {
    fn from(perf: vx_perf_t) -> Self {
        Self {
            tmp: Duration::from_nanos(perf.tmp),
            beg: Duration::from_nanos(perf.beg),
            end: Duration::from_nanos(perf.end),
            sum: Duration::from_nanos(perf.sum),
            avg: Duration::from_nanos(perf.avg),
            min: Duration::from_nanos(perf.min),
            num: Duration::from_nanos(perf.num),
            max: Duration::from_nanos(perf.max),
        }
    }
}
