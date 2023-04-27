// O_o
use std::fmt;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
enum Expr
{
    Sym(String),
    Fun(String, Vec<Expr>),
}

impl fmt::Display for Expr
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match self
        {
            Expr::Sym(name) => write!(f, "{}", name),
            Expr::Fun(name, args) => {
                write!(f, "{}(", name)?;
                for (i, arg) in args.iter().enumerate()
                {
                    if i > 0 { write!(f, ", ")? }
                    write!(f, "{}", arg)?;
                }
                write!(f, ")")
            }
        }
    }
}

#[derive(Debug)]
struct Rule
{
    head: Expr,
    body: Expr
}

fn substitute_bindings(bindings: &Bindings, expr: &Expr) -> Expr
{
    use Expr::*;
    match expr
    {
        Sym(name) => {
            if let Some(value) = bindings.get(name)
            {
                return value.clone();
            } else
            {
                return expr.clone();
            }
        },
        Fun(name, args) => {
            let new_name = match bindings.get(name)
            {
                Some(Sym(new_name)) => new_name.clone(),
                None => name.clone(),
                Some(_) => panic!("Expected symbol in the place of the functor name"),
            };
            let mut new_args = Vec::new();
            for arg in args
            {
                new_args.push(substitute_bindings(bindings, &arg))
            }
            return Fun(new_name, new_args);
        }
    }
}

impl Rule
{
    fn apply_all(&self, expr: &Expr) -> Expr
    {
        if let Some(bindings) = pattern_match(&self.head, expr)
        {
            substitute_bindings(&bindings, &self.body)
        } else
        {
            use Expr::*;
            match expr
            {
                Sym(_) => expr.clone(),
                Fun(name, args) => {
                    let mut new_args = Vec::new();
                    for arg in args 
                    {
                        new_args.push(self.apply_all(arg))
                    }
                    Fun(name.clone(), new_args)
                }
            }
        }
    }
}

impl fmt::Display for Rule
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{} => {}", self.head, self.body)
    }
}

type Bindings = HashMap<String, Expr>;


fn pattern_match(pattern: &Expr, value: &Expr) -> Option<Bindings>
{
    fn pattern_match_impl(pattern: &Expr, value: &Expr, bindings: &mut Bindings) -> bool
    {
        use Expr::*;
        match (pattern, value)
        {
            (Sym(name), _) => {
                if let Some(bound_value) = bindings.get(name)
                {
                    bound_value == value
                } else
                {
                    bindings.insert(name.clone(), value.clone());
                    true
                }
            },
            (Fun(name1, args1), Fun(name2, args2)) => {
                if name1 == name2 && args1.len() == args2.len()
                {
                    for i in 0..args1.len()
                    {
                        if !pattern_match_impl(&args1[i], &args2[i], bindings)
                        {
                            return false;
                        }
                    }
                    true
                } else
                {
                    false
                }
            },
            _ => false,
        }
    }

    let mut bindings = HashMap::new();

    if pattern_match_impl(pattern, value, &mut bindings)
    {
        Some(bindings)
    } else
    {
        None
    }
}

#[derive(Debug)]
enum TokenKind
{
    Sym(String),
    OpenParen,
    CloseParen,
    Comma,
    Equals,
}

#[derive(Debug)]
struct Token 
{
    kind: TokenKind,
    text: String,
}

struct Lexer<Chars: Iterator<Item=char>>
{
    chars: Peekable<Chars>
}

impl<Chars: Iterator<Item=char>> Lexer<Chars>
{
    fn from_iter(chars: Chars) -> Self
    {
        Self { chars: chars.peekable() }
    }
}

impl<Chars: Iterator<Item=char>> Iterator for Lexer<Chars>
{
    type Item = Token;
    fn next(&mut self) -> Option<Token>
    {
        todo!()
    }
}

fn main()
{
    for token in Lexer::from_iter("swap(pair(a, b)) = pair(b, a)".chars())
    {
        println!("{:?}", token);
    }
}

