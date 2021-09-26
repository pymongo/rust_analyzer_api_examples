//! toy_calculator inspire by rust-analyzer source code

use rustc_lexer::{Base, LiteralKind, TokenKind};

fn calculator_eval(expr: &str) -> i32 {
    let mut last_operand = TokenKind::Plus;
    let mut stack = vec![];
    let mut offset = 0;
    for token in rustc_lexer::tokenize(expr) {
        match token.kind {
            TokenKind::Whitespace => {}
            TokenKind::Literal {
                kind:
                    LiteralKind::Int {
                        base: Base::Decimal,
                        ..
                    },
                ..
            } => {
                let rhs = expr[offset..offset + token.len].parse::<i32>().unwrap();
                match last_operand {
                    TokenKind::Plus => stack.push(rhs),
                    TokenKind::Minus => stack.push(-rhs),
                    TokenKind::Star => *stack.last_mut().unwrap() *= rhs,
                    TokenKind::Slash => *stack.last_mut().unwrap() /= rhs,
                    _ => {}
                }
            }
            _ => last_operand = token.kind,
        }
        offset += token.len;
    }
    stack.into_iter().sum()
}

#[test]
fn test_calculator_eval() {
    const TEST_CASES: [(&str, i32); 1] = [("1 + 2 * 3", 7)];
    for (input, output) in TEST_CASES {
        assert_eq!(calculator_eval(input), output);
    }
}

// enum BinaryOperand {
//     Add,
//     Sub,
//     Mul,
//     Div
// }

// enum MyToken {
//     BinaryOp(BinaryOperand),
//     Num(u32),
// }

// enum AstExpr {
//     ArithmeticExpr {
//         lhs: Box<AstExpr>,
//         binary_operand: BinaryOperand,
//         rhs: Box<AstExpr>
//     },
//     LiteralNumber(u32),
// }

#[test]
fn test_rustc_tokenize() {
    let tokens = rustc_lexer::tokenize("fn foo(){}").collect::<Vec<_>>();
    dbg!(tokens);
}

/// current only support u32 type and add/sub/mul/div operand
#[cfg(not)]
fn calculator_eval2(expr: &str) {
    // filter Vec<rustc::Token> to Vec<MyToken>
    let mut tokens = vec![];
    let mut offset = 0;
    for token in rustc_lexer::tokenize(expr) {
        let my_token = match token.kind {
            TokenKind::Literal {
                kind:
                    LiteralKind::Int {
                        base: Base::Decimal,
                        ..
                    },
                ..
            } => {
                let num = expr[offset..offset + token.len].parse::<u32>().unwrap();
                MyToken::Num(num)
            }
            TokenKind::Plus => MyToken::BinaryOp(BinaryOperand::Add),
            TokenKind::Minus => MyToken::BinaryOp(BinaryOperand::Sub),
            TokenKind::Star => MyToken::BinaryOp(BinaryOperand::Mul),
            TokenKind::Slash => MyToken::BinaryOp(BinaryOperand::Div),
            _ => {
                offset += token.len;
                continue;
            }
        };
        offset += token.len;
        tokens.push(my_token);
    }
    // let mut stack = vec![];
    // let mut sign = 1;
    // for token in tokens {
    //     match token {
    //         MyToken::BinaryOp(operand) => match operand {

    //         },
    //         MyToken::Num(num) => stack.push(num * sign),
    //     }
    // }
    // let b = stack.into_iter().sum::<u32>();

    // parse Vec<MyToken>
    // 1. parse all Mul/Div
    // 2. parse all Add/Sub
    // let len = tokens.len();
    // let mut visited = vec![false];
    // 逆波兰表达树，左右子树
}
