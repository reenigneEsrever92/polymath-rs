use std::cell::RefCell;

use self::types::*;

pub mod types;

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Token<'a> {
    pub span: Span<'a>,
    pub token_type: TokenType,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Span<'a> {
    pub text: &'a str,
    pub start: usize,
    pub end: usize,
}

impl<'a> Span<'a> {
    pub fn len(&self) -> usize {
        self.end - self.start
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[derive(Debug)]
pub struct Cursor {
    pos: RefCell<usize>,
}

pub fn tokenize(text: &str) -> Vec<Token> {
    let cursor = Cursor {
        pos: RefCell::new(0),
    };
    let mut token_vec = Vec::new();

    while !cursor.eos(text) {
        cursor.skip_whitespace(text);
        token_vec.push(parse_token(&cursor, text))
    }

    token_vec
}

impl Cursor {
    fn get_pos(&self) -> usize {
        *self.pos.borrow()
    }

    fn set_pos(&self, pos: usize) -> usize {
        *self.pos.borrow_mut() = pos;
        self.get_pos()
    }

    fn skip_whitespace(&self, text: &str) {
        while self.parse(text, &[" "]).is_some() {}
    }

    // fn parse<P: TokenParser>(&self, token_parser: P) {
    //     todo!();
    // }

    fn parse_number<'a>(&self, text: &'a str) -> Option<Span<'a>> {
        if let Some(_number) = self.peek(text, &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"])
        {
            let start = self.get_pos();

            while let Some(_number) =
                self.parse(text, &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"])
            {}

            if let Some(_decimal_point) = self.parse(text, &["."]) {
                while let Some(_number) =
                    self.parse(text, &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"])
                {
                }

                Some(Span {
                    start,
                    end: self.get_pos(),
                    text: &text[start..self.get_pos()],
                })
            } else {
                Some(Span {
                    start,
                    end: self.get_pos(),
                    text: &text[start..self.get_pos()],
                })
            }
        } else if let (Some(_decimal_point), Some(_number)) = (
            self.peek(text, &["."]),
            self.peek(text, &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"]),
        ) {
            let _decimal_point = self.parse(text, &["."]).unwrap();
            let start = self.get_pos();

            while let Some(_number) =
                self.parse(text, &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"])
            {}

            let end = self.get_pos();

            Some(Span {
                start,
                end,
                text: &text[start..end],
            })
        } else {
            None
        }
    }

    fn parse_text<'a>(&self, text: &'a str) -> Option<Span<'a>> {
        let start = self.get_pos();
        if let Some(_double_quotes) = self.peek(text, &["\""]) {
            if let Some(next_double_quotes) = self.find_next(text, &["\""], 1) {
                self.set_pos(next_double_quotes + 1);
                Some(Span {
                    start: start + 1,
                    end: next_double_quotes,
                    text: &text[start + 1..next_double_quotes],
                })
            } else {
                None
            }
        } else {
            None
        }
    }

    fn parse_symbol<'a>(&self, text: &'a str) -> Option<Span<'a>> {
        let start = self.get_pos();
        if let Some(c) = text[self.get_pos()..].chars().next() {
            self.set_pos(self.get_pos() + c.len_utf8());

            Some(Span {
                start,
                end: self.get_pos(),
                text: &text[start..self.get_pos()],
            })
        } else {
            None
        }
    }

    fn find_next(&self, text: &str, patterns: &[&str], offset: usize) -> Option<usize> {
        text.char_indices()
            .skip(self.get_pos() + offset)
            .map(|(offset, _)| (offset, &text[offset..]))
            .find(|(_offset, substr)| patterns.iter().any(|pattern| substr.starts_with(*pattern)))
            .map(|(offset, _)| offset)
    }

    fn peek<'a>(&self, text: &'a str, patterns: &[&str]) -> Option<Span<'a>> {
        self.peek_n(text, patterns, 0)
    }

    fn peek_n<'a>(&self, text: &'a str, patterns: &[&str], offset: usize) -> Option<Span<'a>> {
        patterns
            .iter()
            .find(|pattern| text[self.get_pos() + offset..].starts_with(*pattern))
            .map(|pattern| Span {
                start: self.get_pos() + offset,
                end: self.get_pos() + offset + pattern.len(),
                text: &text[self.get_pos() + offset..self.get_pos() + offset + pattern.len()],
            })
    }

    fn parse<'a>(&self, text: &'a str, patterns: &[&str]) -> Option<Span<'a>> {
        let pattern = patterns
            .iter()
            .find(|pattern| text[self.get_pos()..].starts_with(*pattern))
            .map(|pattern| {
                (
                    self.get_pos() + pattern.len(),
                    Span {
                        start: self.get_pos(),
                        end: self.get_pos() + pattern.len(),
                        text: &text[self.get_pos()..self.get_pos() + pattern.len()],
                    },
                )
            });

        if pattern.is_some() {
            self.set_pos(pattern.as_ref().unwrap().0);
        }

        pattern.map(|tuple| tuple.1)
    }

    fn parse_pattern<'a, T: Clone>(
        &self,
        text: &'a str,
        patterns: &[(&[&str], T)],
    ) -> Option<(Span<'a>, T)> {
        let token = patterns.iter().find_map(|patterns| {
            patterns
                .0
                .iter()
                .find(|pattern| text[self.get_pos()..].starts_with(*pattern))
                .map(|pattern| {
                    (
                        Span {
                            start: self.get_pos(),
                            end: self.get_pos() + pattern.len(),
                            text: &text[self.get_pos()..self.get_pos() + pattern.len()],
                        },
                        patterns.1.clone(),
                    )
                })
        });

        if token.is_some() {
            self.set_pos(token.as_ref().unwrap().0.end);
        }

        token
    }

    fn eos(&self, text: &str) -> bool {
        self.get_pos() >= text.len()
    }
}

fn parse_token<'a>(cursor: &Cursor, text: &'a str) -> Token<'a> {
    if let Some(span) = cursor.parse(text, &["/"]) {
        Token {
            span,
            token_type: TokenType::Division,
        }
    } else if let Some(span) = cursor.parse(text, &["_"]) {
        Token {
            span,
            token_type: TokenType::Underscorce,
        }
    } else if let Some(span) = cursor.parse(text, &["^"]) {
        Token {
            span,
            token_type: TokenType::Hat,
        }
    } else if let Some(span) = cursor.parse_number(text) {
        Token {
            span,
            token_type: TokenType::Number,
        }
    } else if let Some(span) = cursor.parse_text(text) {
        Token {
            span,
            token_type: TokenType::Text,
        }
    } else if let Some((span, token_type)) = cursor.parse_pattern(text, UNARY_OPERATORS) {
        Token { span, token_type }
    } else if let Some((span, token_type)) = cursor.parse_pattern(text, BINARY_OPERATORS) {
        Token { span, token_type }
    } else if let Some(arrow) = cursor.parse_pattern(text, ARROWS) {
        Token {
            span: arrow.0,
            token_type: arrow.1,
        }
    } else if let Some(operation) = cursor.parse_pattern(text, OPERATION) {
        Token {
            span: operation.0,
            token_type: operation.1,
        }
    } else if let Some(greek) = cursor.parse_pattern(text, GREEK) {
        Token {
            span: greek.0,
            token_type: greek.1,
        }
    } else if let Some(misc) = cursor.parse_pattern(text, MISC) {
        Token {
            span: misc.0,
            token_type: misc.1,
        }
    } else if let Some(relational) = cursor.parse_pattern(text, RELATIONAL) {
        Token {
            span: relational.0,
            token_type: relational.1,
        }
    } else if let Some(logical) = cursor.parse_pattern(text, LOGICAL) {
        Token {
            span: logical.0,
            token_type: logical.1,
        }
    } else if let Some(function) = cursor.parse_pattern(text, FUNCTION) {
        Token {
            span: function.0,
            token_type: function.1,
        }
    } else if let Some(l_brace) = cursor.parse_pattern(text, LBRACES) {
        Token {
            span: l_brace.0,
            token_type: l_brace.1,
        }
    } else if let Some(l_brace) = cursor.parse_pattern(text, RBRACES) {
        Token {
            span: l_brace.0,
            token_type: l_brace.1,
        }
    } else if let Some(span) = cursor.parse_symbol(text) {
        Token {
            span,
            token_type: TokenType::Symbol,
        }
    } else {
        Token {
            span: Span {
                text: "",
                start: 0,
                end: 0,
            },
            token_type: TokenType::None,
        }
    }
}
