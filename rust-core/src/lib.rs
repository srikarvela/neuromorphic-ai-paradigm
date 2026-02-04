//! lib.rs
//!
//! Neuromorphic spiking neural network core library.
//!
//! This crate provides a minimal, research-oriented software prototype
//! demonstrating key neuromorphic computing principles:
//! - Event-driven spike-based communication
//! - Time-based neuron dynamics
//! - Local state and learning (no backpropagation)

pub mod neuron;
pub mod spike;
pub mod simulation;
pub mod synapse;
pub mod stdp;

use neuron::NeuronParams;
use simulation::{Simulation, SimulationConfig};
use stdp::STDPParams;

/// Run a minimal example simulation.
///
/// This function is intended for quick validation and experimentation.
/// It simulates a small population of neurons receiving constant input
/// current and returns the emitted spike events.
pub fn run_example() {
    let neuron_params = NeuronParams {
        tau_m: 10.0,
        v_rest: 0.0,
        v_thresh: 1.0,
        v_reset: 0.0,
    };

    let sim_config = SimulationConfig {
        dt: 0.1,
        t_max: 100.0,
    };

    let stdp_params = STDPParams {
        a_plus: 0.01,
        a_minus: 0.012,
        tau_plus: 20.0,
        tau_minus: 20.0,
        w_min: 0.0,
        w_max: 1.0,
    };

    let mut sim = Simulation::new(
        3,
        neuron_params,
        sim_config,
        stdp_params,
        0.5,
    );

    let (spikes, weights) = sim.run(|neuron_id, _time| {
        1.2 + 0.05 * neuron_id as f64
    });


    use std::path::PathBuf;

    // Resolve project root (neuromorphic-ai-paradigm/)
    let project_root: PathBuf = std::env::current_dir()
        .expect("Failed to get current directory")
        .parent()
        .expect("Failed to resolve project root")
        .to_path_buf();

    let csv_path = project_root
        .join("data")
        .join("raw")
        .join("spikes.csv");

    let weights_csv_path = project_root
        .join("data")
        .join("raw")
        .join("weights.csv");

    sim.write_spikes_to_csv(
        &spikes,
        csv_path
            .to_str()
            .expect("Failed to convert CSV path to string"),
    );

    sim.write_weights_to_csv(
        &weights,
        weights_csv_path
            .to_str()
            .expect("Failed to convert weights CSV path to string"),
    );

    println!("Simulation complete. Emitted {} spikes.", spikes.len());

    for spike in spikes.iter().take(10) {
        println!(
            "Spike from neuron {} at time {:.2} ms",
            spike.neuron_id, spike.time
        );
    }

    println!(
        "Recorded {} synaptic weight updates.",
        weights.len()
    );
}