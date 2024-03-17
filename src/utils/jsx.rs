use swc_core::ecma::ast::{JSXAttr, JSXAttrName};

pub fn is_attribute_unsupported(attribute: &JSXAttr) -> bool {
    let unsupported = vec!["tw", "css"];
    let attribute_name = &attribute.name;
    return match attribute_name {
        JSXAttrName::Ident(ident) => {
            let sym = ident.sym.to_string();
            unsupported.contains(&sym.as_str())
        }
        _ => false,
    };
}
pub fn is_attribute_name_ref(attribute: &JSXAttr) -> bool {
    let attribute_name = &attribute.name;
    return match attribute_name {
        JSXAttrName::Ident(ident) => ident.sym.eq("ref"),
        _ => false,
    };
}
