pub mod graph {
    use std::collections::HashMap;

    #[derive(Default)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph::default()
        }

        pub fn with_nodes(mut self, nodes: &Vec<Node>) -> Self {
            self.nodes = nodes.to_owned();
            self
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            self.attrs = attrs
                .iter()
                .map(|&(key, value)| (key.to_owned(), value.to_owned()))
                .collect();
            self
        }

        pub fn with_edges(mut self, edges: &Vec<Edge>) -> Self {
            self.edges = edges.to_owned();
            self
        }

        pub fn node(&self, name: &str) -> Option<Node> {
            self.nodes.iter().find(|&n| n.name == name).cloned()
        }
    }

    #[derive(Clone, PartialEq, Debug)]
    pub struct Node {
        name: String,
        attrs: HashMap<String, String>,
    }

    impl Node {
        pub fn new(name: &str) -> Self {
            Self {
                name: name.to_owned(),
                attrs: HashMap::new(),
            }
        }
        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            self.attrs = attrs
                .iter()
                .map(|&(key, value)| (key.to_owned(), value.to_owned()))
                .collect();
            self
        }
        pub fn attr(&self, key: &str) -> Option<&str> {
            self.attrs
                .iter()
                .find_map(|(k, v)| if k == key { Some(v.as_str()) } else { None })
        }
    }

    #[derive(Clone, PartialEq, Debug)]
    pub struct Edge {
        nodes: (String, String),
        attrs: HashMap<String, String>,
    }

    impl Edge {
        pub fn new(node_a: &str, node_b: &str) -> Self {
            Self {
                nodes: (node_a.to_owned(), node_b.to_owned()),
                attrs: HashMap::new(),
            }
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            self.attrs = attrs
                .iter()
                .map(|&(key, value)| (key.to_owned(), value.to_owned()))
                .collect();
            self
        }

        pub fn attr(&self, key: &str) -> Option<&str> {
            self.attrs
                .iter()
                .find_map(|(k, v)| if k == key { Some(v.as_str()) } else { None })
        }
    }

    #[derive(Clone, PartialEq, Debug)]
    pub struct Attr {
        key: String,
        value: String,
    }
}
