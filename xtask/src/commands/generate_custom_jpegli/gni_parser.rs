use anyhow::bail;
use anyhow::ensure;
use anyhow::Context;
use std::collections::HashMap;

/// A Token
#[derive(Debug, PartialEq)]
enum Token {
    Identifier(String),
    String(String),
    Int(i64),

    Equal,
    LeftBracket,
    RightBracket,
    LeftParenthesis,
    RightParenthesis,
    Comma,
}

/// Tokenize a gni file.
fn tokenize_gni(input: &str) -> anyhow::Result<Vec<Token>> {
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    enum State {
        Initial,
        Comment,
        Identifier,
        String,
        Int,
    }

    let mut state = State::Initial;
    let mut identifier = String::new();
    let mut string = String::new();
    let mut int = String::new();
    let mut tokens = Vec::new();
    for ch in input.chars() {
        loop {
            match (state, ch) {
                (State::Initial, '#') => {
                    state = State::Comment;
                }
                (State::Initial, ch) if ch.is_ascii_alphabetic() || ch == '_' => {
                    identifier.push(ch);
                    state = State::Identifier;
                }
                (State::Initial, ch) if ch.is_ascii_digit() => {
                    int.push(ch);
                    state = State::Int;
                }
                (State::Initial, '\n' | ' ') => {
                    // Ignore whitespace in initial state.
                }
                (State::Initial, '=') => {
                    tokens.push(Token::Equal);
                }
                (State::Initial, '[') => {
                    tokens.push(Token::LeftBracket);
                }
                (State::Initial, ']') => {
                    tokens.push(Token::RightBracket);
                }
                (State::Initial, '(') => {
                    tokens.push(Token::LeftParenthesis);
                }
                (State::Initial, ')') => {
                    tokens.push(Token::RightParenthesis);
                }
                (State::Initial, ',') => {
                    tokens.push(Token::Comma);
                }
                (State::Initial, '"') => {
                    state = State::String;
                }
                (State::Initial, _) => {
                    bail!("Unknown Initial Char: \"{ch}\"");
                }
                (State::Comment, '\n') => {
                    state = State::Initial;
                }
                (State::Comment, _) => {
                    // Ignore Comments
                }
                (State::Identifier, ch) if ch.is_ascii_alphabetic() || ch == '_' => {
                    identifier.push(ch);
                }
                (State::Identifier, ' ') => {
                    tokens.push(Token::Identifier(std::mem::take(&mut identifier)));
                    state = State::Initial;
                }
                (State::Identifier, '(') => {
                    tokens.push(Token::Identifier(std::mem::take(&mut identifier)));
                    state = State::Initial;
                    continue;
                }
                (State::Identifier, _) => {
                    bail!("Unknown Identifier Char: \"{ch}\"");
                }
                (State::String, '"') => {
                    tokens.push(Token::String(std::mem::take(&mut string)));
                    state = State::Initial;
                }
                (State::String, ch) => {
                    string.push(ch);
                }
                (State::Int, ch) if ch.is_ascii_digit() => {
                    int.push(ch);
                }
                (State::Int, '\n') => {
                    let int = std::mem::take(&mut int);
                    let int: i64 = int.parse()?;
                    tokens.push(Token::Int(int));
                    state = State::Initial;
                }
                (State::Int, _) => {
                    bail!("Unknown Int Char: \"{ch}\"");
                }
            }

            break;
        }
    }

    ensure!(state == State::Initial);

    Ok(tokens)
}

/// A Gni expression
// We want to parse this, even if we don't use it.
#[allow(dead_code)]
#[derive(Debug)]
pub enum Expr {
    String(String),
    Int(i64),
    List(Vec<Expr>),
    FunctionCall { name: String, args: Vec<Expr> },
}

impl Expr {
    /// Get this as a string.
    pub fn as_string(&self) -> Option<&str> {
        match self {
            Self::String(string) => Some(string.as_str()),
            _ => None,
        }
    }

    /// Get this as a list.
    pub fn as_list(&self) -> Option<&[Expr]> {
        match self {
            Self::List(list) => Some(list.as_slice()),
            _ => None,
        }
    }
}

/// Parse a gni expr.
fn parse_gni_expr<'a>(tokens_iter: &mut std::vec::IntoIter<Token>) -> anyhow::Result<Expr> {
    let token = tokens_iter.next().context("missing token")?;
    match token {
        Token::LeftBracket => {
            let mut list = Vec::new();
            loop {
                let next_token = tokens_iter.as_slice().get(0).context("missing token")?;
                if next_token == &Token::RightBracket {
                    let _ = tokens_iter.next().is_some();
                    break;
                }

                let expr = parse_gni_expr(tokens_iter)?;
                list.push(expr);

                let next_token = tokens_iter.as_slice().get(0).context("missing token")?;
                if next_token == &Token::RightBracket {
                    let _ = tokens_iter.next().is_some();
                    break;
                }

                let token = tokens_iter.next().context("missing token")?;
                ensure!(token == Token::Comma);
            }

            Ok(Expr::List(list))
        }
        Token::Identifier(identifier) => {
            let token = tokens_iter.next().context("missing token")?;
            ensure!(token == Token::LeftParenthesis);

            let mut args = Vec::new();
            loop {
                let next_token = tokens_iter.as_slice().get(0).context("missing token")?;
                if next_token == &Token::RightParenthesis {
                    let _ = tokens_iter.next().is_some();
                    break;
                }

                let expr = parse_gni_expr(tokens_iter)?;
                args.push(expr);

                let next_token = tokens_iter.as_slice().get(0).context("missing token")?;
                if next_token == &Token::RightParenthesis {
                    let _ = tokens_iter.next().is_some();
                    break;
                }

                let token = tokens_iter.next().context("missing token")?;
                ensure!(token == Token::Comma);
            }

            Ok(Expr::FunctionCall {
                name: identifier,
                args,
            })
        }
        Token::String(string) => Ok(Expr::String(string)),
        Token::Int(int) => Ok(Expr::Int(int)),
        _ => {
            bail!("Unexpected Expr Token \"{token:?}\"");
        }
    }
}

/// A simple function to parse a gni file.
pub(super) fn parse_gni(input: &str) -> anyhow::Result<HashMap<String, Expr>> {
    let tokens = tokenize_gni(input)?;
    let mut tokens_iter = tokens.into_iter();

    let mut map = HashMap::new();
    while let Some(token) = tokens_iter.next() {
        match token {
            Token::Identifier(identifier) => {
                let token_1 = tokens_iter.next().context("missing token")?;
                ensure!(token_1 == Token::Equal);

                let expr = parse_gni_expr(&mut tokens_iter)?;
                map.insert(identifier, expr);
            }
            _ => {
                bail!("Unexpected Token \"{token:?}\"");
            }
        }
    }

    Ok(map)
}
