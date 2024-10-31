// Copyright 2024 Dimitrios Papakonstantinou. All rights reserved.
// Use of this source code is governed by a MIT
// license that can be found in the LICENSE file.

use xfmt::fmt::Formatter;

#[test]
fn test_formatting() {
    let input = String::from("pink x =1+2 ; overtune (a,b){ something , other }\0");
    let expected_output =
        String::from("pink x = 1 + 2;\novertune(a, b) {\n   something, other \n}");
    let mut formatter = Formatter::new(&input);
    let output = formatter.format();

    println!("Output string: {}", output);

    assert_eq!(output, expected_output);
}

#[test]
fn test_wrapping() {
    let input = String::from("name name name name name name name name name\0");
    let expected_output = String::from("name name name name name name name name \n   name ");
    let mut formatter = Formatter::new(&input);
    let output = formatter.format();

    println!("Output string: {}", output);

    assert_eq!(output, expected_output);
}
