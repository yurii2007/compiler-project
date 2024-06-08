use super::{ lexer::{ Lexer, Token, TokenKind }, ASTExpression, ASTStatement };

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new() -> Self {
        Self { tokens: Vec::new(), current: 0 }
    }

    pub fn from_input(input: &str) -> Self {
        let mut lexer = Lexer::new(input);
        let mut tokens = Vec::new();
        while let Some(token) = lexer.next_token() {
            tokens.push(token);
        }
        Self { tokens, current: 0 }
    }

    pub fn nex_statement(&mut self) -> Option<ASTStatement> {
        return self.parse_statement();
    }

    fn peek(&self, offset: usize) -> Option<&Token> {
        self.tokens.get(self.current + offset)
    }

    fn current(&self) -> Option<&Token> {
        self.peek(0)
    }

    fn parse_statement(&mut self) -> Option<ASTStatement> {
        let expr = self.parse_expression()?;
        return Some(ASTStatement::expression(expr));
    }

    fn parse_expression(&mut self) -> Option<ASTExpression> {
        let token = self.current()?;
        match token.kind {
            TokenKind::Number(number) => {
                return Some(ASTExpression::number(number));
            }
            _ => {
                return None;
            }
        }
    }
}