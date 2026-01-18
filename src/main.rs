mod adaline_nn;
mod data_loader;
mod sample;

use adaline_nn::Perceptron;

fn main() {
    // 1. loading data from folder
    let training_data = data_loader::load_dataset("dataset/training");
    let test_data = data_loader::load_dataset("dataset/test");

    // 2. We create a network for the number of letters that were actually found in the folder
    let mut brain = Perceptron::new(
        training_data.len(), // how many samples
        65,                  // Bias + 64 pixels
        0.01,                // Learning rate
    );

    // 3. Learning loop
    let target_mse = 1e-6;
    println!("Teaching nn...");
    for epoch in 0..5000 {
        let mse = brain.train_epoch(&training_data);
        if epoch % 500 == 0 || mse < target_mse{
            println!("Epoch {}: MSE = {}", epoch, mse);
        }
        if mse < target_mse {
            break;
        }
    }

    // 4. Testing nn
    println!("\n--- Test results ---");
    for item in &test_data {
        let (winner_idx, score) = brain.predict(&item.grid);
        let predicted_name = &training_data[winner_idx].character;

        println!(
            "File: {} -> NN says: {} (Score: {:.2})",
            item.character, predicted_name, score
        );
    }
}
