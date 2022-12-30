use crate::language::ast::Expression::IntegerLiteral;

#[derive(Debug)]
struct Program {
    definitions: Vec<FunctionDefinition>,
}

#[derive(Debug)]
struct FunctionDefinition {
    name: String,
    args: Vec<String>,
    body: Vec<Expression>,
}

#[derive(Debug)]
enum Expression {
    BinaryExpression {
        operator: Operator,
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    IntegerLiteral(i32),
    Identifier(String),
    Assignment {
        name: String,
        expression: Box<Expression>,
    },
    FunctionCall {
        name: String,
        args: Vec<Box<Expression>>,
    },
}

#[derive(Debug)]
enum Operator {
    Add,
    Mul,
}

#[test]
fn test() {
    let ast: Program = Program {
        definitions: vec![
            FunctionDefinition {
                name: "main".into(),
                args: vec![],
                body: vec![
                    Expression::Assignment { name: "a".into(), expression: IntegerLiteral(1).into() },
                    Expression::Assignment { name: "b".into(), expression: IntegerLiteral(42).into() },
                    Expression::Assignment {
                        name: "c".into(),
                        expression: Expression::FunctionCall {
                            name: "add".into(),
                            args: vec![
                                Expression::Identifier("a".into()).into(),
                                Expression::Identifier("b".into()).into()
                            ]
                        }.into()
                    },
                    Expression::Identifier("c".into()).into()
                ],
            },
            FunctionDefinition {
                name: "add".into(),
                args: vec!["a".into(), "b".into()],
                body: vec![
                    Expression::BinaryExpression {
                        operator: Operator::Add,
                        lhs: Expression::Identifier("a".into()).into(),
                        rhs: Expression::Identifier("b".into()).into(),
                    }
                ],
            },
        ],
    };

    println!("{:?}", ast);
}
