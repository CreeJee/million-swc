use swc_core::ecma::ast::*;
use swc_core::ecma::utils::*;

pub fn is_use_client(program: &Program) -> bool {
    return match program.as_module() {
        Some(module) => module.body.iter().any(|i| {
            return match i.as_ref() {
                Some(Stmt::Expr(expr)) => match &*expr.expr {
                    Expr::Lit(Lit::Str(Str {
                        raw: Some(value), ..
                    })) => value == "\"use client\"" || value == "'use client'",
                    _ => false,
                },
                _ => false,
            };
        }),
        _ => false,
    };
}
