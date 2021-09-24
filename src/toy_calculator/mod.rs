//! toy_calculator inspire by rust-analyzer source code

use rustc_lexer::{TokenKind, LiteralKind, Base};

enum BinaryOperand {
    Add,
    Sub,
    Mul,
    Div
}

enum MyToken {
    BinaryOp(BinaryOperand),
    Num(u32),
}

enum AstExpr {
    ArithmeticExpr {
        lhs: Box<AstExpr>,
        binary_operand: BinaryOperand,
        rhs: Box<AstExpr>
    },
    LiteralNumber(u32),
}

#[test]
fn test_rustc_tokenize() {
    let tokens = rustc_lexer::tokenize("fn foo(){}").collect::<Vec<_>>();
    dbg!(tokens);
}

/// current only support u32 type and add/sub/mul/div operand
fn calculator_eval(expr: &str) {
    // filter Vec<rustc::Token> to Vec<MyToken>
    let mut tokens = vec![];
    let mut offset = 0;
    for token in rustc_lexer::tokenize(expr) {
        let my_token = match token.kind {
            TokenKind::Literal { kind: LiteralKind::Int { base: Base::Decimal, .. }, .. } => {
                let num = expr[offset..offset+token.len].parse::<u32>().unwrap();
                MyToken::Num(num)
            },
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

#[test]
fn test_calculator() {
    let expr = "1 + 2 * 3";
    calculator_eval("1+2*3");
}
