#[derive(Debug, PartialEq, Clone)]
pub struct Program {
    pub definitions: Vec<FunctionDefinition>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionDefinition {
    pub name: String,
    pub args: Vec<String>,
    pub body: Vec<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
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

#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
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
                    Expression::Assignment { name: "a".into(), expression: Expression::IntegerLiteral(1).into() },
                    Expression::Assignment { name: "b".into(), expression: Expression::IntegerLiteral(42).into() },
                    Expression::Assignment {
                        name: "c".into(),
                        expression: Expression::FunctionCall {
                            name: "add".into(),
                            args: vec![
                                Expression::Identifier("a".into()).into(),
                                Expression::Identifier("b".into()).into(),
                            ],
                        }.into(),
                    },
                    Expression::Identifier("c".into()).into(),
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

#[test]
fn test2() {
    let x = Expression::IntegerLiteral(1);
    let y: Vec<Expression> = vec![
        Expression::IntegerLiteral(2),
        Expression::IntegerLiteral(3),
    ];

    // (1, [2, 3]) -> add(1, add(2, 3))

    let _z: Expression = y.iter().fold(
        x,
        |expr, y| {
            Expression::BinaryExpression {
                operator: Operator::Add,
                lhs: Box::new(expr),
                rhs: Box::new(y.clone()),
            }
        });

}