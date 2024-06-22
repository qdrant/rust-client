use std::fs;
use std::path::{Path, PathBuf};

fn generate_snippet_test(snippet: &str, name: &str, dir: &Path) {
    // Pad each line of the snippet with 8 spaces

    let snippet = snippet
        .lines()
        .map(|line| format!("        {}", line))
        .collect::<Vec<_>>()
        .join("\n");

    let test_snippet = format!(
        r#"
#[tokio::test]
async fn test_{name}() {{
    async fn {name}() -> Result<(), Box<dyn std::error::Error>> {{
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/{name}.rs` file
{snippet}
        Ok(())
    }}
    let _ = {name}().await;
}}
"#
    );

    // Write the test snippet to the file
    let mut test_snippet_path = PathBuf::from(dir);
    test_snippet_path.push(format!("test_{}.rs", name));
    fs::write(test_snippet_path, test_snippet).unwrap();
}

fn main() {
    println!("cargo:rerun-if-changed=tests/snippets");

    // Open all files in the `./tests/snippets` directory and save them to the `./tests/snippets_converted` directory
    // Wrap text in the `tests/test_snippets.rs` file into { }
    let snippets_dir = Path::new("./tests/snippets");
    let tests_output_dir = Path::new("./tests/snippet_tests");

    // Create the converted directory if it doesn't exist
    if !tests_output_dir.exists() {
        fs::create_dir_all(tests_output_dir).unwrap();
    }

    let mut snippet_names: Vec<_> = fs::read_dir(snippets_dir)
        .unwrap()
        .filter_map(|entry| {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() {
                let snippet_name = path.file_stem().unwrap().to_str().unwrap().to_string();
                Some((path, snippet_name))
            } else {
                None
            }
        })
        .collect();

    snippet_names.sort_unstable_by_key(|(_, name)| name.clone());

    for (path, name) in &snippet_names {
        let content = fs::read_to_string(path).unwrap();
        generate_snippet_test(&content, name, tests_output_dir);
    }

    // Generate `tests/snippet_tests/mod.rs` file
    // For each file in `./tests/snippet_tests` directory, generate a line `mode {file_name};`

    let mod_file = snippet_names
        .iter()
        .map(|(_, name)| format!("mod test_{};", name))
        .collect::<Vec<_>>()
        .join("\n");

    let mut mod_file_path = PathBuf::from(tests_output_dir);
    mod_file_path.push("mod.rs");
    fs::write(mod_file_path, mod_file).unwrap();
}
