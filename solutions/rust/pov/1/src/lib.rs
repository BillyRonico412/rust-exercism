use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Debug,
    rc::Rc,
};

type Nodes<T> = BTreeMap<Rc<T>, Node<T>>;

#[derive(Debug, PartialEq, Eq)]
pub struct Tree<T>
where
    T: Debug + Eq + Ord,
{
    root: Rc<T>,
    nodes: Nodes<T>,
}

#[derive(Debug)]
struct Node<T: Debug + Eq + Ord> {
    label: Rc<T>,
    children: BTreeSet<Rc<T>>,
    parent: Option<Rc<T>>,
}

impl<T: Debug + Ord + Eq> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label
    }
}

impl<T: Debug + Ord + Eq> Eq for Node<T> {}

impl<T: Debug + Ord + Eq> PartialOrd for Node<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.label.partial_cmp(&other.label)
    }
}

impl<T: Debug + Ord + Eq> Ord for Node<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.label.cmp(&other.label)
    }
}

impl<T: Debug + Eq + Ord> Tree<T> {
    pub fn new(label: T) -> Self {
        let label = Rc::new(label);

        let node = Node {
            label: label.clone(),
            children: BTreeSet::new(),
            parent: None,
        };

        let mut nodes = BTreeMap::new();
        nodes.insert(label.clone(), node);
        Self {
            root: label.clone(),
            nodes,
        }
    }

    fn add_child(nodes: &mut Nodes<T>, parent: Rc<T>, child: Rc<T>) {
        nodes
            .entry(parent.clone())
            .or_insert(Node {
                label: parent.clone(),
                children: BTreeSet::new(),
                parent: None,
            })
            .children
            .insert(child.clone());
        nodes
            .entry(child.clone())
            .or_insert(Node {
                label: child.clone(),
                children: BTreeSet::new(),
                parent: None,
            })
            .parent = Some(parent.clone())
    }

    pub fn with_child(mut self, child: Self) -> Self {
        self.nodes.extend(child.nodes);
        Self::add_child(&mut self.nodes, self.root.clone(), child.root.clone());
        self
    }

    pub fn pov_from(&mut self, from: &T) -> bool {
        let Some(old_from) = self.nodes.get(from) else {
            return false;
        };
        let mut new_nodes: Nodes<T> = BTreeMap::new();
        new_nodes.insert(
            old_from.label.clone(),
            Node {
                label: old_from.label.clone(),
                children: BTreeSet::new(),
                parent: None,
            },
        );
        let result = self.pov_from_rec(old_from.label.clone(), &mut new_nodes);
        if !result {
            return false;
        }
        self.root = old_from.label.clone();
        self.nodes = new_nodes;
        true
    }

    fn pov_from_rec(&self, from: Rc<T>, new_nodes: &mut Nodes<T>) -> bool {
        let Some(old_from) = self.nodes.get(&from) else {
            return false;
        };
        old_from
            .children
            .iter()
            .chain(old_from.parent.iter())
            .for_each(|node| {
                if !new_nodes.contains_key(node) {
                    Self::add_child(new_nodes, old_from.label.clone(), node.clone());
                    self.pov_from_rec(node.clone(), new_nodes);
                }
            });
        true
    }

    pub fn path_between<'a>(&'a mut self, from: &'a T, to: &'a T) -> Option<Vec<&'a T>> {
        if !self.pov_from(from) {
            return None;
        }
        let mut path = Vec::new();
        self.path_between_rec(&self.root, to, &mut path)
            .then_some(path)
    }

    fn path_between_rec<'a>(&'a self, current: &'a Rc<T>, to: &T, path: &mut Vec<&'a T>) -> bool {
        path.push(current);
        if **current == *to {
            return true;
        }
        let node = &self.nodes[current];
        for child in &node.children {
            if self.path_between_rec(child, to, path) {
                return true;
            }
        }
        path.pop();
        false
    }
}
