mod tree {
    use crate::string_handlers::string_handlers::split_corpus;
    use crate::string_handlers::string_handlers::split_sentence;
    use crate::string_handlers::string_handlers::suffixes;
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

        fn get_depth(&self) -> usize {
            if self.children_names().len() == 0 {
                0
            } else {
                1 + self
                    .children
                    .values()
                    .map(|vt| vt.get_depth())
                    .max()
                    .unwrap_or_default()
            }
        }

        fn children_names(&self) -> Vec<String> {
            self.children.keys().cloned().collect()
        }

        fn step_down(&self, name: String) -> Option<&Self> {
            self.children.get(&name)
        }

        pub fn from_corpus(corpus: String) -> Self {
            let mut tree = Tree::default();
            let sentences = split_corpus(corpus);
            for sentence in sentences {
                for suffix in suffixes(split_sentence(sentence)) {
                    tree.add_phrase(suffix)
                }
            }
            tree
        }

        pub fn names_at_path(&self, path: Vec<String>) -> Option<Vec<String>> {
            let mut node = self;
            for k in path {
                node = node.step_down(k)?;
            }
            Some(node.children_names())
        }

        pub fn span_map(&self) -> HashMap<String, usize> {
            self.children_names()
                .into_iter()
                .map(|name| {
                    (
                        name.clone(),
                        self.step_down(name)
                            .expect("iterating over names means the names are there")
                            .children_names()
                            .len(),
                    )
                })
                .collect()
        }

        pub fn depth_map(&self) -> HashMap<String, usize> {
            self.children_names()
                .into_iter()
                .map(|name| {
                    (
                        name.clone(),
                        self.step_down(name)
                            .expect("iterating over names means the names are there")
                            .get_depth(),
                    )
                })
                .collect()
        }
    }

    #[cfg(test)]
    mod tests {
        use std::vec;

        use crate::tree::tree::Tree;

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

        #[test]
        fn it_ingests_a_corpus() {
            let t = Tree::from_corpus("a b! c d. a, c? b: d;".to_string());

            //root has four children
            assert_eq!(t.children_names().len(), 4);
            assert!(t.children_names().contains(&"a".to_string()));
            assert!(t.children_names().contains(&"b".to_string()));
            assert!(t.children_names().contains(&"c".to_string()));
            assert!(t.children_names().contains(&"d".to_string()));

            // a has two children
            assert_eq!(
                t.step_down("a".to_string()).unwrap().children_names().len(),
                2
            );
            assert!(t
                .step_down("a".to_string())
                .unwrap()
                .children_names()
                .contains(&"b".to_string()));
            assert!(t
                .step_down("a".to_string())
                .unwrap()
                .children_names()
                .contains(&"c".to_string()));

            // b has one child
            assert_eq!(
                t.step_down("b".to_string()).unwrap().children_names().len(),
                1
            );
            assert!(t
                .step_down("b".to_string())
                .unwrap()
                .children_names()
                .contains(&"d".to_string()));

            // c has one child
            assert_eq!(
                t.step_down("c".to_string()).unwrap().children_names().len(),
                1
            );
            assert!(t
                .step_down("c".to_string())
                .unwrap()
                .children_names()
                .contains(&"d".to_string()));

            // d has no children
            assert_eq!(
                t.step_down("d".to_string()).unwrap().children_names().len(),
                0
            );

            // bad paths give back none
            assert!(t
                .step_down("a".to_string())
                .unwrap()
                .step_down("a".to_string())
                .is_none());
        }

        #[test]
        fn it_finds_names_at_a_path() {
            let t = Tree::from_corpus("a b c. a b d. a b e".to_string());

            let res = t
                .names_at_path(vec!["a".to_string(), "b".to_string()])
                .unwrap();
            assert_eq!(res.len(), 3);
            assert!(res.contains(&"c".to_string()));
            assert!(res.contains(&"d".to_string()));
            assert!(res.contains(&"e".to_string()));
        }

        #[test]
        fn it_returns_none_for_nonexistent_paths() {
            let t = Tree::from_corpus("a b c. a b d. a b e".to_string());

            let res = t.names_at_path(vec![
                "a".to_string(),
                "b".to_string(),
                "d".to_string(),
                "f".to_string(),
            ]);
            assert_eq!(res, None);
        }

        #[test]
        fn it_builds_a_span_map() {
            let t = Tree::from_corpus("a b c. a b d. a b e".to_string());

            let m = t.span_map();
            assert_eq!(m[&"a".to_string()], 1);
            assert_eq!(m[&"b".to_string()], 3);
            assert_eq!(m[&"c".to_string()], 0);
            assert_eq!(m[&"d".to_string()], 0);
            assert_eq!(m[&"e".to_string()], 0);
        }

        #[test]
        fn it_builds_a_depth_map() {
            let t = Tree::from_corpus("a. a b. b c d e.".to_string());

            let m = t.depth_map();
            assert_eq!(m[&"a".to_string()], 1);
            assert_eq!(m[&"b".to_string()], 3);
            assert_eq!(m[&"c".to_string()], 2);
            assert_eq!(m[&"d".to_string()], 1);
            assert_eq!(m[&"e".to_string()], 0);
        }
    }
}
