use swc_core::ecma::ast::{JSXAttr, JSXAttrName};

pub fn is_attribute_unsupported(attribute: JSXAttr) -> bool {
    let unsupported = vec!["tw", "css", "ref"];
    return match attribute.name {
        JSXAttrName::JSXNamespacedName(namespace) => {
            let sym = namespace.name.sym.to_string();
            unsupported.contains(&sym.as_str())
        }
        JSXAttrName::Ident(ident) => {
            let sym = ident.sym.to_string();
            unsupported.contains(&sym.as_str())
        }
    };
}
