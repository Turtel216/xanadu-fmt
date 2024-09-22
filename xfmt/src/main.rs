// Copyright 2024 Dimitrios Papakonstantinou. All rights reserved.
// Use of this source code is governed by a MIT
// license that can be found in the LICENSE file.

use std::env;

use xfmt::format_file;

fn main() {
    let args: Vec<String> = env::args().collect();
    format_file(&args[1]);
}
