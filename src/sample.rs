//! One labeled character sample: name + 65-dim feature vector (bias + 64 pixels as ±1).

/// Represents a single training/test sample with a character label and pixel grid
pub struct Sample {
    pub character: String,
    pub grid: [f32; 65], // Bias (1.0) + 64 pixels
}

impl Sample {
    /// Creates a new sample from a character name and 64-pixel grid
    /// The grid is converted to f32 with bias prepended (1.0 for bias, 1.0/-1.0 for pixels)
    pub fn new(c: impl Into<String>, grid: [i32; 64]) -> Self {
        let mut f32_grid = [0.0; 65];
        f32_grid[0] = 1.0; // Bias term
        for (i, &val) in grid.iter().enumerate() {
            f32_grid[i + 1] = if val > 0 { 1.0 } else { -1.0 };
        }
        Self {
            character: c.into(),
            grid: f32_grid,
        }
    }
}