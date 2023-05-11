use std::fmt;
use std::iter::Peekable;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Loc
{
	pub file_path: Option<String>,
	pub row: usize,
	pub col: usize,
}

impl fmt::Display for Loc
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match &self.file_path
            {
                Some(file_path) => write!(f, "{}:{}:{}", file_path, self.row, self.col),
                None => write!(f, "{}:{}", self.row, self.col),
            }
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum TokenKind
{
    Sym,
    // Keywords
    Rule,
    Shape,
    Apply,
    Done,
    // Special Characters
    OpenParen,
    CloseParen,
    Comma,
    Equals,
    Colon,
    // Terminators
    Invalid,
    End,
}

fn keyword_by_name(text: &str) -> Option<TokenKind>
{
    match text
    {
        "rule" => Some(TokenKind::Rule),
        "shape" => Some(TokenKind::Shape),
        "apply" => Some(TokenKind::Apply),
        "done" => Some(TokenKind::Done),
        _ => None,
    }
}

impl fmt::Display for TokenKind
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        use TokenKind::*;
        match self
        {
            Sym => write!(f, "symbol"),
            Rule => write!(f, "rule keyword"),
            Shape => write!(f, "shape keyword"),
            Apply => write!(f, "apply keyword"),
            Done => write!(f, "done keyword"),
            OpenParen => write!(f, "open paren"),
            CloseParen => write!(f, "close paren"),
            Comma => write!(f, "comma"),
            Equals => write!(f, "equals"),
            Colon => write!(f, "colon"),
            Invalid => write!(f, "invalid token"),
            End => write!(f, "end of input"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Token
{
	pub kind: TokenKind,
	pub text: String,
	pub loc: Loc,
}

pub struct Lexer<Chars: Iterator<Item=char>>
{
    chars: Peekable<Chars>,
    exhausted: bool,
    file_path: Option<String>,
    lnum: usize,
    bol: usize,
    cnum: usize,
}

impl<Chars: Iterator<Item=char>> Lexer<Chars>
{
    pub fn from_iter(chars: Chars) -> Self
    {
        Self { 
            chars: chars.peekable(), 
            exhausted: false ,
            file_path: None,
            lnum: 0,
            bol: 0,
            cnum: 0,
        }
    }

    fn loc(&self) -> Loc
    {
        Loc {
            file_path: self.file_path.clone(),
            row: self.lnum,
            col: self.cnum - self.bol,
        }
    }

    pub fn set_file_path(&mut self, file_path: &str)
    {
        self.file_path = Some(file_path.to_string())
    }
}

impl<Chars: Iterator<Item=char>> Iterator for Lexer<Chars>
{
    type Item = Token;

    fn next(&mut self) -> Option<Token>
    {
        if self.exhausted { return None }

        while let Some(x) = self.chars.next_if(|x| x.is_whitespace())
        {
            self.cnum += 1;
            if x == '\n'
            {
                self.lnum += 1;
                self.bol = self.cnum;
            }
        }

        let loc = self.loc();
        match self.chars.next()
        {
            Some(x) => {
                self.cnum += 1;
                let mut text = x.to_string();
                match x
                {
                    '(' => Some(Token {kind: TokenKind::OpenParen, text, loc}),
                    ')' => Some(Token {kind: TokenKind::CloseParen, text, loc}),
                    ',' => Some(Token {kind: TokenKind::Comma, text, loc}),
                    '=' => Some(Token {kind: TokenKind::Equals, text, loc}),
                    ':' => Some(Token {kind: TokenKind::Colon, text, loc}),
                    _ => {
                        if !x.is_alphanumeric()
                        {
                            self.exhausted = true;
                            Some(Token{kind: TokenKind::Invalid, text, loc})
                        } else
                        {
                            while let Some(x) = self.chars.next_if(|x| x.is_alphanumeric())
                            {
                                self.cnum += 1;
                                text.push(x)
                            }

                            if let Some(kind) = keyword_by_name(&text)
                            {
                                Some(Token{kind, text, loc})
                            } else 
                            {
                                Some(Token{ kind: TokenKind::Sym, text, loc })
                            }
                        }
                    }
                }
            },
            None => {
                self.exhausted = true;
                Some(Token{kind: TokenKind::End, text: "".to_string(), loc})
            }
        }
    }
}
