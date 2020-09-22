//
//  This Source Code Form is subject to the terms of the Mozilla Public
//  License, v. 2.0. If a copy of the MPL was not distributed with this
//  file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

mod doc;

use std::path::Path;
use std::fs::{self, File};
use std::io::{self, prelude::*, BufReader};

use doc::{Doc, Definition};

use regex::Regex;

static BACH_DIR: &str = "./bach";

fn walk(p: &Path, pattern: Regex) -> Result<Vec<Doc>, Box<dyn std::error::Error>> {
    let paths = fs::read_dir(p)?;
    let mut isdoc = false;
    let mut comments: Vec<Doc> = Vec::new();
    let mut idx = 0usize;

    // init comments vec
    comments.push(Doc::new());

    'outer: for path in paths {
        // shadow path to unwrap and convert to an actual Path
        let path = path?.path();

        if path.is_dir() {
            // If we find a directory, walk that directory recursively and append the result to the
            // comment Vec
            comments.append(&mut walk(&path, pattern.clone())?);
        } else {
            let reader = BufReader::new(File::open(path)?);

            for line in reader.lines() {
                let line = match line {
                    Ok(l) => l,
                    Err(_) => continue 'outer,
                };
                if pattern.is_match(line.as_str()) {
                    if comments.len() <= idx {
                        comments.push(Doc::new());
                    }
                    comments[idx].push(line);
                    isdoc = true;
                } else {
                    if isdoc {
                        match Definition::derive(line) {
                            // if we match, set that to the definition
                            Some(d) => {
                                comments[idx].set_def(d);
                                isdoc = false;
                                idx += 1;
                            },
                            // if not, just continue
                            None => ()
                        }
                    }
                }
            }
        }
    }

    Ok(comments)
}

fn main() -> std::io::Result<()> {
    // Create ./bach if it does not exist
    if !Path::new(BACH_DIR).exists() {
        std::fs::create_dir(BACH_DIR)?;
    }

    // Match doc comments
    let pattern: Regex = Regex::new(r"(?i)^\s*///.*").unwrap();
    let cwd = Path::new("./");
    let comments = walk(cwd, pattern).unwrap();

    println!("{:#?}", comments);
    Ok(())
}
