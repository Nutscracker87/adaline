use crate::sample::Sample;

pub struct Perceptron {
    weights: Vec<Vec<f32>>,
    learning_rate: f32,
}

impl Perceptron {
    pub fn new(num_outputs: usize, num_inputs: usize, lr: f32) -> Self {
        Self {
            weights: vec![vec![0.0; num_inputs]; num_outputs],
            learning_rate: lr,
        }
    }

    pub fn dot_product(input: &[f32], weights: &[f32]) -> f32 {
        input.iter().zip(weights).map(|(x, w)| x * w).sum()
    }

    // Go over all neurons and returns neuron idx and val, with better Score
    pub fn predict(&self, input: &[f32]) -> (usize, f32) {
        self.weights
            .iter()
            .enumerate()
            .map(|(idx, w_row)| (idx, Self::dot_product(input, w_row)))
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .unwrap()
    }

    // One training step for all neurons
    pub fn training_step(&mut self, input: &[f32], target_idx: usize) -> f32 {
        let mut total_error = 0.0;
        let num_neurons = self.weights.len();

        for neuron_idx in 0..num_neurons {
            let prediction = Self::dot_product(input, &self.weights[neuron_idx]);
            let target = if neuron_idx == target_idx { 1.0 } else { -1.0 };
            let error = target - prediction;

            total_error += error.powi(2);

            // Update weights
            for wi in 0..self.weights[neuron_idx].len() {
                self.weights[neuron_idx][wi] += 2.0 * self.learning_rate * error * input[wi];
            }
        }

        total_error
    }

    // Training epoch for one sample
    pub fn train_epoch(&mut self, samples: &[Sample]) -> f32 {
        let mut epoch_error = 0.0;

        for (idx, sample) in samples.iter().enumerate() {
            // for each sample call training step
            epoch_error += self.training_step(&sample.grid, idx);
        }

        epoch_error
    }
}
