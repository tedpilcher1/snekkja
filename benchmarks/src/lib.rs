use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use snekkja::Parser;

pub fn load_samples(filename: &str) -> Vec<Vec<u8>> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("samples")
        .join(filename);

    let file =
        File::open(&path).unwrap_or_else(|e| panic!("failed to open {}: {}", path.display(), e));

    BufReader::new(file)
        .lines()
        .map(|l| l.unwrap().into_bytes())
        .collect()
}

pub fn parse_lines(parser: &mut Parser, lines: &[Vec<u8>]) {
    for line in lines {
        let _ = parser.parse(line).unwrap();
    }
}
