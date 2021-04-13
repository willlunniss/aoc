#[derive(PartialEq, Debug, Clone, Copy)]
enum Token {
    Num(usize),
    Add,
    Mult,
    Paren,
    Root,
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Mult,
}

#[derive(Debug, Clone)]
struct Node {
    children: Vec<Node>,
    entry: Token,
}

impl Node {
    const fn new(token: Token) -> Self {
        Self {
            children: Vec::new(),
            entry: token,
        }
    }

    fn new_sub(children: Vec<Self>) -> Self {
        Self {
            children,
            entry: Token::Paren,
        }
    }

    fn root(children: Vec<Self>) -> Self {
        Self {
            children,
            entry: Token::Root,
        }
    }
}

/// Recursively parses a String as a char iterator into a nested vector of nodes
fn parse(iter: &mut std::str::Chars<'_>) -> Vec<Node> {
    let mut nodes = Vec::new();
    while let Some(c) = iter.next() {
        match c {
            '0'..='9' => nodes.push(Node::new(Token::Num(c.to_digit(10).unwrap() as usize))),
            '+' => nodes.push(Node::new(Token::Add)),
            '*' => nodes.push(Node::new(Token::Mult)),
            '(' => nodes.push(Node::new_sub(parse(iter))),
            ')' => return nodes,
            ' ' => {}
            _ => panic!("Unexpected char '{}'", c),
        }
    }
    nodes
}

/// Performs the specified operation on two numbers
const fn operate(op: Operator, a: usize, b: usize) -> usize {
    match op {
        Operator::Add => a + b,
        Operator::Mult => a * b,
    }
}

/// Evaluates the equation left to right (recursing down into parens as needed)
fn evaluate(node: &Node) -> usize {
    let mut result = 0;
    let mut op: Operator = Operator::Add;
    for child in &node.children {
        match child.entry {
            Token::Num(value) => result = operate(op, result, value),
            Token::Add => op = Operator::Add,
            Token::Mult => op = Operator::Mult,
            Token::Paren => result = operate(op, result, evaluate(child)),
            _ => {
                panic!("Unexpected child {:?}", child.entry)
            }
        }
    }
    result
}

/// Transforms the equation such that addition has higher
/// precedence by wrapping them in parens
fn promote_add(node: &Node) -> Node {
    let mut it = node.children.iter().peekable();
    let mut new = Node {
        children: Vec::new(),
        entry: node.entry,
    };
    // If the node has children process them
    while let Some(child) = it.next() {
        if let Some(next) = it.peek() {
            if next.entry == Token::Add {
                // Transform Num + ... into (Num + ...) by creating a new Paren
                // with Num in it (next iteration will deal with + ...)
                new.children
                    .push(Node::new_sub([promote_add(&child.clone())].to_vec()));
                continue;
            }
        }
        if child.entry == Token::Add {
            // If this is an Add we know that there is already a new Paren for us to add onto
            // + is always followed by Num or Paren so add the it.next() as well
            new.children
                .last_mut()
                .unwrap()
                .children
                .push(child.clone());
            new.children
                .last_mut()
                .unwrap()
                .children
                .push(promote_add(&it.next().unwrap().clone()));
        } else {
            new.children.push(promote_add(&child.clone()));
        }
    }
    new
}

#[aoc_generator(day18)]
fn gen(input: &str) -> Vec<Node> {
    return input
        .lines()
        .map(|line| Node::root(parse(&mut line.chars())))
        .collect();
}

#[aoc(day18, part1)]
fn part1(input: &[Node]) -> usize {
    return input.iter().map(|e| evaluate(e)).sum();
}

#[aoc(day18, part2)]
fn part2(input: &[Node]) -> usize {
    return input.iter().map(|e| evaluate(&promote_add(e))).sum();
}
