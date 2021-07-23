use std::path::PathBuf;

use asefile::AsepriteFile;

use crate::processing::{self, ResourceData};

fn test_path(name: &str) -> PathBuf {
    let mut path = PathBuf::new();
    path.push("tests");
    path.push("data");
    path.push(format!("{}.aseprite", name));
    path
}

fn load_test_file(path: &PathBuf) -> AsepriteFile {
    println!("Loading file: {}", path.display());
    AsepriteFile::read_file(path).unwrap()
}

fn load_test_file_as_assets(name: &str) -> ResourceData {
    let path = test_path(name);
    let ase = load_test_file(&path);
    let mut inputs = Vec::new();
    inputs.push((path, ase));
    processing::ResourceData::new(inputs)
}

#[test]
fn tileset_file() {
    let assets = load_test_file_as_assets("tileset");
    let tilesets = assets.tilesets.0;
    assert_eq!(tilesets.len(), 1);
}
