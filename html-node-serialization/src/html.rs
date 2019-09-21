mod serialize;

use std::collections::HashMap;

#[derive(Debug)]
pub enum Node {
    Element(Element),
    Text(Text),
}

#[derive(Debug, Default)]
pub struct Element {
    name: String,
    attributes: HashMap<String, String>,
    children: Vec<Node>,
}

#[derive(Debug)]
pub struct Text {
    text: String,
}

#[allow(dead_code)]
pub fn element(
    name: impl Into<String>,
    attributes: HashMap<String, String>,
    children: Vec<Node>,
) -> Node {
    Node::Element(Element {
        name: name.into(),
        attributes,
        children,
    })
}

#[allow(dead_code)]
pub fn text(string: impl Into<String>) -> Node {
    Node::Text(Text {
        text: string.into(),
    })
}
