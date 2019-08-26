use glob::glob;
use regex::Regex;
use std::collections::VecDeque;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::{Path, PathBuf};

fn main() {
    let path = format!("{}/{}", env!("CARGO_MANIFEST_DIR"), "temp");
    let dir = Path::new(&path);
    if dir.exists() {
        std::fs::remove_dir_all(dir).expect("failed to remove temp dir");
    }
    std::fs::create_dir(dir).expect("Failed to create temp dir");
    let files = find_files();
    let temp_docs = copy_to_temp(&files, dir);
    skeptic::generate_doc_tests(&temp_docs[..]);
}

fn copy_to_temp<T>(paths: &[T], dir: &Path) -> Vec<PathBuf>
where
    T: AsRef<Path>,
    T: Clone,
{
    paths
        .into_iter()
        .map(|path| {
            let file_name = path
                .as_ref()
                .file_name()
                .expect("Bad file name")
                .to_str()
                .expect("Failed to pass name");
            let raw_template_path = find_template(file_name).unwrap_or_else(|| {
                PathBuf::from(format!(
                    "{}/{}",
                    env!("CARGO_MANIFEST_DIR"),
                    "boilerplate/default.md"
                ))
            });
            let template_name = format!("{}{}", file_name, ".skt.md");

            let mut raw_doc = File::open(path).expect("Failed to open doc");
            let mut raw_template =
                File::open(raw_template_path.clone()).expect("Failed to open template");

            let mut buffer = Vec::new();
            raw_doc.read_to_end(&mut buffer).unwrap();
            let mut doc_buffer =
                String::from_utf8(buffer).expect("Failed to parse contents of doc");
            let mut buffer = Vec::new();
            raw_template
                .read_to_end(&mut buffer)
                .expect("Failed to read template");
            let template_buffer =
                String::from_utf8(buffer).expect("Failed to parse contents of template");

            let file_path = dir.join(file_name);
            let template_path = dir.join(template_name);

            let mut d = File::create(&file_path).expect("Failed to open temp doc");
            let mut t = File::create(&template_path).expect("Failed to open temp template");

            add_tags(&mut doc_buffer, &template_buffer);

            write!(&mut d, "{}", doc_buffer).expect("Failed to write");
            write!(&mut t, "{}", template_buffer).expect("Failed to write");

            file_path
        })
        .collect()
}

fn add_tags(doc_buffer: &mut String, template_buffer: &String) {
    let re = Regex::new(r"```rust.+").expect("Failed to create regex");
    let mut captures: VecDeque<String> = re
        .captures_iter(template_buffer)
        .flat_map(|cap: regex::Captures| {
            cap.iter()
                .flat_map(|m| m.map(|m| m.as_str().to_owned()))
                .collect::<Vec<_>>()
        })
        .collect();
    *doc_buffer = doc_buffer
        .lines()
        .map(|line| {
            if line.contains("```rust") {
                let c = captures.pop_front();
                match c {
                    Some(c) => c,
                    None => line.to_string(),
                }
            } else {
                line.to_string()
            }
        })
        .map(|line| format!("{}\n", line))
        .collect();
}

fn find_files() -> Vec<PathBuf> {
    glob("../docs/**/*.md")
        .expect("Failed to read glob pattern")
        .filter_map(Result::ok)
        .collect()
}

fn find_template(file_name: &str) -> Option<PathBuf> {
    let template_name = format!("{}{}", file_name, ".skt.md");
    let mut template_path =
        PathBuf::from(format!("{}/{}", env!("CARGO_MANIFEST_DIR"), "boilerplate"));
    template_path.push(&template_name);
    if template_path.exists() {
        Some(template_path)
    } else {
        None
    }
}
