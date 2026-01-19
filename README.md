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

Place your test files in the `dataset/test/` directory with the same format. Test files can have any name (e.g., `A_broken.txt`, `test_B.txt`), but the first part of the filename (before the first underscore or dot) should indicate the expected character.

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

You can add multiple training files for the same character by using different filenames:
- `A.txt`, `A_variant1.txt`, `A_variant2.txt` - All will train the 'A' neuron

**Note:** The program uses the filename stem (part before first `.` or `_`) to identify the character class.

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
