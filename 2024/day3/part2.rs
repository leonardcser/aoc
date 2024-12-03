use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process::exit;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() <= 1 {
        eprintln!("Usage: {} <file_path>", args[0]);
        exit(1);
    }
    if let Err(err) = process_file(&args[1]) {
        eprintln!("Error: {}", err);
        exit(1);
    }
}

#[derive(Debug, PartialEq)]
enum TokenType {
    OpenParen,
    CloseParen,
    Comma,
    NumberLiteral,
    MulFun,
    Unknown,
    DoFun,
    DontFun,
}

#[derive(Debug)]
struct Token {
    token_type: TokenType,
    value: String,
}

fn is_token(slice: &[char], prefix: &str) -> bool {
    slice.iter().collect::<String>().starts_with(prefix)
}

fn tokenize(chars: &[char]) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    let mut i = 0;
    while i < chars.len() {
        match chars[i] {
            'm' => {
                // Check for "mul" function
                if i + 2 < chars.len() && is_token(&chars[i..], "mul") {
                    tokens.push(Token {
                        token_type: TokenType::MulFun,
                        value: "mul".to_string(),
                    });
                    i += 3; // Advance past "mul"
                } else {
                    i += 1;
                }
            }
            'd' => {
                if i + 3 < chars.len() && is_token(&chars[i..], "do()") {
                    tokens.push(Token {
                        token_type: TokenType::DoFun,
                        value: "do()".to_string(),
                    });
                    i += 4;
                } else if i + 6 < chars.len() && is_token(&chars[i..], "don't()") {
                    tokens.push(Token {
                        token_type: TokenType::DontFun,
                        value: "don't()".to_string(),
                    });
                    i += 7;
                } else {
                    i += 1;
                }
            }
            '(' => {
                tokens.push(Token {
                    token_type: TokenType::OpenParen,
                    value: "(".to_string(),
                });
                i += 1;
            }
            ')' => {
                tokens.push(Token {
                    token_type: TokenType::CloseParen,
                    value: ")".to_string(),
                });
                i += 1;
            }
            ',' => {
                tokens.push(Token {
                    token_type: TokenType::Comma,
                    value: ",".to_string(),
                });
                i += 1;
            }
            '0'..='9' => {
                // Handle number literals
                let mut number = String::new();
                while i < chars.len() && chars[i].is_ascii_digit() {
                    number.push(chars[i]);
                    i += 1;
                }
                tokens.push(Token {
                    token_type: TokenType::NumberLiteral,
                    value: number,
                });
            }
            _ => {
                tokens.push(Token {
                    token_type: TokenType::Unknown,
                    value: chars[i].to_string(),
                });
                i += 1;
            }
        }
    }

    tokens
}

fn parse(tokens: &[Token]) -> i32 {
    let mut sum: i32 = 0;
    let mut i = 0;
    let mut should_do = true;
    while i < tokens.len() - 5 {
        match tokens[i].token_type {
            TokenType::DoFun => should_do = true,
            TokenType::DontFun => should_do = false,
            _ => {}
        }
        if should_do
            && tokens[i].token_type == TokenType::MulFun
            && tokens[i + 1].token_type == TokenType::OpenParen
            && tokens[i + 2].token_type == TokenType::NumberLiteral
            && tokens[i + 3].token_type == TokenType::Comma
            && tokens[i + 4].token_type == TokenType::NumberLiteral
            && tokens[i + 5].token_type == TokenType::CloseParen
        {
            let left: i32 = tokens[i + 2].value.parse().unwrap();
            let right: i32 = tokens[i + 4].value.parse().unwrap();
            sum += left * right;
            i += 6;
            continue;
        }
        i += 1;
    }
    sum
}

fn process_file(file_path: &str) -> io::Result<()> {
    let chars: Vec<char> = read_lines(file_path)?
        .flatten()
        .flat_map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let tokens = tokenize(&chars);
    let sum = parse(&tokens);

    dbg!(sum);
    Ok(())
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
