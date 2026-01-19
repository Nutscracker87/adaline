mod adaline_nn;
mod data_loader;
mod sample;

use adaline_nn::Perceptron;

// Constants
const BIAS_SIZE: usize = 1;
const PIXEL_SIZE: usize = 64;
const INPUT_SIZE: usize = BIAS_SIZE + PIXEL_SIZE; // 65: bias + 64 pixels
const LEARNING_RATE: f32 = 0.01;
const MAX_EPOCHS: usize = 5000;
const TARGET_MSE: f32 = 1e-6;
const PRINT_INTERVAL: usize = 500;

fn main() {
    // 1. Load training and test datasets
    let training_data = data_loader::load_dataset("dataset/training");
    let test_data = data_loader::load_dataset("dataset/test");

    if training_data.is_empty() {
        eprintln!("Error: No training data found!");
        return;
    }

    // Extract character names for neuron mapping
    let character_names: Vec<String> = training_data
        .iter()
        .map(|s| s.character.clone())
        .collect();

    // 2. Create network: one neuron per character class
    let mut brain = Perceptron::new(
        training_data.len(), // Number of output neurons (one per character)
        INPUT_SIZE,          // Input size: bias + 64 pixels
        LEARNING_RATE,
        &character_names,
    );

    // 3. Training loop
    println!("Training Adaline network...");
    println!("Target MSE: {}", TARGET_MSE);
    println!("Max epochs: {}", MAX_EPOCHS);
    println!();

    for epoch in 0..MAX_EPOCHS {
        let mse = brain.train_epoch(&training_data);

        // Print progress periodically or when target is reached
        if epoch % PRINT_INTERVAL == 0 || mse < TARGET_MSE {
            println!("Epoch {:4}: MSE = {:.6}", epoch, mse);
        }

        if mse < TARGET_MSE {
            println!("\nTarget MSE reached at epoch {}!", epoch);
            break;
        }
    }

    // 4. Test the network
    println!("\n--- Test Results ---");
    for item in &test_data {
        let (winner_idx, score) = brain.predict(&item.grid);
        let predicted_name = &training_data[winner_idx].character;

        let is_correct = item.character == *predicted_name;
        let status = if is_correct { "✓" } else { "✗" };

        println!(
            "{} Broken Symbol: {} -> Recognised as: {} (Score: {:.2})",
            status, item.character, predicted_name, score
        );
    }
}
