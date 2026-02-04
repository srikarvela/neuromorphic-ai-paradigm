//! simulation.rs
//!
//! Core simulation loop for spiking neural networks.
//!
//! This module implements a simple time-stepped simulation engine that
//! advances neuron states, detects spike events, and records spike timing.
//! While simplified, this structure mirrors how event-driven neuromorphic
//! systems operate at a conceptual level.

use crate::neuron::{Neuron, NeuronParams};
use crate::spike::Spike;
use std::fs::File;
use std::io::{BufWriter, Write};
use crate::synapse::Synapse;
use crate::stdp::STDPParams;

/// Simulation configuration parameters.
#[derive(Debug, Clone)]
pub struct SimulationConfig {
    /// Simulation time step (ms)
    pub dt: f64,
    /// Total simulation duration (ms)
    pub t_max: f64,
}

/// A minimal spiking neural network simulation.
pub struct Simulation {
    neurons: Vec<Neuron>,
    synapses: Vec<Synapse>,
    stdp_params: STDPParams,
    config: SimulationConfig,
    time: f64,
}

/// Snapshot of synaptic weight at a given time.
#[derive(Debug)]
struct WeightRecord {
    time: f64,
    pre: usize,
    post: usize,
    weight: f64,
}

impl Simulation {
    /// Create a new simulation with identical neuron parameters.
    pub fn new(
        num_neurons: usize,
        neuron_params: NeuronParams,
        config: SimulationConfig,
        stdp_params: STDPParams,
        initial_weight: f64,
    ) -> Self {
        let neurons = (0..num_neurons)
            .map(|_| Neuron::new(neuron_params.clone()))
            .collect();

        // Fully connected feedforward synapses (excluding self-connections)
        let mut synapses = Vec::new();
        for pre in 0..num_neurons {
            for post in 0..num_neurons {
                if pre != post {
                    synapses.push(Synapse::new(pre, post, initial_weight));
                }
            }
        }

        Self {
            neurons,
            synapses,
            stdp_params,
            config,
            time: 0.0,
        }
    }

    /// Run the simulation and return all emitted spike events and weight log.
    ///
    /// `input_current_fn` provides external input current as a function
    /// of neuron index and simulation time.
    pub fn run<F>(&mut self, input_current_fn: F) -> (Vec<Spike>, Vec<(f64, usize, usize, f64)>)
    where
        F: Fn(usize, f64) -> f64,
    {
        let mut spikes: Vec<Spike> = Vec::new();
        let mut weight_log: Vec<WeightRecord> = Vec::new();

        while self.time < self.config.t_max {
            for (i, neuron) in self.neurons.iter_mut().enumerate() {
                let input_current = input_current_fn(i, self.time);
                let fired = neuron.step(input_current, self.config.dt);

                if fired {
                    spikes.push(Spike::new(i, self.time));

                    // Notify synapses of spike events
                    for syn in self.synapses.iter_mut() {
                        if syn.pre_neuron == i {
                            syn.on_pre_spike(self.time, &self.stdp_params);
                        }
                        if syn.post_neuron == i {
                            syn.on_post_spike(self.time, &self.stdp_params);
                        }
                    }
                    // Log synaptic weights after learning event
                    for syn in self.synapses.iter() {
                        weight_log.push(WeightRecord {
                            time: self.time,
                            pre: syn.pre_neuron,
                            post: syn.post_neuron,
                            weight: syn.weight,
                        });
                    }
                }
            }

            self.time += self.config.dt;
        }

        (
            spikes,
            weight_log
                .into_iter()
                .map(|r| (r.time, r.pre, r.post, r.weight))
                .collect(),
        )
    }

    /// Write spike events to a CSV file for downstream analysis.
    pub fn write_spikes_to_csv(&self, spikes: &[Spike], path: &str) {
        let file = File::create(path)
            .expect("Failed to create spike CSV file");
        let mut writer = BufWriter::new(file);

        // CSV header
        writeln!(writer, "neuron_id,time_ms")
            .expect("Failed to write CSV header");

        for spike in spikes {
            writeln!(writer, "{},{}", spike.neuron_id, spike.time)
                .expect("Failed to write spike row");
        }
    }
    /// Write synaptic weight evolution to CSV.
    pub fn write_weights_to_csv(
        &self,
        weights: &[(f64, usize, usize, f64)],
        path: &str,
    ) {
        let file = File::create(path)
            .expect("Failed to create weights CSV file");
        let mut writer = BufWriter::new(file);

        writeln!(writer, "time_ms,pre_neuron,post_neuron,weight")
            .expect("Failed to write weights CSV header");

        for (t, pre, post, w) in weights {
            writeln!(writer, "{},{},{},{}", t, pre, post, w)
                .expect("Failed to write weight row");
        }
    }
}