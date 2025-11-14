/*
A trie (pronounced as "try") or prefix tree is a tree data structure
used to efficiently store and retrieve keys in a dataset of strings.

There are various applications of this data structure,
such as autocomplete and spellchecker.

Implement the Trie class:

Trie() Initializes the trie object.
    void insert(String word) Inserts the string word into the trie.
    boolean search(String word)
        Returns true if the string word is in
        the trie (i.e., was inserted before), and false otherwise.
    boolean startsWith(String prefix) Returns true if there is a previously inserted
        string word that has the prefix prefix, and false otherwise.

Example 1:
    Input
        ["Trie", "insert", "search", "search", "startsWith", "insert", "search"]
        [[], ["apple"], ["apple"], ["app"], ["app"], ["app"], ["app"]]
    Output
        [null, null, true, false, true, null, true]

    Explanation
        Trie trie = new Trie();
        trie.insert("apple");
        trie.search("apple");   // return True
        trie.search("app");     // return False
        trie.startsWith("app"); // return True
        trie.insert("app");
        trie.search("app");     // return True

Constraints:
    1 <= word.length, prefix.length <= 2000
    word and prefix consist only of lowercase English letters.
    At most 3 * 104 calls in total will be made to insert, search, and startsWith.
 */

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
            let idx = Self::byte_to_index(byte).unwrap();
            node = node.children[idx].get_or_insert_with(|| Box::new(Node::new())).as_mut();
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
            let idx = Self::byte_to_index(byte).unwrap();
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
