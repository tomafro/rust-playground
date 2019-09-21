use serde::ser::SerializeMap;
use serde::{Serialize, Serializer};

use super::*;

impl Serialize for Node {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Node::Element(element) => element.serialize(serializer),
            Node::Text(text) => text.serialize(serializer),
        }
    }
}

struct AttributesAndChildren<'a> {
    attributes: &'a HashMap<String, String>,
    children: &'a Vec<Node>,
}

impl<'a> Serialize for AttributesAndChildren<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.attributes.len() + 1))?;

        for (attribute, value) in self.attributes {
            map.serialize_entry(&format!("@{}", attribute), value)?;
        }

        if self.children.len() > 0 {
            map.serialize_entry("children", self.children)?;
        }

        map.end()
    }
}

impl Serialize for Element {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(1))?;
        let attributes_and_children = AttributesAndChildren {
            attributes: &self.attributes,
            children: &self.children,
        };
        map.serialize_entry(&self.name, &attributes_and_children)?;
        map.end()
    }
}

impl Serialize for Text {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry("text!", &self.text)?;
        map.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! hashmap {
        ($( $key: expr => $val: expr ),*) => {{
            let mut map = ::std::collections::HashMap::new();
            $( map.insert($key, $val); )*
            map
        }}
    }

    #[test]
    fn serialize_element() {
        let attributes = hashmap!["class".to_string() => "playground".to_string()];
        let children: Vec<Node> = vec![text("Hello World")];
        let node = element("html", attributes, children);
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            r#"{"html":{"@class":"playground","children":[{"text!":"Hello World"}]}}"#,
            json
        );
    }

    #[test]
    fn serialize_text() {
        let node = text("Hello World!");
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(r#"{"text!":"Hello World!"}"#, json);
    }
}
