use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct Node {
    pub name: String,
    pub attrs: HashMap<String, String>,
}
impl Node {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            attrs: HashMap::new(),
        }
    }
    pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
        Self {
            attrs: HashMap::from_iter(
                attrs
                    .iter()
                    .map(|(k, v)| (String::from(*k), String::from(*v))),
            ),
            ..self
        }
    }
    pub fn get_attr(&self, key: &str) -> Option<&str> {
        self.attrs.get(key).map(|x| &**x)
    }
}
