// Copyright 2024 Dimitrios Papakonstantinou. All rights reserved.
// Use of this source code is governed by a MIT
// license that can be found in the LICENSE file.

use xfmt::fmt::Formatter;

#[test]
fn test_formatting() {
    let input = String::from("pink x =1+2 ; overtune{ something , other }");
    let expected_output = String::from("pink x = 1 + 2;\novertune {\n   something, other \n}");
    let mut formatter = Formatter::new(&input);
    let output = formatter.format();

    println!("Output string: {}", output);

    assert_eq!(output, expected_output);
}
