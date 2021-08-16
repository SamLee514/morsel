use std::{collections::HashMap, hash::Hash};

enum Input {
    Dit,
    Dah,
    Space,
}

struct Node {
    left: Option<char>,
    right: Option<char>,
    value: Option<char>,
}

struct Tree {
    nodes: HashMap<Option<String>, Node>,
}

impl Tree {
    pub fn new() -> Self {
        let empty_fields = Node {
            left: None,
            right: None,
            value: None,
        };
        Self.nodes = HashMap::new();
        Self.nodes.insert(
            "5",
            Node {
                left: None,
                right: None,
                value: Some('5'),
            },
        );
        Self.nodes.insert(
            "4",
            Node {
                value: Some('4'),
                ..empty_fields
            },
        );
        Self.nodes.insert(
            "3",
            Node {
                value: Some('3'),
                ..empty_fields
            },
        );
        Self.nodes.insert(
            "2",
            Node {
                value: Some('2'),
                ..empty_fields
            },
        );
        Self.nodes.insert(
            "+",
            Node {
                value: Some('+'),
                ..empty_fields
            },
        );
        Self.nodes.insert(
            "1",
            Node {
                value: Some('1'),
                ..empty_fields
            },
        );
        Self.nodes.insert(
            "6",
            Node {
                value: Some('6'),
                ..empty_fields
            },
        );
        Self.nodes.insert()
    }
}

fn get_tree() -> Tree<'static> {
    static empty_fields: Node = Node {
        left: None,
        right: None,
        value: Some('5'),
    };
    static four: Node = Node {
        value: Some('4'),
        ..empty_fields
    };
    static three: Node = Node {
        value: Some('3'),
        ..empty_fields
    };
    static two: Node = Node {
        value: Some('2'),
        ..empty_fields
    };
    static plus: Node = Node {
        value: Some('+'),
        ..empty_fields
    };
    static one: Node = Node {
        value: Some('1'),
        ..empty_fields
    };
    static six: Node = Node {
        value: Some('6'),
        ..empty_fields
    };
    static equals: Node = Node {
        value: Some('='),
        ..empty_fields
    };
    static slash: Node = Node {
        value: Some('/'),
        ..empty_fields
    };
    static seven: Node = Node {
        value: Some('7'),
        ..empty_fields
    };
    static eight: Node = Node {
        value: Some('8'),
        ..empty_fields
    };
    static nine: Node = Node {
        value: Some('9'),
        ..empty_fields
    };
    static zero: Node = Node {
        value: Some('0'),
        ..empty_fields
    };
    static h: Node = Node {
        left: Some("5"),
        right: Some("4"),
        value: Some('h'),
    };
    static v: Node = Node {
        left: None,
        right: Some("3"),
        value: Some('v'),
    };
    static f: Node = Node {
        value: Some('f'),
        ..empty_fields
    };
    static pre_two: Node = Node {
        left: None,
        right: Some("2"),
        value: None,
    };
    static l: Node = Node {
        value: Some('l'),
        ..empty_fields
    };
    static pre_plus: Node = Node {
        left: Some("+"),
        right: None,
        value: None,
    };
    static p: Node = Node {
        value: Some('p'),
        ..empty_fields
    };
    static j: Node = Node {
        left: None,
        right: Some("1"),
        value: Some('j'),
    };
    static b: Node = Node {
        left: Some("6"),
        right: Some("="),
        value: Some('b'),
    };
    static x: Node = Node {
        left: Some("/"),
        right: None,
        value: Some('x'),
    };
    static c: Node = Node {
        value: Some('c'),
        ..empty_fields
    };
    static y: Node = Node {
        value: Some('y'),
        ..empty_fields
    };
    static z: Node = Node {
        left: Some("7"),
        right: None,
        value: Some('z'),
    };
    static q: Node = Node {
        value: Some('q'),
        ..empty_fields
    };
    static pre_eight: Node = Node {
        left: Some("8"),
        right: None,
        value: None,
    };
    static pre_nine_zero: Node = Node {
        left: Some("9"),
        right: Some("0"),
        value: None,
    };
    static s: Node = Node {
        left: Some("h"),
        right: Some("v"),
        value: Some('s'),
    };
    static u: Node = Node {
        left: Some("f"),
        right: Some("p2"),
        value: Some('u'),
    };
    static r: Node = Node {
        left: Some("l"),
        right: Some("p+"),
        value: Some('r'),
    };
    static w: Node = Node {
        left: Some("p"),
        right: Some("j"),
        value: Some('w'),
    };
    static d: Node = Node {
        left: Some("b"),
        right: Some("x"),
        value: Some('d'),
    };
    static k: Node = Node {
        left: Some("c"),
        right: Some("y"),
        value: Some('k'),
    };
    static g: Node = Node {
        left: Some("z"),
        right: Some("q"),
        value: Some('g'),
    };
    static o: Node = Node {
        left: Some("p8"),
        right: Some("p90"),
        value: Some('o'),
    };
    static i: Node = Node {
        left: Some("s"),
        right: Some("u"),
        value: Some('i'),
    };
    static a: Node = Node {
        left: Some("r"),
        right: Some("w"),
        value: Some('a'),
    };
    static n: Node = Node {
        left: Some("d"),
        right: Some("k"),
        value: Some('n'),
    };
    static m: Node = Node {
        left: Some("g"),
        right: Some("o"),
        value: Some('m'),
    };
    static e: Node = Node {
        left: Some("i"),
        right: Some("a"),
        value: Some('e'),
    };
    static t: Node = Node {
        left: Some("n"),
        right: Some("m"),
        value: Some('t'),
    };
    return Tree {
        head: Node {
            left: Some("e"),
            right: Some("t"),
            value: None,
        },
    };
}

// Build the tree as an array
