use ast::{ evaluator::ASTEvaluator, lexer::Lexer, parser::Parser, Ast };

mod ast;

fn main() {
    let input = "(3 + 4) * 7";

    let mut lexer = Lexer::new(input);
    let mut tokens = Vec::new();

    while let Some(token) = lexer.next_token() {
        tokens.push(token);
    }
    println!("${:?}", tokens);

    let mut ast: Ast = Ast::new();
    let mut parser = Parser::new(tokens);
    while let Some(stmt) = parser.next_statement() {
        ast.add_statement(stmt);
    }
    ast.visualize();

    let mut eval = ASTEvaluator::new();
    ast.visit(&mut eval);
    println!("Should be equal to a 35 => {:?}", eval.last_value)
}
