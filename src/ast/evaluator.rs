use super::{ ASTBinaryExpression, ASTBinaryOperatorKind, ASTNumberExpression, ASTVisitor };

pub struct ASTEvaluator {
    pub last_value: Option<i64>,
}

impl ASTEvaluator {
    pub fn new() -> Self {
        Self { last_value: None }
    }
}

impl ASTVisitor for ASTEvaluator {
    fn visit_number(&mut self, number: &ASTNumberExpression) {
        self.last_value = Some(number.number);
    }

    fn visit_binary_expression(&mut self, binary_expression: &ASTBinaryExpression) {
        self.visit_expression(&binary_expression.left);
        let left = self.last_value.unwrap();
        self.visit_expression(&binary_expression.right);
        let right = self.last_value.unwrap();
        self.last_value = Some(match binary_expression.operator.kind {
            ASTBinaryOperatorKind::Plus => left + right,
            ASTBinaryOperatorKind::Minus => left - right,
            ASTBinaryOperatorKind::Multiply => left * right,
            ASTBinaryOperatorKind::Divide => left / right,
        });
    }
}
