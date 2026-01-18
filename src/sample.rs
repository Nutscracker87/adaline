pub struct Sample {
    pub character: String,
    pub grid: [f32; 65],
}

impl Sample {
    pub fn new(c: impl Into<String>, grid: [i32; 64]) -> Self {
        let mut f32_grid = [0.0; 65];
        f32_grid[0] = 1.0;
        for (i, &val) in grid.iter().enumerate() {
            f32_grid[i + 1] = if val > 0 { 1.0 } else { -1.0 };
        }
        Self {
            character: c.into(),
            grid: f32_grid,
        }
    }
}