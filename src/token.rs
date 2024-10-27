#[derive(Clone, Copy, Debug)]
pub enum TokenType {
    Number,
    Comma,
    Caret,
    String,
    Dollar,
    Dot,
    DotDot,
    Name,
    Pipe,
    PipePipe,
    Colon,
    ColonColon,
    Semicolon,
    Plus,
    PlusPlus,
    PlusEquals,
    Dash,
    DashEquals,
    Exclamation,
    Asterisk,
    AsteriskAsterisk,
    AsteriskEquals,
    ForwardSlash,
    ForwardSlashForwardSlash,
    ForwardSlashEquals,
    Equals,
    EqualsEquals,
    EqualsTilde,
    ExclamationTilde,
    ExclamationEquals,
    LParen,
    LSquare,
    LCurly,
    LessThan,
    LessThanEqual,
    RParen,
    RSquare,
    RCurly,
    GreaterThan,
    GreaterThanEqual,
    Ampersand,
    AmpersandAmpersand,
    QuestionMark,
    ThinArrow,
    ThickArrow,
    Newline,
    ErrGreaterThanPipe,
    OutErrGreaterThanPipe,
    OutGreaterThan,
    OutGreaterGreaterThan,
    ErrGreaterThan,
    ErrGreaterGreaterThan,
    OutErrGreaterThan,
    OutErrGreaterGreaterThan,
}

#[derive(Clone, Copy, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub span_start: usize,
    pub span_end: usize,
}

impl Token {
    pub fn is_operator(&self, source: &[u8]) -> bool {
        match &self.token_type {
            TokenType::Name => source == b"and" || source == b"or",
            TokenType::Asterisk
            | TokenType::AsteriskAsterisk
            | TokenType::Dash
            | TokenType::EqualsEquals
            | TokenType::ExclamationEquals
            | TokenType::ForwardSlash
            | TokenType::LessThan
            | TokenType::LessThanEqual
            | TokenType::Plus
            | TokenType::PlusPlus
            | TokenType::GreaterThan
            | TokenType::GreaterThanEqual
            | TokenType::AmpersandAmpersand
            | TokenType::PipePipe
            | TokenType::Equals
            | TokenType::PlusEquals
            | TokenType::DashEquals
            | TokenType::AsteriskEquals
            | TokenType::ForwardSlashEquals => true,
            _ => false,
        }
    }

    pub fn is_comma(&self) -> bool {
        matches!(self.token_type, TokenType::Comma)
    }

    // pub fn is_lcurly(&mut self) -> bool {
    //     matches!(
    //         self.peek(),
    //         Some(Token {
    //             token_type: TokenType::LCurly,
    //             ..
    //         })
    //     )
    // }

    // pub fn is_lcurly2(&self) -> bool {
    //     matches!(
    //         self.next_token,
    //         Some(Token {
    //             token_type: TokenType::LCurly,
    //             ..
    //         })
    //     )
    // }

    // pub fn is_rcurly(&mut self) -> bool {
    //     matches!(
    //         self.peek(),
    //         Some(Token {
    //             token_type: TokenType::RCurly,
    //             ..
    //         })
    //     )
    // }

    // pub fn is_rcurly2(&self) -> bool {
    //     matches!(
    //         self.next_token,
    //         Some(Token {
    //             token_type: TokenType::RCurly,
    //             ..
    //         })
    //     )
    // }

    // pub fn is_lparen(&mut self) -> bool {
    //     matches!(
    //         self.peek(),
    //         Some(Token {
    //             token_type: TokenType::LParen,
    //             ..
    //         })
    //     )
    // }

    // pub fn is_lparen2(&self) -> bool {
    //     matches!(
    //         self.next_token,
    //         Some(Token {
    //             token_type: TokenType::LParen,
    //             ..
    //         })
    //     )
    // }

    // pub fn is_rparen(&mut self) -> bool {
    //     matches!(
    //         self.peek(),
    //         Some(Token {
    //             token_type: TokenType::RParen,
    //             ..
    //         })
    //     )
    // }

    // pub fn is_lsquare(&mut self) -> bool {
    //     matches!(
    //         self.peek(),
    //         Some(Token {
    //             token_type: TokenType::LSquare,
    //             ..
    //         })
    //     )
    // }

    // pub fn is_lsquare2(&self) -> bool {
    //     matches!(
    //         self.next_token,
    //         Some(Token {
    //             token_type: TokenType::LSquare,
    //             ..
    //         })
    //     )
    // }

    pub fn is_rsquare(&self) -> bool {
        matches!(self.token_type, TokenType::RSquare)
    }

    // pub fn is_less_than(&mut self) -> bool {
    //     matches!(
    //         self.peek(),
    //         Some(Token {
    //             token_type: TokenType::LessThan,
    //             ..
    //         })
    //     )
    // }

    // pub fn is_greater_than(&mut self) -> bool {
    //     matches!(
    //         self.peek(),
    //         Some(Token {
    //             token_type: TokenType::GreaterThan,
    //             ..
    //         })
    //     )
    // }

    // pub fn is_pipe(&mut self) -> bool {
    //     matches!(
    //         self.peek(),
    //         Some(Token {
    //             token_type: TokenType::Pipe,
    //             ..
    //         })
    //     )
    // }

    // pub fn is_dollar(&mut self) -> bool {
    //     matches!(
    //         self.peek(),
    //         Some(Token {
    //             token_type: TokenType::Dollar,
    //             ..
    //         })
    //     )
    // }

    // pub fn is_dollar2(&mut self) -> bool {
    //     matches!(
    //         self.next_token,
    //         Some(Token {
    //             token_type: TokenType::Dollar,
    //             ..
    //         })
    //     )
    // }

    // pub fn is_question_mark(&mut self) -> bool {
    //     matches!(
    //         self.peek(),
    //         Some(Token {
    //             token_type: TokenType::QuestionMark,
    //             ..
    //         })
    //     )
    // }

    // pub fn is_thin_arrow(&mut self) -> bool {
    //     matches!(
    //         self.peek(),
    //         Some(Token {
    //             token_type: TokenType::ThinArrow,
    //             ..
    //         })
    //     )
    // }

    // pub fn is_thick_arrow(&mut self) -> bool {
    //     matches!(
    //         self.peek(),
    //         Some(Token {
    //             token_type: TokenType::ThickArrow,
    //             ..
    //         })
    //     )
    // }

    // // pub fn is_double_pipe(&mut self) -> bool {
    // //     matches!(
    // //         self.peek(),
    // //         Some(Token {
    // //             token_type: TokenType::PipePipe,
    // //             ..
    // //         })
    // //     )
    // // }

    // // pub fn is_double_ampersand(&mut self) -> bool {
    // //     matches!(
    // //         self.peek(),
    // //         Some(Token {
    // //             token_type: TokenType::AmpersandAmpersand,
    // //             ..
    // //         })
    // //     )
    // // }

    // // pub fn is_dash(&mut self) -> bool {
    // //     matches!(
    // //         self.peek(),
    // //         Some(Token {
    // //             token_type: TokenType::Dash,
    // //             ..
    // //         })
    // //     )
    // // }

    // pub fn is_colon(&mut self) -> bool {
    //     matches!(
    //         self.peek(),
    //         Some(Token {
    //             token_type: TokenType::Colon,
    //             ..
    //         })
    //     )
    // }

    pub fn is_newline(&self) -> bool {
        matches!(self.token_type, TokenType::Newline)
    }

    pub fn is_semicolon(&self) -> bool {
        matches!(self.token_type, TokenType::Semicolon)
    }

    // pub fn is_semicolon2(&self) -> bool {
    //     matches!(
    //         self.next_token,
    //         Some(Token {
    //             token_type: TokenType::Semicolon,
    //             ..
    //         })
    //     )
    // }

    // pub fn is_dot(&mut self) -> bool {
    //     matches!(
    //         self.peek(),
    //         Some(Token {
    //             token_type: TokenType::Dot,
    //             ..
    //         })
    //     )
    // }

    // pub fn is_dot2(&self) -> bool {
    //     matches!(
    //         self.next_token,
    //         Some(Token {
    //             token_type: TokenType::Dot,
    //             ..
    //         })
    //     )
    // }

    // pub fn is_dotdot(&mut self) -> bool {
    //     matches!(
    //         self.peek(),
    //         Some(Token {
    //             token_type: TokenType::DotDot,
    //             ..
    //         })
    //     )
    // }

    // pub fn is_dotdot2(&self) -> bool {
    //     matches!(
    //         self.next_token,
    //         Some(Token {
    //             token_type: TokenType::DotDot,
    //             ..
    //         })
    //     )
    // }

    // pub fn is_coloncolon(&mut self) -> bool {
    //     matches!(
    //         self.peek(),
    //         Some(Token {
    //             token_type: TokenType::ColonColon,
    //             ..
    //         })
    //     )
    // }

    // pub fn is_number(&mut self) -> bool {
    //     matches!(
    //         self.peek(),
    //         Some(Token {
    //             token_type: TokenType::Number,
    //             ..
    //         })
    //     )
    // }

    // pub fn is_number2(&self) -> bool {
    //     matches!(
    //         self.next_token,
    //         Some(Token {
    //             token_type: TokenType::Number,
    //             ..
    //         })
    //     )
    // }

    // pub fn is_string(&mut self) -> bool {
    //     matches!(
    //         self.peek(),
    //         Some(Token {
    //             token_type: TokenType::String,
    //             ..
    //         })
    //     )
    // }

    // pub fn is_string2(&self) -> bool {
    //     matches!(
    //         self.next_token,
    //         Some(Token {
    //             token_type: TokenType::String,
    //             ..
    //         })
    //     )
    // }

    pub fn is_keyword(&self, keyword: &[u8], source: &[u8]) -> bool {
        matches!(self.token_type, TokenType::Name if source == keyword)
    }

    // pub fn is_keyword2(&self, keyword: &[u8]) -> bool {
    //     let _span = span!();
    //     matches!(
    //         self.next_token,
    //         Some(Token {
    //             token_type: TokenType::Name,
    //             span_start,
    //             span_end,
    //         }) if &self.compiler.source[span_start..span_end] == keyword
    //     )
    // }

    // pub fn is_name(&mut self) -> bool {
    //     matches!(
    //         self.peek(),
    //         Some(Token {
    //             token_type: TokenType::Name,
    //             ..
    //         })
    //     )
    // }

    // pub fn is_bareword(&mut self, name_strictness: NameStrictness) -> bool {
    //     matches!(
    //         self.peek_bareword(name_strictness),
    //         Some(Token {
    //             token_type: TokenType::Name,
    //             ..
    //         })
    //     )
    // }

    // pub fn is_bareword2(&mut self, name_strictness: NameStrictness) -> bool {
    //     matches!(
    //         self.next_token,
    //         Some(Token {
    //             token_type: TokenType::Name,
    //             ..
    //         })
    //     )
    // }

    pub fn is_expression(&self, source: &[u8]) -> bool {
        self.is_simple_expression(source)
            || self.is_keyword(b"if", source)
            || self.is_keyword(b"match", source)
            || self.is_keyword(b"where", source)
    }

    pub fn is_simple_expression(&self, source: &[u8]) -> bool {
        match self.token_type {
            TokenType::Number
            | TokenType::String
            | TokenType::LCurly
            | TokenType::LSquare
            | TokenType::LParen
            | TokenType::Dot
            | TokenType::Dollar => true,
            _ => {
                self.is_keyword(b"true", source)
                    | self.is_keyword(b"false", source)
                    | self.is_keyword(b"null", source)
            }
        }
    }

    pub fn is_value(&self) -> bool {
        match self.token_type {
            TokenType::Number
            | TokenType::String
            | TokenType::LCurly
            | TokenType::LSquare
            | TokenType::LParen
            | TokenType::Dot
            | TokenType::Dollar
            | TokenType::Name => true,
            _ => false,
        }
    }
}
