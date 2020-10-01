//
//  This Source Code Form is subject to the terms of the Mozilla Public
//  License, v. 2.0. If a copy of the MPL was not distributed with this
//  file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

mod doc;
mod gen;

use std::path::Path;
use std::fs::{self, File};
use std::io::{prelude::*, BufReader};

use doc::{Doc, Definition};
use gen::{Generator, Theme};

use regex::Regex;

// Find a specific type of documentation out of a Vec<Doc>
#[macro_export]
macro_rules! find {
    ( $x:ident => classes ) => {
        // TODO(@monarrk): Is it possible to use into_iter() here to we don't need to return a
        // reference?
        $x.iter().filter(|d| d.is_class()).collect::<Vec<&Doc>>()
    };
}

// Where to output
static BACH_DIR: &str = "./bach";

// Walk through every directory and scan files
fn walk(p: &Path, pattern: Regex) -> Result<Vec<Doc>, Box<dyn std::error::Error>> {
    let paths = fs::read_dir(p)?;

    // TODO(@monarrk): Remove global muts?
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
                // Unwrap line safely
                let line = match line {
                    Ok(l) => l,
                    Err(_) => continue 'outer,
                };

                // is `line` a doc comment?
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
    let docs = walk(cwd, pattern).unwrap();

    // Get classes out of the docs
    let classes = find!(docs => classes);
    let mut generator = Generator::new(std::env::current_dir()?.to_str().unwrap().to_string(), classes, Theme::Default);
    let out = generator.generate();

    // Create the output file
    let mut file = match File::create(&format!("{}/index.html", BACH_DIR)) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Couldn't open file for writing: {}", e);
            std::process::exit(1);
        }
    };
    
    // Write the output file
    match file.write_all(out.as_bytes()) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Couldn't write to file: {}", e);
            std::process::exit(1);
        }
    };

    println!("Done!");

    Ok(())
}
