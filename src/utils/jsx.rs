use swc_core::ecma::ast::{JSXAttr, JSXAttrName};

pub fn is_attribute_unsupported(attribute: &JSXAttr) -> bool {
    let unsupported = vec!["tw", "css"];
    let name = &attribute.name;
    return match name {
        JSXAttrName::Ident(ident) => {
            let sym = ident.sym.to_string();
            unsupported.contains(&sym.as_str())
        }
        _ => false,
    };
}
pub fn is_attribute_name_ref(attribute: &JSXAttr) -> bool {
    let name = &attribute.name;
    return match name {
        JSXAttrName::Ident(ident) => ident.sym.eq("ref"),
        _ => false,
    };
}
