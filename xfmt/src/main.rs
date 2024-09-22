// Copyright 2024 Dimitrios Papakonstantinou. All rights reserved.
// Use of this source code is governed by a MIT
// license that can be found in the LICENSE file.

use std::env;

use xfmt::format_file;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Eror");
    } else if args.len() == 2 {
        format_file(&args[1]);
    } else {
        println!("Usage: xfmt [path]");
    }
}
