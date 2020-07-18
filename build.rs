use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use walkdir::WalkDir;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    let executable_path = find_parent_target_dir(&out_dir)
        .expect("failed to find target dir")
        .join(env::var("PROFILE").unwrap());

    recursive_copy(
        &manifest_dir.join("assets"),
        &executable_path.join("assets"),
    );
}

// find the parent directory that is called target to copy files to
fn find_parent_target_dir(mut child: &Path) -> Option<&Path> {
    loop {
        if child.ends_with("target") {
            return Some(child);
        }
        child = match child.parent() {
            Some(path) => path,
            None => break,
        }
    }
    None
}

fn recursive_copy(from: &Path, to: &Path) {
    let from_path: PathBuf = from.into();
    let to_path: PathBuf = to.into();
    for entry in WalkDir::new(from_path.clone()) {
        let entry = entry.unwrap();

        if let Ok(rel_path) = entry.path().strip_prefix(&from_path) {
            let target_path = to_path.join(rel_path);

            if entry.file_type().is_dir() {
                fs::DirBuilder::new()
                    .recursive(true)
                    .create(target_path)
                    .expect("failed to create target dir");
            } else {
                fs::copy(entry.path(), &target_path).expect("failed to copy");
            }
        }
    }
}
