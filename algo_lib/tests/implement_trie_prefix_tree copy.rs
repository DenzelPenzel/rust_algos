struct Node {
    is_word: bool,
    children: [Option<Box<Node>>; 26],
}

impl Node {
    fn new() -> Self {
        Self {
            is_word: false,
            children: std::array::from_fn(|_| None),
        }
    }
}

struct Trie {
    root: Node,
}

impl Trie {
    fn new() -> Self {
        Self { root: Node::new() }
    }

    fn insert(&mut self, word: String) {
        let mut node = &mut self.root;
        for &byte in word.as_bytes() {
            let idx =
                Self::byte_to_index(byte).expect("word must contain only lowercase ASCII letters");
            node = node.children[idx]
                .get_or_insert_with(|| Box::new(Node::new()))
                .as_mut();
        }
        node.is_word = true;
    }

    fn search(&self, word: String) -> bool {
        self.walk(&word).map_or(false, |node| node.is_word)
    }

    fn starts_with(&self, prefix: String) -> bool {
        self.walk(&prefix).is_some()
    }

    fn walk<'a>(&'a self, s: &str) -> Option<&'a Node> {
        let mut node = &self.root;
        for &byte in s.as_bytes() {
            let idx = Self::byte_to_index(byte)?;
            node = node.children[idx].as_deref()?;
        }
        Some(node)
    }

    fn byte_to_index(byte: u8) -> Option<usize> {
        (b'a'..=b'z')
            .contains(&byte)
            .then(|| (byte - b'a') as usize)
    }
}
