// Copyright 2024 Dimitrios Papakonstantinou. All rights reserved.
// Use of this source code is governed by a MIT
// license that can be found in the LICENSE file.

struct Builder {
    id: isize,
}

use crate::fmt::Node;
use crate::fmt::NodeGroup;

impl Builder {
    pub fn new() -> Self {
        Builder { id: 0 }
    }

    fn new_id(&mut self) -> isize {
        self.id += 1;
        self.id
    }

    fn string(&mut self, value: String) -> Node {
        Node::Group(NodeGroup {
            id: self.new_id(),
            nodes: vec![
                Node::Text('"'.to_string()),
                Node::Text(value),
                Node::Text('"'.to_string()),
            ],
        })
    }

    fn call(&mut self, name: String, argumments: Vec<Node>) -> Node {
        let id = self.new_id();

        if argumments.is_empty() {
            return Node::Group(NodeGroup {
                id,
                nodes: vec![Node::Text(name), Node::Text("()".to_string())],
            });
        }

        let max = argumments.len() - 1;
        let vals = argumments
            .iter()
            .enumerate()
            .map(|(index, node)| match (index, node) {
                (index, node) => {
                    if index < max {
                        Node::Nodes(vec![node, Node::Text(','.to_string()), Node::SpaceOrLine])
                    } else {
                        Node::Nodes(vec![
                            node,
                            Node::IfWrap(id, Node::Text(','.to_string())),
                            Node::Text("".to_string()),
                        ])
                    }
                }
            })
            .collect();

        Node::Group(NodeGroup {
            id,
            nodes: vec![
                Node::Text('('.to_string()),
                Node::Line,
                Node::Indent(vals),
                Node::Line,
                Node::Text(')'.to_string()),
            ],
        })
    }
}
