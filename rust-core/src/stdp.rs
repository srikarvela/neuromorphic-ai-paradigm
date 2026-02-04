//! stdp.rs
//!
//! Spike-Timing Dependent Plasticity (STDP).
//!
//! STDP is a local, biologically inspired learning rule where synaptic
//! strength changes based on the relative timing of pre- and post-synaptic
//! spikes. This contrasts with backpropagation-based learning and is a core
//! idea in neuromorphic computing.

/// Parameters controlling the STDP learning rule.
#[derive(Debug, Clone)]
pub struct STDPParams {
    /// Learning rate for potentiation (LTP)
    pub a_plus: f64,
    /// Learning rate for depression (LTD)
    pub a_minus: f64,
    /// Time constant for potentiation (ms)
    pub tau_plus: f64,
    /// Time constant for depression (ms)
    pub tau_minus: f64,
    /// Minimum synaptic weight
    pub w_min: f64,
    /// Maximum synaptic weight
    pub w_max: f64,
}

/// Clamp synaptic weight to biologically plausible bounds.
pub fn clamp_weight(w: f64, w_min: f64, w_max: f64) -> f64 {
    if w < w_min {
        w_min
    } else if w > w_max {
        w_max
    } else {
        w
    }
}

/// Compute synaptic weight change based on spike timing.
///
/// # Arguments
/// * `delta_t` - Time difference between post- and pre-synaptic spikes
///               (t_post - t_pre)
/// * `params` - STDP parameters
///
/// # Returns
/// * Weight change Δw
pub fn stdp_update(delta_t: f64, params: &STDPParams) -> f64 {
    if delta_t > 0.0 {
        // Pre-synaptic spike occurred before post-synaptic spike
        params.a_plus * (-delta_t / params.tau_plus).exp()
    } else {
        // Post-synaptic spike occurred before pre-synaptic spike
        -params.a_minus * (delta_t / params.tau_minus).exp()
    }
}

/// Apply STDP update to an existing synaptic weight.
///
/// # Arguments
/// * `w` - current synaptic weight
/// * `delta_t` - t_post - t_pre
/// * `params` - STDP parameters
///
/// # Returns
/// * Updated synaptic weight
pub fn apply_stdp(w: f64, delta_t: f64, params: &STDPParams) -> f64 {
    let dw = stdp_update(delta_t, params);
    clamp_weight(w + dw, params.w_min, params.w_max)
}