use std::time::{Duration, Instant};

/// Trait for formatting Duration values into human-readable strings
/// 
/// Provides consistent formatting of time durations across different time scales,
/// from nanoseconds to seconds.
pub trait ReportDuration {
    /// Converts the duration into a human-readable string representation
    fn report(&self) -> String;
}

impl ReportDuration for Duration {
    /// Formats the duration using appropriate units based on its magnitude:
    /// - For durations >= 1 second: Returns "seconds.milliseconds"
    /// - For durations < 1 second: Uses the most appropriate unit (ns, μs, or ms)
    /// 
    /// # Examples
    /// ```
    /// use std::time::Duration;
    /// use stopwatch::ReportDuration;
    /// 
    /// let duration = Duration::from_secs(2);
    /// assert_eq!(duration.report(), "2.00s");
    /// 
    /// let duration = Duration::from_nanos(500);
    /// assert_eq!(duration.report(), "500ns");
    /// ```
    fn report(&self) -> String {
        let seconds = self.as_secs();
        if seconds > 0 {
            format!("{seconds}.{:.2}s", self.as_millis())
        } else {
            match self.as_nanos() {
                ..1_000 => format!("{}ns", self.as_nanos()),
                1_000..1_000_000 => format!("{}μs", self.as_micros()),
                1_000_000.. => format!("{}ms", self.as_millis()),
            }
        }
    }
}

/// A stopwatch implementation for measuring elapsed time with lap and split functionality
///
/// The stopwatch can be started, stopped, reset, and queried for various timing measurements.
/// It keeps track of total elapsed time and supports lap timing and split timing operations.
#[derive(Clone, Debug, Default)]
pub struct Stopwatch {
    /// Indicates whether the stopwatch is currently running
    pub is_running: bool,
    elapsed: Duration,
    last_start: Option<Instant>,   
    lap_start: Option<Instant>,
}

impl std::fmt::Display for Stopwatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.elapsed().report())
    }
}

impl Stopwatch {
    /// Creates a new stopwatch initialized to zero and not running
    /// 
    /// # Examples
    /// ```
    /// use everybody_codes::stopwatch::Stopwatch;
    /// let stopwatch = Stopwatch::new();
    /// assert!(!stopwatch.is_running);
    /// ```
    pub fn new() -> Self {
        Self {
            is_running: false,
            elapsed: Duration::ZERO,
            last_start: None,
            lap_start: None,
        }
    }

    /// Starts the stopwatch if it isn't already running
    /// 
    /// # Returns
    /// - `true` if the stopwatch was successfully started
    /// - `false` if the stopwatch was already running
    pub fn start(&mut self) -> bool {
        if self.is_running {
            false
        } else {
            self.last_start = Some(Instant::now());
            self.lap_start = self.last_start;
            self.is_running = true;
            true
        }
    }

    /// Stops the stopwatch and returns the total elapsed time
    /// 
    /// If the stopwatch is running, adds the time since the last start
    /// to the total elapsed time. If already stopped, returns the current
    /// elapsed time without modification.
    /// 
    /// # Returns
    /// The total elapsed time the stopwatch has been running
    pub fn stop(&mut self) -> Duration {
        if self.is_running {
            let now = Instant::now();
            self.is_running = false;
            self.elapsed += now - self.last_start.unwrap();
            self.last_start = None;
            self.lap_start = None;
        }
        self.elapsed
    }

    /// Records a lap time and starts timing a new lap
    /// 
    /// # Returns
    /// - If running: Duration of the completed lap
    /// - If stopped: Duration::ZERO
    pub fn lap(&mut self) -> Duration {
        if self.is_running {
            let now = Instant::now();
            let lap = now - self.lap_start.unwrap();
            self.lap_start = Some(now);
            lap
        } else {
            Duration::ZERO
        }
    }

    /// Returns the time elapsed while the stopwatch has been running
    /// without affecting the running state
    /// 
    /// # Returns
    /// - If running: Time since last start
    /// - If stopped: Duration::ZERO
    pub fn elapsed(&self) -> Duration {
        if self.is_running {
            let now = Instant::now();
            self.elapsed + (now - self.last_start.unwrap())
        } else {
            self.elapsed
        }
    }

    /// Resets the stopwatch to its initial state
    /// 
    /// Clears all accumulated time and stops the stopwatch if it was running
    pub fn reset(&mut self) {
        self.elapsed = Duration::ZERO;
        self.last_start = None;
        self.lap_start = None;
        self.is_running = false;
    }
}
