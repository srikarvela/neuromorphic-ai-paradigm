

//! neuron.rs
//!
//! Leaky Integrate-and-Fire (LIF) neuron model.
//!
//! This module implements a minimal spiking neuron used in neuromorphic
//! computing research. Unlike conventional artificial neurons that output
//! continuous values, this neuron communicates using discrete spike events
//! whose timing carries information.
//!
//! The model is intentionally simple and software-focused, serving as a
//! conceptual exploration of event-driven, time-based computation.

/// Parameters governing neuron dynamics.
#[derive(Debug, Clone)]
pub struct NeuronParams {
    /// Membrane time constant (ms)
    pub tau_m: f64,
    /// Resting membrane potential
    pub v_rest: f64,
    /// Firing threshold
    pub v_thresh: f64,
    /// Reset potential after a spike
    pub v_reset: f64,
}

/// Leaky Integrate-and-Fire neuron state.
#[derive(Debug, Clone)]
pub struct Neuron {
    /// Current membrane potential
    pub v_mem: f64,
    /// Neuron parameters
    pub params: NeuronParams,
}

impl Neuron {
    /// Create a new neuron with given parameters.
    pub fn new(params: NeuronParams) -> Self {
        Self {
            v_mem: params.v_rest,
            params,
        }
    }

    /// Advance neuron state by one time step.
    ///
    /// # Arguments
    /// * `input_current` - Synaptic input current at this timestep
    /// * `dt` - Time step (ms)
    ///
    /// # Returns
    /// * `true` if the neuron emits a spike
    /// * `false` otherwise
    pub fn step(&mut self, input_current: f64, dt: f64) -> bool {
        // Leaky integration of membrane potential
        let dv = (-(self.v_mem - self.params.v_rest) + input_current) / self.params.tau_m;
        self.v_mem += dv * dt;

        // Check for spike
        if self.v_mem >= self.params.v_thresh {
            self.v_mem = self.params.v_reset;
            true
        } else {
            false
        }
    }
}