use regex::Regex;
use std::collections::HashMap;
use std::iter::Peekable;
use std::process;
use std::str::Chars;

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum TokenCategory {
    Number,
    KeywordReturn,
    Identifier,
    KeywordInt,
    Plus,
    SemiColon,
    LeftCurlyBrace,
    RightCurlyBrace,
    LeftBracket,
    RightBracket,
    Eof,
    TokenError, // If token is invalid
}

#[derive(Debug)]
pub struct Token {
    pub category: TokenCategory,
    pub attribute_value: Option<String>,
}

pub struct Lexer<'a> {
    input_file: &'a str,
    char_iterator: Peekable<Chars<'a>>,
    matches: HashMap<TokenCategory, Regex>,
}

// 'a avoids copuing the string
impl<'a> Lexer<'a> {
    pub fn new(input_file: &'a str) -> Self {
        let mut lexer = Lexer {
            input_file,
            char_iterator: input_file.chars().peekable(),
            matches: HashMap::from([
                (TokenCategory::Number, Regex::new(r"[0-9]+").unwrap()),
                (TokenCategory::KeywordReturn, Regex::new(r"return\b").unwrap()),
                (
                    TokenCategory::Identifier,
                    Regex::new(r"[A-Za-z][A-Za-z0-9]*").unwrap(),
                ),
                (TokenCategory::KeywordInt, Regex::new(r"int\b").unwrap()),
                (TokenCategory::Plus, Regex::new(r"\+").unwrap()),
                (TokenCategory::SemiColon, Regex::new(r";").unwrap()),
                (TokenCategory::LeftCurlyBrace, Regex::new(r"\{").unwrap()),
                (TokenCategory::RightCurlyBrace, Regex::new(r"\}").unwrap()),
                (TokenCategory::TokenError, Regex::new(r".*").unwrap()),
            ]),
        };

        return lexer;
    }

    pub fn print_input(&mut self) {
        println!("{}", self.input_file);
    }

    pub fn get_next_token(&mut self) -> Token {
        use TokenCategory::*;
        let mut cur_lexeme = String::from("");
        let mut token = Token {
            category: TokenError,
            attribute_value: None,
        };

        // find next non whitespace
        while self
            .char_iterator
            .peek()
            .map_or(false, |c| c.is_whitespace())
        {
            self.char_iterator.next();
        }

        if self.char_iterator.peek().is_none() {
            token.category = Eof;
            return token;
        }

        let first_char = self.char_iterator.next().unwrap();
        let maybe_identifier = first_char.is_alphabetic();
        let is_number = first_char.is_digit(10);
        cur_lexeme.push(first_char);

        if maybe_identifier {
            while self
                .char_iterator
                .peek()
                .map_or(false, |c| c.is_alphanumeric())
            {
                cur_lexeme.push(self.char_iterator.next().unwrap());
            }
        } else if is_number {
            while self.char_iterator.peek().map_or(false, |c| c.is_digit(10)) {
                cur_lexeme.push(self.char_iterator.next().unwrap());
            }
            if self
                .char_iterator
                .peek()
                .map_or(false, |c| c.is_alphabetic())
            {
                // Error state : e.g. 1011aaaa is not valid
                cur_lexeme.push(self.char_iterator.next().unwrap());
                eprintln!("Error on line n (line numbers not implemented): {}", cur_lexeme);
                process::exit(1);
            }               
        }

        match cur_lexeme.as_str() {
            "}" => token.category = RightCurlyBrace,
            "{" => token.category = LeftCurlyBrace,
            "int" => token.category = KeywordInt,
            "return" => token.category = KeywordReturn,
            ";" => token.category = SemiColon,
            "+" => token.category = Plus,
            ")" => token.category = RightBracket,
            "(" => token.category = LeftBracket,
            _ => if maybe_identifier || is_number {
                    token.attribute_value = Some(cur_lexeme); 
                    if maybe_identifier {token.category = Identifier; }
                    else {token.category = Number;}
                } else {
                    eprintln!("Error! Unexpected token {} on line n", cur_lexeme);
                    process::exit(1);
                },
        }
        

        return token;
    }
}
