use swc_core::ecma::ast::{JSXAttr, JSXAttrName};

pub fn is_attribute_unsupported(attribute: JSXAttr) -> bool {
    let unsupported = vec!["tw", "css"];
    return match attribute.name {
        JSXAttrName::Ident(ident) => {
            let sym = ident.sym.to_string();
            unsupported.contains(&sym.as_str())
        }
        _ => false,
    };
}
pub fn is_attribute_name_ref(attribute: JSXAttr) -> bool {
    return match attribute.name {
        JSXAttrName::Ident(ident) => ident.sym.to_string().eq("ref"),
        _ => false,
    };
}
