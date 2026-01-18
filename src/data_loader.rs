use std::fs;
use std::io;
use crate::sample::Sample;

pub fn load_dataset(directory: &str) -> Vec<Sample> {
    let mut entries = fs::read_dir(directory)
        .expect("The dir: {directory} is not exists")
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()
        .expect("Cant read directory");
    // The order in which `read_dir` returns entries is not guaranteed. If reproducible
    // ordering is required the entries should be explicitly sorted.
    entries.sort();

    println!(
        "Found {} datasets in the directory {}",
        entries.len(),
        directory
    );

    let mut character_data = Vec::new();
    for path in entries {
        // 1. Get the character name (e.g., 'A')
        let file_stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .expect("Cant read file name");
        // let c = file_stem.chars().next().unwrap();

        // 2. Read file content
        let content = fs::read_to_string(&path)
            .expect(&format!("Could not read file {:?} content", path.to_str()));

        // 3. Convert symbols (# and .) into i32s
        let mut grid_values = [0i32; 64];

        // Filter out newlines/whitespace so we only look at the dots and hashes
        let symbols = content.chars().filter(|ch| *ch == '#' || *ch == '.');

        for (i, symbol) in symbols.enumerate() {
            if i < 64 {
                grid_values[i] = if symbol == '#' { 1 } else { 0 };
            }
        }
        // 4. Create and store
        character_data.push(Sample::new(file_stem, grid_values));
    }

    character_data
}