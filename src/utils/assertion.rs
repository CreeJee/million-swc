use swc_core::ecma::ast::Expr;

// check.ts

// isComponent
pub fn is_react_component(node: Box<Expr>) -> bool {
    return node.is_arrow() || node.is_fn_expr();
}
