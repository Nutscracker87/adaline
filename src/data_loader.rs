//! Loads 8×8 character grids from text files (`#` / `.`) into `Sample`s.

use std::fs;
use crate::sample::Sample;

/// Grid dimensions for character recognition
const GRID_SIZE: usize = 64;

/// Loads all character samples from a directory
/// 
/// # Arguments
/// * `directory` - Path to directory containing character files
/// 
/// # Returns
/// Vector of samples, sorted by filename
/// 
/// # Panics
/// Panics if the directory cannot be read or files are malformed
pub fn load_dataset(directory: &str) -> Vec<Sample> {
    let mut entries: Vec<_> = fs::read_dir(directory)
        .unwrap_or_else(|e| panic!("Directory '{}' does not exist: {}", directory, e))
        .map(|res| {
            res.map(|e| e.path())
                .map_err(|e| format!("Error reading directory entry: {}", e))
        })
        .collect::<Result<Vec<_>, _>>()
        .unwrap_or_else(|e| panic!("Cannot read directory '{}': {}", directory, e));

    // Sort for reproducible ordering
    entries.sort();

    println!(
        "Found {} files in directory '{}'",
        entries.len(),
        directory
    );

    let mut character_data = Vec::new();
    for path in entries {
        // Extract character name from filename (e.g., "A.txt" -> "A")
        let file_stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or_else(|| panic!("Cannot extract filename from path: {:?}", path));

        // Read file content
        let content = fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("Cannot read file {:?}: {}", path, e));

        // Parse grid: convert '#' and '.' to 1 and 0
        let mut grid_values = [0i32; GRID_SIZE];
        let symbols: Vec<char> = content
            .chars()
            .filter(|ch| *ch == '#' || *ch == '.')
            .take(GRID_SIZE)
            .collect();

        if symbols.len() != GRID_SIZE {
            panic!(
                "File {:?} has {} symbols, expected {}",
                path,
                symbols.len(),
                GRID_SIZE
            );
        }

        for (i, symbol) in symbols.iter().enumerate() {
            grid_values[i] = if *symbol == '#' { 1 } else { 0 };
        }

        character_data.push(Sample::new(file_stem, grid_values));
    }

    character_data
}