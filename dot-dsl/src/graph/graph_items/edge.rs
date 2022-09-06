use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct Edge {
    pub from: String,
    pub to: String,
    pub attrs: HashMap<String, String>,
}
impl Edge {
    pub fn new(from: &str, to: &str) -> Self {
        Self {
            from: String::from(from),
            to: String::from(to),
            attrs: HashMap::new(),
        }
    }
    pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
        let mut acc = HashMap::new();
        for (k, v) in attrs {
            acc.insert(String::from(*k), String::from(*v));
        }
        Self { attrs: acc, ..self }
    }
}
