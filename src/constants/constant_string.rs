pub const RENDER_SCOPE: &'static str = "slot";
pub const SKIP_ANNOTATION: &'static str = "@million skip";
pub const SVG_ELEMENTS: &[&'static str] = &[
    "circle",
    "ellipse",
    "foreignObject",
    "image",
    "line",
    "path",
    "polygon",
    "polyline",
    "rect",
    "text",
    "textPath",
    "tspan",
    "svg",
    "g",
];

pub const NO_PX_PROPERTIES: &[&'static str] = &[
    "animationIterationCount",
    "boxFlex",
    "boxFlexGroup",
    "boxOrdinalGroup",
    "columnCount",
    "fillOpacity",
    "flex",
    "flexGrow",
    "flexPositive",
    "flexShrink",
    "flexNegative",
    "flexOrder",
    "fontWeight",
    "lineClamp",
    "lineHeight",
    "opacity",
    "order",
    "orphans",
    "stopOpacity",
    "strokeDashoffset",
    "strokeOpacity",
    "strokeWidth",
    "tabSize",
    "widows",
    "zIndex",
    "zoom",
    // SVG-related properties
    "fillOpacity",
    "floodOpacity",
    "stopOpacity",
    "strokeDasharray",
    "strokeDashoffset",
    "strokeMiterlimit",
    "strokeOpacity",
    "strokeWidth",
];

// pub const JSON = json!{
//     client: {
//         source: "million/react-server",
//         target: "million/react",
//     },
//     server: {
//         source: "million/react",
//         target: "million/react-server",
//     }
// };

// https://www.codeconvert.ai/typescript-to-rust-converter
// https://doc.rust-kr.org/ch06-02-match.html
