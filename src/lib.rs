use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
struct Tree {
    name: String,
    children: HashMap<String, Tree>,
}

impl Tree {
    fn default() -> Self {
        Tree::new("root".to_string())
    }

    fn new(name: String) -> Tree {
        Tree {
            name,
            children: HashMap::default(),
        }
    }

    fn add_phrase(&mut self, phrase: Vec<String>) {
        let mut node = self;
        for subkey in phrase {
            node = node
                .children
                .entry(subkey.to_string())
                .or_insert_with(|| Tree::new(subkey));
        }
    }

    fn children_names(&self) -> Vec<String> {
        self.children.keys().cloned().collect()
    }

    fn step_down(&self, name: String) -> Option<&Self> {
        self.children.get(&name)
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::Tree;

    #[test]
    fn it_defaults() {
        let result = Tree::default();
        assert_eq!(result.name, "root".to_string());
        assert_eq!(result.children.len(), 0);
    }

    #[test]
    fn it_is_new() {
        let result = Tree::new("Gerald".to_string());
        assert_eq!(result.name, "Gerald".to_string());
        assert_eq!(result.children.len(), 0);
    }

    #[test]
    fn it_ingests_a_phrase_of_length_one() {
        let mut result = Tree::default();
        result.add_phrase(vec!["a".to_string()]);
        assert_eq!(result.name, "root".to_string());
        assert_eq!(result.children.len(), 1);
    }

    #[test]
    fn it_ingests_a_phrase_of_length_two() {
        let mut result = Tree::default();
        result.add_phrase(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(result.name, "root".to_string());
        assert_eq!(result.children.len(), 1);
    }

    #[test]
    fn it_steps_down_a_word_that_is_there() {
        let mut result = Tree::default();
        result.add_phrase(vec!["a".to_string(), "b".to_string()]);
        let last = result.step_down("a".to_string());
        assert_eq!(last.unwrap().children_names(), vec!["b".to_string()]);
    }

    #[test]
    fn it_steps_down_a_word_that_is_not_there() {
        let mut result = Tree::default();
        result.add_phrase(vec!["a".to_string(), "b".to_string()]);
        let last = result.step_down("c".to_string());
        assert_eq!(last, None);
    }

    #[test]
    fn it_exposess_children_names() {
        let mut result = Tree::default();
        result.add_phrase(vec!["a".to_string()]);
        assert_eq!(result.children_names(), vec!["a".to_string()]);
    }

    #[test]
    fn it_ingests_multiple_phrases() {
        let mut result = Tree::default();
        result.add_phrase(vec!["a".to_string(), "b".to_string()]);
        result.add_phrase(vec!["a".to_string(), "c".to_string()]);
        assert_eq!(result.name, "root".to_string());
        assert_eq!(result.children_names(), vec!["a".to_string()]);
        let a_tree = result.step_down("a".to_string()).unwrap();
        assert_eq!(a_tree.children_names().len(), 2);
        assert!(a_tree.children_names().contains(&"b".to_string()));
        assert!(a_tree.children_names().contains(&"c".to_string()));
    }
}
