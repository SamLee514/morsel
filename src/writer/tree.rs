use std::{collections::HashMap, hash::Hash};

#[derive(Copy, Clone)]
pub enum Input {
    Dit,
    Dah,
    Space,
}

pub enum Output {
    Value(char),
    Pass,
    Oopsie,
}

// Need a character, a "pass" or an error
// Dits and Dahs don't try to access the character, so they are either "pass" or Error
// Space try to access the character, so they are either character or Error

struct Node<'a> {
    left: Option<&'a str>,
    right: Option<&'a str>,
    value: Option<char>,
}

pub struct Tree<'a> {
    nodes: HashMap<&'a str, Node<'a>>,
    cur: &'a str,
}

impl Tree<'_> {
    pub fn new() -> Self {
        let empty_fields = Node {
            left: None,
            right: None,
            value: None,
        };
        let mut nodes = HashMap::new();
        nodes.insert(
            "5",
            Node {
                left: None,
                right: None,
                value: Some('5'),
            },
        );
        nodes.insert(
            "4",
            Node {
                value: Some('4'),
                ..empty_fields
            },
        );
        nodes.insert(
            "3",
            Node {
                value: Some('3'),
                ..empty_fields
            },
        );
        nodes.insert(
            "2",
            Node {
                value: Some('2'),
                ..empty_fields
            },
        );
        nodes.insert(
            "+",
            Node {
                value: Some('+'),
                ..empty_fields
            },
        );
        nodes.insert(
            "1",
            Node {
                value: Some('1'),
                ..empty_fields
            },
        );
        nodes.insert(
            "6",
            Node {
                value: Some('6'),
                ..empty_fields
            },
        );
        nodes.insert(
            "=",
            Node {
                value: Some('='),
                ..empty_fields
            },
        );
        nodes.insert(
            "/",
            Node {
                value: Some('/'),
                ..empty_fields
            },
        );
        nodes.insert(
            "7",
            Node {
                value: Some('7'),
                ..empty_fields
            },
        );
        nodes.insert(
            "8",
            Node {
                value: Some('8'),
                ..empty_fields
            },
        );
        nodes.insert(
            "9",
            Node {
                value: Some('9'),
                ..empty_fields
            },
        );
        nodes.insert(
            "0",
            Node {
                value: Some('0'),
                ..empty_fields
            },
        );
        nodes.insert(
            "h",
            Node {
                left: Some("5"),
                right: Some("4"),
                value: Some('h'),
            },
        );
        nodes.insert(
            "v",
            Node {
                left: None,
                right: Some("3"),
                value: Some('v'),
            },
        );
        nodes.insert(
            "f",
            Node {
                value: Some('f'),
                ..empty_fields
            },
        );
        nodes.insert(
            "pre2",
            Node {
                left: None,
                right: Some("2"),
                value: None,
            },
        );
        nodes.insert(
            "l",
            Node {
                value: Some('l'),
                ..empty_fields
            },
        );
        nodes.insert(
            "pre+",
            Node {
                left: Some("+"),
                right: None,
                value: None,
            },
        );
        nodes.insert(
            "p",
            Node {
                value: Some('p'),
                ..empty_fields
            },
        );
        nodes.insert(
            "j",
            Node {
                left: None,
                right: Some("1"),
                value: Some('j'),
            },
        );
        nodes.insert(
            "b",
            Node {
                left: Some("6"),
                right: Some("="),
                value: Some('b'),
            },
        );
        nodes.insert(
            "x",
            Node {
                left: Some("/"),
                right: None,
                value: Some('x'),
            },
        );
        nodes.insert(
            "c",
            Node {
                value: Some('c'),
                ..empty_fields
            },
        );
        nodes.insert(
            "y",
            Node {
                value: Some('y'),
                ..empty_fields
            },
        );
        nodes.insert(
            "z",
            Node {
                left: Some("7"),
                right: None,
                value: Some('z'),
            },
        );
        nodes.insert(
            "q",
            Node {
                value: Some('q'),
                ..empty_fields
            },
        );
        nodes.insert(
            "pre8",
            Node {
                left: Some("8"),
                right: None,
                value: None,
            },
        );
        nodes.insert(
            "pre90",
            Node {
                left: Some("9"),
                right: Some("0"),
                value: None,
            },
        );
        nodes.insert(
            "s",
            Node {
                left: Some("h"),
                right: Some("v"),
                value: Some('s'),
            },
        );
        nodes.insert(
            "u",
            Node {
                left: Some("f"),
                right: Some("p2"),
                value: Some('u'),
            },
        );
        nodes.insert(
            "r",
            Node {
                left: Some("l"),
                right: Some("p+"),
                value: Some('r'),
            },
        );
        nodes.insert(
            "w",
            Node {
                left: Some("p"),
                right: Some("j"),
                value: Some('w'),
            },
        );
        nodes.insert(
            "d",
            Node {
                left: Some("b"),
                right: Some("x"),
                value: Some('d'),
            },
        );
        nodes.insert(
            "k",
            Node {
                left: Some("c"),
                right: Some("y"),
                value: Some('k'),
            },
        );
        nodes.insert(
            "g",
            Node {
                left: Some("z"),
                right: Some("q"),
                value: Some('g'),
            },
        );
        nodes.insert(
            "o",
            Node {
                left: Some("pre8"),
                right: Some("pre90"),
                value: Some('o'),
            },
        );
        nodes.insert(
            "i",
            Node {
                left: Some("s"),
                right: Some("u"),
                value: Some('i'),
            },
        );
        nodes.insert(
            "a",
            Node {
                left: Some("r"),
                right: Some("w"),
                value: Some('a'),
            },
        );
        nodes.insert(
            "n",
            Node {
                left: Some("d"),
                right: Some("k"),
                value: Some('n'),
            },
        );
        nodes.insert(
            "m",
            Node {
                left: Some("g"),
                right: Some("o"),
                value: Some('m'),
            },
        );
        nodes.insert(
            "e",
            Node {
                left: Some("i"),
                right: Some("a"),
                value: Some('e'),
            },
        );
        nodes.insert(
            "t",
            Node {
                left: Some("n"),
                right: Some("m"),
                value: Some('t'),
            },
        );
        nodes.insert(
            "head",
            Node {
                left: Some("e"),
                right: Some("t"),
                value: None,
            },
        );
        Self { nodes, cur: "head" }
    }

    pub fn traverse(&mut self, input: Input) -> Output {
        match input {
            Input::Dit => match self.nodes.get(self.cur).unwrap().left {
                Some(key) => {
                    self.cur = key;
                    return Output::Pass;
                }
                None => {
                    self.cur = "head";
                    return Output::Oopsie;
                }
            },
            Input::Dah => match self.nodes.get(self.cur).unwrap().right {
                Some(key) => {
                    self.cur = key;
                    return Output::Pass;
                }
                None => {
                    self.cur = "head";
                    return Output::Oopsie;
                }
            },
            Input::Space => match self.nodes.get(self.cur).unwrap().value {
                Some(value) => {
                    self.cur = "head";
                    return Output::Value(value);
                }
                None => {
                    self.cur = "head";
                    return Output::Oopsie;
                }
            },
        }
    }
}

// Build the tree as an array
// Rest of the app passes in an array of dits and dahs, expects either a letter or an error
// Error signals to kill the currently-being-typed letter
