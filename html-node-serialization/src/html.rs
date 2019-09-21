mod serialize;

use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Node {
    Element(Element),
    Text(Text),
}

impl Node {
    fn element(&self) -> Result<&Element, String> {
        match self {
            Node::Element(element) => Ok(element),
            _ => Err("Not an element".to_string())
        }
    }

    fn mut_element(&mut self) -> Result<&mut Element, String> {
        match self {
            Node::Element(element) => Ok(element),
            _ => Err("Not an element".to_string())
        }
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct Element {
    name: String,
    attributes: HashMap<String, String>,
    children: Vec<Node>,
}

impl Element {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn attributes(&self) -> &HashMap<String, String> {
        &self.attributes
    }

    pub fn children(&self) -> &Vec<Node> {
        &self.children
    }

    pub fn id(&self) -> Option<&String> {
        self.attributes.get("id".into())
    }

    pub fn append(&mut self, node: Node) {
        self.children.push(node);
    }
}

#[derive(Debug, PartialEq)]
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
pub fn text(content: impl Into<String>) -> Node {
    Node::Text(Text {
        text: content.into(),
    })
}

#[cfg(test)]
mod test {
    use crate::html;

    macro_rules! hashmap {
        ($( $key: expr => $val: expr ),*) => {{
            let mut map = ::std::collections::HashMap::new();
            $( map.insert($key, $val); )*
            map
        }}
    }

    #[test]
    fn text_equality() {
        assert_eq!(html::text("A"), html::text("A"));
        assert_ne!(html::text("B"), html::text("A"));
    }

    #[test]
    fn append_to_element() -> Result<(), String> {
        let mut node = html::element("html", hashmap![], vec![]);
        node.mut_element()?.append(html::text("hello"));
        assert_eq!(node.element()?.children(), &vec!(html::text("hello")));
        Ok(())
    }
}
