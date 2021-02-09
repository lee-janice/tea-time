use crate::command::Command;

const HELPER_WORDS: [&str; 7] = ["a", "an", "the", "at", "to", "go", "of"];
const PREPOSITIONS: [&str; 4] = ["in", "into", "for", "inside"];
const DIRECTIONS: [&str; 8] = ["north", "n", "south", "s", "east", "e", "west", "w"];

#[derive(Clone, Debug)]
enum Token {
    Verb { value: String },
    Object { value: String },
    Preposition { value: String },
    Direction { value: String },
    EOF,
}

pub struct Parser;

/// Assumes that commands are either a direction
/// or a phrase of the form [verb] [object]
/// or [verb] [object] [prep] [object]
impl Parser {
    pub fn parse(input: &str) -> Result<Command, &'static str> {
        let tokens = Parser::tokenize(Parser::clean(input));
        match Parser::parse_tokens(tokens) {
            Some(tokens) => Ok(tokens),
            None => Err("I didn't get that, come again?"),
        }
    }

    fn clean(input: &str) -> Vec<String> {
        input
            .split_whitespace()
            .map(str::to_lowercase)
            .filter(|word| !(HELPER_WORDS).contains(&word.as_str()))
            .collect()
    }

    fn tokenize(words: Vec<String>) -> Vec<Token> {
        let mut tokens = Vec::new();

        if words.is_empty() {
            return tokens;
        }

        // assume the first word is either a verb or a direction
        let first_word = &words[0];
        if DIRECTIONS.contains(&first_word.as_str()) {
            tokens.push(Token::Direction {
                value: first_word.to_string(),
            });
        } else {
            tokens.push(Token::Verb {
                value: first_word.to_string(),
            });
        }

        // assume the rest of the words are objects/preps/directions
        for word in words[1..].to_vec() {
            if PREPOSITIONS.contains(&word.as_str()) {
                tokens.push(Token::Preposition { value: word });
            } else if DIRECTIONS.contains(&word.as_str()) {
                tokens.push(Token::Direction { value: word });
            } else {
                tokens.push(Token::Object { value: word });
            }
        }

        tokens.push(Token::EOF);
        tokens
    }

    fn parse_tokens(tokens: Vec<Token>) -> Option<Command> {
        if tokens.is_empty() {
            None
        } else {
            let mut cmd_tokens = Command {
                verb: None,
                obj: None,
                prep: None,
                obj_prep: None,
            };
            let token = &tokens[0];
            match token {
                Token::Verb { value } | Token::Direction { value } => {
                    cmd_tokens.verb = Some(value.to_string());
                    Parser::parse_verb(tokens[1..].to_vec(), &mut cmd_tokens)
                }
                _ => None,
            }
        }
    }

    /// Parses remaining tokens when the previous token was a Verb
    fn parse_verb(tokens: Vec<Token>, cmd_tokens: &mut Command) -> Option<Command> {
        let token = &tokens[0];
        match token {
            Token::Object { value } => {
                cmd_tokens.obj = Some(value.to_string());
                Parser::parse_obj(tokens[1..].to_vec(), cmd_tokens)
            }
            Token::EOF => Some(cmd_tokens.clone()),
            _ => None,
        }
    }

    /// Parses remaining tokens when the previous token was an Object
    fn parse_obj(tokens: Vec<Token>, cmd_tokens: &mut Command) -> Option<Command> {
        let token = &tokens[0];
        match token {
            Token::Preposition { value } => {
                cmd_tokens.prep = Some(value.to_string());
                Parser::parse_prep(tokens[1..].to_vec(), cmd_tokens)
            }
            Token::Object { value } => {
                let prev_obj = cmd_tokens.obj.as_ref().unwrap();
                cmd_tokens.obj = Some([prev_obj.to_string(), value.to_string()].join(" "));
                Parser::parse_obj(tokens[1..].to_vec(), cmd_tokens)
            }
            Token::EOF => Some(cmd_tokens.clone()),
            _ => None,
        }
    }

    /// Parses remaining tokens when the previous token was a Preposition
    fn parse_prep(tokens: Vec<Token>, cmd_tokens: &mut Command) -> Option<Command> {
        let token = &tokens[0];
        match token {
            Token::Object { value } => {
                cmd_tokens.obj_prep = Some(value.to_string());
                Parser::parse_obj_prep(tokens[1..].to_vec(), cmd_tokens)
            }
            _ => None,
        }
    }

    /// Parses remaining tokens when the previous token was an Object after Preposition
    fn parse_obj_prep(tokens: Vec<Token>, cmd_tokens: &mut Command) -> Option<Command> {
        let token = &tokens[0];
        match token {
            Token::Object { value } => {
                let prev_obj = cmd_tokens.obj_prep.as_ref().unwrap();
                cmd_tokens.obj_prep = Some([prev_obj.to_string(), value.to_string()].join(" "));
                Parser::parse_obj_prep(tokens[1..].to_vec(), cmd_tokens)
            }
            Token::EOF => Some(cmd_tokens.clone()),
            _ => None,
        }
    }
}
