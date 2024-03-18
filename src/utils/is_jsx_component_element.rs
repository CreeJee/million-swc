use swc_core::ecma::ast::{JSXAttrOrSpread, JSXElement, JSXElementName};

use regex::Regex;

use super::jsx::{is_attribute_name_ref, is_attribute_unsupported};

pub fn is_jsx_component_element(jsx: JSXElement) -> bool {
    let component_name_regex = Regex::new("^[A-Z_]").unwrap();

    let is_member_expr_is_acceptable = match jsx.opening.name {
        JSXElementName::Ident(ident) => {
            let content = ident.sym.to_string();
            //@TODO : 여기서 재귀로 JSXComponentable::JSXAttr을 분기 태우는게 원래의 로직해석에 맞는데 그 방법을 정확히 모르겟음.
            return component_name_regex.is_match(content.as_str());
        }
        JSXElementName::JSXMemberExpr(expr) => true,
        _ => false,
    };
    let is_attr_has_whitelist = jsx.opening.attrs.iter().any(|attr| match attr {
        JSXAttrOrSpread::JSXAttr(attr) => {
            let is_unsupported = is_attribute_unsupported(attr);
            let is_attribute_ref = is_attribute_name_ref(attr);
            return is_unsupported || is_attribute_ref;
        }

        //@TODO: 스팩상 spread element에 대한 처리가 없는데 정말 불가능한가?
        JSXAttrOrSpread::SpreadElement(SpreadElement) => false,
    });

    return is_member_expr_is_acceptable || is_attr_has_whitelist;
}
