# Adaline Neural Network

An implementation of the Adaline (Adaptive Linear Neuron) neural network in Rust for character recognition. This project demonstrates the LMS (Least Mean Squares) algorithm for training a multi-class classifier.

## Overview

The Adaline network learns to recognize characters from 8x8 pixel grids. Each character is represented as a 64-pixel image where `#` represents a filled pixel and `.` represents an empty pixel.

## Requirements

- Rust (edition 2021 or later)
- Cargo (comes with Rust)

## How to Run

1. **Clone the repository:**
   ```bash
   git clone https://github.com/Nutscracker87/adaline.git
   cd adaline
   ```

2. **Build and run:**
   ```bash
   cargo run
   ```

   Or build first, then run:
   ```bash
   cargo build --release
   ./target/release/adaline
   ```

3. **The program will:**
   - Load training data from `dataset/training/`
   - Train the neural network until convergence or max epochs
   - Test the network on data from `dataset/test/`
   - Display predictions with accuracy indicators

## Dataset Format

### Training Data

Place your training files in the `dataset/training/` directory. Each file should:
- Be named after the character it represents (e.g., `A.txt`, `B.txt`, `C.txt`)
- Contain an 8x8 grid of characters
- Use `#` for filled pixels and `.` for empty pixels
- Have exactly 64 pixels (8 rows × 8 columns)

**Example format (`A.txt`):**
```
..####..
.##..##.
#.....##
#.######
##....##
##....##
##....##
........
```

### Test Data

Place your test files in the `dataset/test/` directory with the same format. The **character label** used for comparison is the filename without extension (the “stem”). For example, `A_broken.txt` has stem `A_broken`; the program will expect that file to be recognized as the class `A_broken`. To test against the same class as training, use a stem that matches a training file (e.g. `A.txt` and `A_noisy.txt` both have stems `A` and `A_noisy` — only `A.txt` matches the training class `A`). For a test of the “A” class, name the file so the stem is exactly `A` (e.g. `A.txt` or `A` with no extension).

## Adding New Datasets

### Adding a New Character Class

1. Create a new training file in `dataset/training/`:
   ```bash
   # Example: Add character 'Z'
   touch dataset/training/Z.txt
   ```

2. Edit the file with your 8x8 grid:
   ```
   ########
   .....##.
   ....##..
   ...##...
   ..##....
   .##.....
   ########
   ........
   ```

3. The program will automatically detect the new character and create a corresponding output neuron.

### Adding More Training Samples

The program uses the **filename stem** (filename without extension) as the character class. So `A.txt` → class `A`, and `A_variant1.txt` → class `A_variant1` (a different class). To add more training samples for the same character, keep the same stem and use a different extension, e.g.:
- `A.txt`, `A.1`, `A.2` — all train the same `A` neuron (stems are `A`, `A`, `A`).

Names like `A_variant1.txt` create a separate class `A_variant1`, not extra samples for `A`.

### Adding Test Cases

1. Create test files in `dataset/test/`:
   ```bash
   touch dataset/test/Z_test.txt
   ```

2. Use the same 8x8 grid format as training data

3. The program will test these and show predictions

## Project Structure

```
adaline/
├── src/
│   ├── main.rs          # Main program entry point
│   ├── adaline_nn.rs    # Neural network implementation
│   ├── data_loader.rs   # Dataset loading logic
│   └── sample.rs        # Sample data structure
├── dataset/
│   ├── training/        # Training character files
│   └── test/            # Test character files
└── Cargo.toml           # Rust project configuration
```

## Algorithm Details

- **Network Type:** Adaline (Adaptive Linear Neuron)
- **Learning Algorithm:** LMS (Least Mean Squares)
- **Input:** 65 features (1 bias + 64 pixels)
- **Output:** One neuron per character class
- **Activation:** Linear (no activation function)
- **Target Values:** +1.0 for correct class, -1.0 for others

## License

This is a learning project. Feel free to use and modify as needed.
