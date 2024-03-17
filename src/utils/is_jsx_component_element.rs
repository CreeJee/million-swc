use swc_core::ecma::ast::{JSXAttr, JSXMemberExpr};

use regex::Regex;

use super::jsx::{is_attribute_name_ref, is_attribute_unsupported};

pub enum JSXComponentable {
    JSXMemberExpr(JSXMemberExpr),
    JSXAttr(JSXAttr),
}
pub fn is_jsx_component_element(expr: JSXComponentable) -> bool {
    let component_name_regex = Regex::new("^[A-Z_]").unwrap();
    match expr {
        JSXComponentable::JSXAttr(attr) => {
            let is_unsupported = is_attribute_unsupported(attr.clone());
            let is_attribute_ref = is_attribute_name_ref(attr.clone());
            return is_unsupported || is_attribute_ref;
        }
        JSXComponentable::JSXMemberExpr(expr) => {
            let content = expr.prop.to_string();
            //@TODO : 여기서 재귀로 JSXComponentable::JSXAttr을 분기 태우는게 원래의 로직해석에 맞는데 그 방법을 정확히 모르겟음.
            return component_name_regex.is_match(content.as_str());
        }
    }
}
