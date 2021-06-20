use std::{fs, path::PathBuf};

pub fn read_dir(path: &str) -> Vec<PathBuf> {
    let entries = fs::read_dir(path);

    let mut paths: Vec<PathBuf> = Vec::new();

    for e in entries {
        for ex in e {
            for exx in ex {
                let path = exx.path();
                paths.push(path);
            }
        }
    }

    paths
}
