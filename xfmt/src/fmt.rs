// Copyright 2024 Dimitrios Papakonstantinou. All rights reserved.
// Use of this source code is governed by a MIT
// license that can be found in the LICENSE file.
//
// The formatter interface used by the cli tool

use std::collections::HashSet;

// A node in the AST
enum Node {
    Text(String),
    SpaceOrLine, // Renders to space if is inside a group, renders to new line if not
    Line,        // Renders to a new line
    Indent(Vec<Node>), // Renders on eore more nodes
    Group(NodeGroup), // A grouping of nodes
    Nodes(Vec<Node>), // A collection of nodes that are render without spacial handling
} //TODO Add IfWrap

enum Wrap {
    Enable,
    Detect,
}

impl Wrap {
    fn enable(&self) -> bool {
        match self {
            Wrap::Enable => true,
            _ => false,
        }
    }
}

impl Node {
    fn width(&mut self, wrapped: &[isize]) -> isize {
        match self {
            Node::Nodes(nodes) => nodes.iter_mut().map(|n| n.width(wrapped)).sum(),
            Node::Group(node_group) => node_group.nodes.iter_mut().map(|n| n.width(wrapped)).sum(),
            Node::Indent(nodes) => nodes.iter_mut().map(|n| n.width(wrapped)).sum(),
            Node::Text(source) => source.len() as isize,
            Node::SpaceOrLine => 1,
            _ => 0,
        }
    }
}

// A grouping of nodes
struct NodeGroup {
    nodes: Vec<Node>, // The nodes inside the group
    id: isize,        // The id of the group
}

// Struct holding formatter meta data
pub struct Generator {
    buffer: String,          // Holds formatted code
    size: usize,             // Number of characters on current line
    indent: usize,           // indentation size
    indent_size: usize,      // Size of an indentation
    max: usize,              // Max numbers per line
    wrapped: HashSet<isize>, // Hash set with all groups that need to be wrapped
}

// Indentation template of 3 spaces
const INDENT: &str = "   ";

impl Generator {
    pub fn new(max: usize) -> Self {
        Self {
            buffer: String::new(),
            size: 0,
            indent: 0,
            indent_size: 3,
            max,
            wrapped: HashSet::new(),
        }
    }

    fn text(&mut self, value: String, char_num: usize) -> () {
        self.size += char_num;
        self.buffer.push_str(&value);
    }

    fn new_line(&mut self) -> () {
        self.size = self.indent_size * self.indent;
        self.buffer.push('\n');

        for _ in 0..self.indent {
            self.buffer.push_str(INDENT);
        }
    }

    fn node(&mut self, node: Node) -> () {}
}
