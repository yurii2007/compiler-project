pub mod lexer;
pub mod parser;

pub struct Ast {
    pub statements: Vec<ASTStatement>,
}

impl Ast {
    pub fn new() -> Self {
        Self { statements: Vec::new() }
    }

    pub fn add_statement(&mut self, statement: ASTStatement) {
        self.statements.push(statement)
    }

    pub fn visit(&mut self, visitor: &mut dyn ASTVisitor) {
        for statement in self.statements.iter() {
            visitor.visis_statement(statement);
        }
    }
}

pub trait ASTVisitor {
    fn visis_statement(&mut self, statement: &ASTStatement) {
        match &statement.kind {
            ASTStatementKind::Expression(expr) => { self.visit_expression(&expr) }
        }
    }
    fn visit_expression(&mut self, expression: &ASTExpression) {
        match &expression.kind {
            ASTExpressionKind::Number(number) => { self.visit_number(&number) }
        }
    }
    fn visit_number(&mut self, number: &ASTNumberExpression);
}

pub struct ASTPrinter {
    indent: usize,
}

impl ASTVisitor for ASTPrinter {
    fn visit_number(&mut self, number: &ASTNumberExpression) {
        self.print_with_indent(&format!("Number {}", number.number))
    }
}

impl ASTPrinter {
    fn print_with_indent(&mut self, text: &str) {
        println!("{}{}", " ".repeat(self.indent), text)
    }
}

pub enum ASTStatementKind {
    Expression(ASTExpression),
}

pub struct ASTExpression {
    kind: ASTExpressionKind,
}

impl ASTExpression {
    pub fn new(kind: ASTExpressionKind) -> Self {
        ASTExpression { kind }
    }

    pub fn number(number: i64) -> Self {
        ASTExpression::new(ASTExpressionKind::Number(ASTNumberExpression { number }))
    }
}

pub enum ASTExpressionKind {
    Number(ASTNumberExpression),
}

pub struct ASTNumberExpression {
    number: i64,
}

pub struct ASTStatement {
    pub kind: ASTStatementKind,
}

impl ASTStatement {
    pub fn new(kind: ASTStatementKind) -> Self {
        Self { kind }
    }

    pub fn expression(expr: ASTExpression) -> Self {
        ASTStatement::new(ASTStatementKind::Expression(expr))
    }
}
