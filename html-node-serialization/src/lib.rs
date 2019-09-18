extern crate serde;
extern crate serde_json;

use std::collections::HashMap;

use serde::{ Serialize, Serializer };
use serde::ser::SerializeMap;

#[derive(Debug, Default)]
struct Element {
    name: String,
    attributes: HashMap<String, String>,
    children: Vec<Node>
}

impl serde::Serialize for Element {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        {
            let mut map = serializer.serialize_map(Some(1))?;
            let data: HashMap<String, String> = HashMap::new();
            map.serialize_entry(&self.name, &data)?;
            map.end()
        }
}

#[derive(Debug)]
enum Node {
    Element(Element),
    Text(String)
}

impl serde::Serialize for Node {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Node::Element(element) => element.serialize(serializer),
            _                      => serializer.serialize_i32(123)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_serialization() {
        let dom = Node::Element(Element { name: "html".to_string(), ..Default::default() });
        let json = serde_json::to_string(&dom).unwrap();
        assert_eq!(r#"{"html":{}}"#, json);
    }
}
