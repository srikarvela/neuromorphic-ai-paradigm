//! synapse.rs
//!
//! Synapse model for spiking neural networks.
//!
//! A synapse connects a pre-synaptic neuron to a post-synaptic neuron
//! and modulates signal transmission via a weight. In neuromorphic systems,
//! synapses are typically the locus of learning through local plasticity
//! rules such as STDP.

use crate::stdp::{apply_stdp, STDPParams};

/// A synapse connecting two neurons.
#[derive(Debug, Clone)]
pub struct Synapse {
    /// Index of pre-synaptic neuron
    pub pre_neuron: usize,
    /// Index of post-synaptic neuron
    pub post_neuron: usize,
    /// Synaptic weight
    pub weight: f64,
    /// Last pre-synaptic spike time (ms)
    pub last_pre_spike: Option<f64>,
    /// Last post-synaptic spike time (ms)
    pub last_post_spike: Option<f64>,
}

impl Synapse {
    /// Create a new synapse with an initial weight.
    pub fn new(pre_neuron: usize, post_neuron: usize, weight: f64) -> Self {
        Self {
            pre_neuron,
            post_neuron,
            weight,
            last_pre_spike: None,
            last_post_spike: None,
        }
    }

    /// Register a pre-synaptic spike and apply STDP if possible.
    pub fn on_pre_spike(&mut self, t_pre: f64, params: &STDPParams) {
        if let Some(t_post) = self.last_post_spike {
            self.weight = apply_stdp(self.weight, t_post - t_pre, params);
        }
        self.last_pre_spike = Some(t_pre);
    }

    /// Register a post-synaptic spike and apply STDP if possible.
    pub fn on_post_spike(&mut self, t_post: f64, params: &STDPParams) {
        if let Some(t_pre) = self.last_pre_spike {
            self.weight = apply_stdp(self.weight, t_post - t_pre, params);
        }
        self.last_post_spike = Some(t_post);
    }
}