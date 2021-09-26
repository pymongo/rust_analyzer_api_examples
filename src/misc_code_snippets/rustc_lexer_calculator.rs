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

/*
parser 思路:
1. tokens -> TokenTree
2. 先转换成中缀表达式/逆波兰表达式
3. AST
*/
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
