//! Adaline (Adaptive Linear Neuron) network with LMS learning. One output neuron per class; struct name "Perceptron" is historical.

use crate::sample::Sample;
use std::collections::HashMap;

/// Adaline (Adaptive Linear Neuron) neural network implementation
/// Uses multiple output neurons, one for each class (character)
pub struct Perceptron {
    weights: Vec<Vec<f32>>,
    learning_rate: f32,
    /// Maps character names to neuron indices for training
    character_to_index: HashMap<String, usize>,
}

impl Perceptron {
    /// Creates a new Adaline network
    /// 
    /// # Arguments
    /// * `num_outputs` - Number of output neurons (one per class)
    /// * `num_inputs` - Number of input features (bias + pixels = 65)
    /// * `lr` - Learning rate
    /// * `characters` - List of character names in the order they correspond to neurons
    pub fn new(num_outputs: usize, num_inputs: usize, lr: f32, characters: &[String]) -> Self {
        // Build mapping from character to neuron index
        let character_to_index: HashMap<String, usize> = characters
            .iter()
            .enumerate()
            .map(|(idx, char)| (char.clone(), idx))
            .collect();

        Self {
            weights: vec![vec![0.0; num_inputs]; num_outputs],
            learning_rate: lr,
            character_to_index,
        }
    }

    /// Computes the dot product of input and weights
    fn dot_product(input: &[f32], weights: &[f32]) -> f32 {
        input.iter().zip(weights).map(|(x, w)| x * w).sum()
    }

    /// Predicts which character the input represents
    /// 
    /// # Returns
    /// A tuple of (neuron_index, activation_score) for the winning neuron
    pub fn predict(&self, input: &[f32]) -> (usize, f32) {
        self.weights
            .iter()
            .enumerate()
            .map(|(idx, w_row)| (idx, Self::dot_product(input, w_row)))
            .max_by(|a, b| {
                a.1.partial_cmp(&b.1)
                    .expect("NaN values should not occur in predictions")
            })
            .expect("Network must have at least one output neuron")
    }

    /// Performs one training step for a single sample
    /// Updates all neuron weights using the LMS (Least Mean Squares) algorithm
    /// 
    /// # Arguments
    /// * `input` - Input feature vector (bias + pixels)
    /// * `target_idx` - Index of the target neuron (should output 1.0, others -1.0)
    /// 
    /// # Returns
    /// Total squared error across all neurons
    fn training_step(&mut self, input: &[f32], target_idx: usize) -> f32 {
        let mut total_error = 0.0;

        for (neuron_idx, weights) in self.weights.iter_mut().enumerate() {
            let prediction = Self::dot_product(input, weights);
            let target = if neuron_idx == target_idx { 1.0 } else { -1.0 };
            let error = target - prediction;

            total_error += error.powi(2);

            // LMS weight update: w_new = w_old + 2 * lr * error * input
            for (w, &x) in weights.iter_mut().zip(input.iter()) {
                *w += 2.0 * self.learning_rate * error * x;
            }
        }

        total_error
    }

    /// Trains the network on all samples for one epoch
    /// 
    /// # Returns
    /// Total mean squared error across all samples
    pub fn train_epoch(&mut self, samples: &[Sample]) -> f32 {
        let mut epoch_error = 0.0;

        for sample in samples {
            let target_idx = self
                .character_to_index
                .get(&sample.character)
                .copied()
                .unwrap_or_else(|| {
                    panic!(
                        "Character '{}' not found in training set",
                        sample.character
                    )
                });
            epoch_error += self.training_step(&sample.grid, target_idx);
        }

        epoch_error / samples.len() as f32 // Return MSE (mean squared error)
    }
}
