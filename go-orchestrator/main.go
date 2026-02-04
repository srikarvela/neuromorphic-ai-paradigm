package main

import (
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
)

func main() {
	fmt.Println("=== Neuromorphic Experiment Orchestrator ===")

	// Resolve project root (go-orchestrator/..)
	projectRoot, err := filepath.Abs("..")
	if err != nil {
		panic(err)
	}

	// --- Run Rust simulation ---
	rustDir := filepath.Join(projectRoot, "rust-core")
	rustCmd := exec.Command("cargo", "run")
	rustCmd.Dir = rustDir
	rustCmd.Stdout = os.Stdout
	rustCmd.Stderr = os.Stderr

	fmt.Println("Running Rust simulation...")
	if err := rustCmd.Run(); err != nil {
		fmt.Println("Rust simulation failed:", err)
		return
	}

	// --- Verify outputs ---
	spikes := filepath.Join(projectRoot, "data", "raw", "spikes.csv")
	weights := filepath.Join(projectRoot, "data", "raw", "weights.csv")

	if _, err := os.Stat(spikes); err == nil {
		fmt.Println("✓ spikes.csv found")
	} else {
		fmt.Println("✗ spikes.csv missing")
	}

	if _, err := os.Stat(weights); err == nil {
		fmt.Println("✓ weights.csv found")
	} else {
		fmt.Println("✗ weights.csv missing")
	}

	fmt.Println("Experiment completed successfully.")
}