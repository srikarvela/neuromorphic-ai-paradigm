//! spike.rs
//!
//! Spike event representation.
//!
//! In neuromorphic systems, information is transmitted using discrete
//! spike events rather than continuous-valued signals. Each spike is
//! characterized primarily by *when* it occurs, making time a first-class
//! computational variable.

/// A spike emitted by a neuron at a specific time.
#[derive(Debug, Clone, Copy)]
pub struct Spike {
    /// ID of the neuron that emitted the spike
    pub neuron_id: usize,
    /// Time of spike emission (ms)
    pub time: f64,
}

impl Spike {
    /// Create a new spike event.
    pub fn new(neuron_id: usize, time: f64) -> Self {
        Self { neuron_id, time }
    }
}
