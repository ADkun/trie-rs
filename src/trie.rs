use std::collections::HashMap;

struct Node {
    /// 是否是单词结尾
    done: bool,
    /// 子节点
    children: HashMap<char, Box<Node>>,
}

impl Node {
    /// 创建一个新的Node
    fn new(done: bool) -> Self {
        Self {
            done,
            children: HashMap::new(),
        }
    }

    /// 创建一个新的Box<Node>
    fn boxed(done: bool) -> Box<Self> {
        Box::new(Self::new(done))
    }
}

struct Trie {
    head: Box<Node>,
}

impl Trie {
    /// 创建一个Trie
    pub fn new() -> Self {
        Self {
            head: Node::boxed(false),
        }
    }

    /// 添加词语
    pub fn add(&mut self, s: &str) {
        let mut current = &mut self.head;
        for c in s.chars() {
            current = current
                .children
                .entry(c)
                .or_insert_with(|| Node::boxed(false));
        }
        current.done = true; // 标记这个节点为单词的终点
    }

    /// 搜索词语是否存在
    pub fn search(&self, s: &str) -> bool {
        let mut current = &self.head;
        for c in s.chars() {
            match current.children.get(&c) {
                Some(cur) => current = cur,
                None => return false,
            }
        }
        current.done
    }

    /// 过滤文本
    pub fn filter(&self, text: &str, replace: &str) -> String {
        let mut result = String::with_capacity(text.len());
        let chars: Vec<char> = text.chars().collect();
        let mut position = 0;
        let mut begin = 0;
        let mut current = &self.head;
        while position < chars.len() {
            match current.children.get(&chars[position]) {
                None => {
                    result.push(chars[begin]);
                    begin += 1;
                    position = begin;
                    current = &self.head;
                }
                Some(node) => match node.done {
                    true => {
                        result.push_str(replace);
                        position += 1;
                        begin = position;
                        current = &self.head;
                    }
                    false => {
                        current = node;
                        position += 1;
                    }
                },
            }
        }
        result
    }
}

#[allow(dead_code)]
pub fn test() {
    test_basic();
    test_filter();
}

fn test_filter() {
    let mut trie = Trie::new();
    trie.add("王八羔子");
    trie.add("王八蛋");
    trie.add("大坏蛋");
    trie.add("大猪蹄子");
    trie.add("讨厌");

    let text = "哈哈大王八子大猪蹄子哦";
    let filtered = trie.filter(text, "***");
    println!("{filtered}");
}

fn test_basic() {
    let mut trie = Trie::new();
    trie.add("123");

    assert!(!trie.search("1"));
    assert!(!trie.search("2"));
    assert!(!trie.search("3"));
    assert!(!trie.search("12"));
    assert!(trie.search("123"));
    assert!(!trie.search("1234"));

    trie.add("12");
    assert!(!trie.search("1"));
    assert!(trie.search("12"));
    assert!(!trie.search("1234"));

    trie.add("中国人");
    trie.add("中国");
    assert!(!trie.search("中"));
    assert!(trie.search("中国"));
    assert!(trie.search("中国人"));
    assert!(!trie.search("中国人是"));
}
