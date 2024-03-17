use swc_core::ecma::ast::{JSXAttr, JSXMemberExpr};

use regex::Regex;

pub enum JSXComponentable {
    JSXMemberExpr(JSXMemberExpr),
    JSXAttr(JSXAttr),
}
pub fn is_jsx_component_element(expr: JSXComponentable) -> bool {
    let component_name_regex = Regex::new("^[A-Z_]").unwrap();
    match expr {
        JSXComponentable::JSXAttr(JSXAttr { span, name, value }) => false,
        JSXComponentable::JSXMemberExpr(JSXMemberExpr { obj, prop }) => {
            let content = prop.to_string();
            return component_name_regex.is_match(content.as_str());
        }
    }
}
