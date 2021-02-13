#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Token {    
    Num(usize),
    Add,
    Mult,
    Paren,
    Root,
}

#[derive(Debug, Clone, Copy)]
pub enum Operator {
    Add,
    Mult,
}

#[derive(Debug, Clone)]
pub struct Node {
    children: Vec<Node>,
    entry: Token,
}

impl Node {
    pub fn new(token: Token) -> Node {
        Node { children: Vec::new(), entry: token }
    }

    pub fn new_sub(children: Vec<Node>) -> Node {
        Node { children: children, entry: Token::Paren }
    }

    pub fn root(children: Vec<Node>) -> Node {
        Node {children: children, entry: Token::Root }
    }
}

/// Parses a String as a char iterator into a nested vector of nodes
pub fn parse(iter: &mut std::str::Chars<'_>) -> Vec<Node> {
    let mut nodes = Vec::new();
    while let Some(c) = iter.next() {
        match c {
            '0'..='9' => { nodes.push(Node::new(Token::Num(c.to_digit(10).unwrap() as usize))) },
            '+' => { nodes.push(Node::new(Token::Add)) },
            '*' => { nodes.push(Node::new(Token::Mult)) },
            '(' => { nodes.push(Node::new_sub(parse(iter))) }
            ')' => { return nodes },
            ' ' => {},
            _ => panic!("Unexpected char '{}'", c)
        }
    }
    return nodes;
}

/// Performs the specified operation on two numbers
pub fn operate(op: &Operator, a: usize, b: usize) -> usize {
    match op {
        Operator::Add => return a + b,
        Operator::Mult => return a * b,
    }
}

/// Evaluates the equation left to right
pub fn evaulate(node: &Node) -> usize {
    let mut result = 0;
    let mut op : Operator = Operator::Add;
    for child in &node.children {
        match child.entry {
            Token::Num(value) => { result = operate(&op, result, value) },
            Token::Add => { op = Operator::Add },
            Token::Mult => { op = Operator::Mult },
            Token::Paren => { result = operate(&op, result, evaulate(&child)) }
            _ => { panic!("Unexpected child {:?}", child.entry) }
        }
    }
    return result;
}

/// Transforms the equation such that addition has higher
/// precendence by wrappinng them in parens
pub fn promote_add(node: &Node) -> Node {
    let mut it = node.children.iter().peekable();
    let mut new = Node { children: Vec::new(), entry: node.entry.clone() };
    while let Some(child) = it.next() {
        if let Some(next) = it.peek() {
            if next.entry == Token::Add {
                // Transform Num + ... into (Num + ...) by creating a new Paren
                // with Num in it (next iteration will deal with + ...)
                new.children.push(Node::new_sub([promote_add(&child.clone())].to_vec()));
                continue;
            }
        }
        if child.entry == Token::Add {
            // If this is an Add we know that there is already a new Paren for us to add onto
            // + is always followed by Num or Paren so add the it.next() as well
            new.children.last_mut().unwrap().children.push(child.clone());
            new.children.last_mut().unwrap().children.push(promote_add(&it.next().unwrap().clone()));
        } else {
            new.children.push(promote_add(&child.clone()));
        }
    }
    return new;
}

#[aoc_generator(day18)]
pub fn gen(input: &str) -> Vec<Node> {
    let mut equations = Vec::new();
    for line in input.lines() {
        equations.push(Node::root(parse(&mut line.chars())));
    }
    return equations;
}

#[aoc(day18, part1)]
fn part1(input: &Vec<Node>) -> usize {
    return input.iter().map(|e| evaulate(e)).sum();
}

#[aoc(day18, part2)]
fn part2(input: &Vec<Node>) -> usize {
    return input.iter().map(|e| evaulate(&promote_add(e))).sum();
}
