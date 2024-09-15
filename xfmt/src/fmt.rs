// Copyright 2024 Dimitrios Papakonstantinou. All rights reserved.
// Use of this source code is governed by a MIT
// license that can be found in the LICENSE file.
//
// The formatter interface used by the cli tool

// A node in the AST
enum Node {
    Text(String),
    SpaceOrLine, // Renders to space if is inside a group, renders to new line if not
    Line,        // Renders to a new line
    Indent(Vec<Node>), // Renders on eore more nodes
    Group(NodeGroup), // A grouping of nodes
    Nodes(Vec<Node>), // A collection of nodes that are render without spacial handling
}

// A grouping of nodes
struct NodeGroup {
    nodes: Vec<Node>, // The nodes inside the group
    id: usize,        // The id of the group
}

pub struct fmt {
    id_counter: usize, // Variable for keeping track of the next node
}

impl fmt {
    pub fn new() -> Self {
        Self { id_counter: 0 }
    }
}
