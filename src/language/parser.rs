use super::ast::*;

peg::parser! {
    grammar l5parser() for str {

        // utils

        rule w() -> () = [' ' | '\n' | '\t']+ { () }

        rule remove_ws<T>(parser: rule<T>) -> T = w()? res:parser() w()? { res }

        rule lower_chars() -> String = str:remove_ws(<$(['a'..='z']+)>) { str.to_string() }

        // Program

        pub rule program() -> Program
            = fns: remove_ws(<function_definition()>)* { Program { definitions: fns } }

        // FunctionDefinition

        rule function_definition() -> FunctionDefinition
            = "function" name:lower_chars() "(" args:(lower_chars() ** ",") ")" w()? "{"
                body: remove_ws(<expression()>)+
            "}" { FunctionDefinition { name, args, body} }

        // Expression

        pub rule expression() -> Expression = assignment() / additive()

        rule additive() -> Expression
            = x:remove_ws(<multitive()>) ys:(remove_ws(<"+">) y:remove_ws(<multitive()>) { y })* {
                ys.iter().fold(x, |expr, y| { Expression::BinaryExpression { operator: Operator::Add, lhs: Box::new(expr), rhs: Box::new(y.clone()) } } )
            }

        rule multitive() -> Expression
            = x:remove_ws(<primary()>) ys:(remove_ws(<"*">) y:remove_ws(<primary()>) { y })* {
                ys.iter().fold(x, |expr, y| { Expression::BinaryExpression { operator: Operator::Mul, lhs: Box::new(expr), rhs: Box::new(y.clone()) } } )
            }

        rule primary() -> Expression
            = (remove_ws(<"(">) e:remove_ws(<additive()>) remove_ws(<")">) { e })
              / integer_literal()
              / function_call()
              / identifier()

        rule integer_literal() -> Expression
            = n:remove_ws(<$(['0'..='9']+)>) {? n.parse().map(|num| Expression::IntegerLiteral(num)).or(Err("error invalid num.")) }

        rule identifier() -> Expression
            = str:lower_chars() { Expression::Identifier(str) }

        rule assignment() -> Expression
            = name:lower_chars() "=" expr:expression() ";" w()? { Expression::Assignment { name, expression: Box::new(expr) } }

        rule function_call() -> Expression
            = name:lower_chars() "(" args:((expr:expression() { expr.into() }) ** ",") ")" w()? { Expression::FunctionCall { name, args } }
    }

}

#[test]
fn test() {

    assert_eq!(
        l5parser::expression("42"),
        Ok(Expression::IntegerLiteral(42))
    );

    assert_eq!(
        l5parser::expression(" ( 1+2      *3)  "),
        Ok(Expression::BinaryExpression {
            operator: Operator::Add,
            lhs: Expression::IntegerLiteral(1).into(),
            rhs: Expression::BinaryExpression {
                operator: Operator::Mul,
                lhs: Expression::IntegerLiteral(2).into(),
                rhs: Expression::IntegerLiteral(3).into()
            }.into()
        })
    );

    assert_eq!(
        l5parser::program(
            "function main() {
                a = 1;
                b = 42;
                c = add(a, b);
                c
            }

            function add(a, b) {
                a + b
            }"),
        Ok(Program {
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
        })
    );

}
