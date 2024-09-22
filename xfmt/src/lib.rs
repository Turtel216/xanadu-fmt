// Copyright 2024 Dimitrios Papakonstantinou. All rights reserved.
// Use of this source code is governed by a MIT
// license that can be found in the LICENSE file.

use std::{
    fs::{self, File},
    io::Write,
    process,
};

use fmt::Formatter;

mod builder;
pub mod fmt;
mod tokenizer;

pub fn format_file(path: &String) -> () {
    let code = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(error) => {
            eprint!("Unable to read file {}: {}", path, error);
            process::exit(74);
        }
    };

    let mut formatter = Formatter::new(&code);
    let output = formatter.format();
    write_to_file(path, &output);
}

fn write_to_file(path: &String, code: &String) -> () {
    let mut file = match File::open(path) {
        Ok(s) => s,
        Err(e) => {
            eprint!("Unable to write to file {}: {}", path, e);
            process::exit(74);
        }
    };

    match file.write_all(code.as_bytes()) {
        Ok(_) => (),
        Err(e) => {
            eprint!("Unable to read file {}: {}", path, e);
            process::exit(74);
        }
    }
}
