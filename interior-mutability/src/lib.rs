use std::collections::HashMap;

#[derive(Default)]
struct Colours {
    colours: HashMap<String, String>
}

impl Colours {
    fn new() -> Colours {
        let colours: HashMap<String, String> = HashMap::new();
        Colours { colours }
    }

    fn get(&self, name: &str) -> Option<&String> {
        self.colours.get(name)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let colours = Colours::new();
        colours.get("hello");
    }
}
