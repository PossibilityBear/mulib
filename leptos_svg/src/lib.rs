use std::{fmt::Display, fs::{self, File}, path::PathBuf};

use proc_macro::TokenStream;

mod parse_svg;
use parse_svg::*;

use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream}, parse_macro_input, token::{At, Paren}, Attribute, Expr, ExprLit, Lit::Str, Token
};


#[proc_macro]
pub fn svg(item: TokenStream) -> TokenStream {
    let svg =  parse_macro_input!(item as Svg);
    todo!()
}
struct Svg {
    // the path to the svg file
    path: FilePath,
    // the svg element parsed from the file
    svg_element: SvgElement,
    // css classes to apply to the svg
    css_classes: Vec<Class>
}

impl Display for Svg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FilePath: {}", self.path)
    }
}

impl Parse for Svg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let path = input.parse::<FilePath>()?;

        let css_classes = parse_zero_or_more::<Class>(input);

        let svg_element = read_svg(&path);

        Ok(Svg {path, svg_element, css_classes})
    }
}

/// helper function to parse zero or more of the generic type T 
/// from the provided ParseStream
fn parse_zero_or_more<T: Parse>(input: ParseStream) -> Vec<T> {
    let mut result = Vec::new();
    
    while let Ok(item) = input.parse::<T>() {
        result.push(item);
    }

    return result
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let svg = read_svg(&FilePath("../public/play.svg".to_string()));
        println!("{}", svg);
    }
}



// ideal syntax svg!("path/to/your.svg", css_class1, css_class2, ...)

    // potentially todo:
    // Create a function like proc macro that 
    // 1. parses svg file at provided path 
    // 2. strips xml info, comments, inline styles, etc
    // 3. takes a vec of classes Strings to apply to svg
    // 4. applies classes to svg
    // 5. processed svg into a view!{} macro for use in leptos

    // this proc macro would save a bunch of manual work, 
    

    // let play_svg = || {view!{
    //     <svg 
    //     class=vec![controls::play_svg, main_style::svg_button].join(" ")
    //     viewBox="0 0 512 512"
    //     version="1.1"
    //     id="svg5"
    //     xmlns="http://www.w3.org/2000/svg"
    //     xmlns:svg="http://www.w3.org/2000/svg">
    //     <defs
    //         id="defs2" />
    //     <g
    //         id="layer1">{}
    //         <path
    //         id="path511"
    //         d="M 416,160 248,256.99485 80,353.98969 80,159.99999 80,-33.98969 248,63.005158 Z"
    //         transform="matrix(1.1572647,0,0,1.1572647,-31.001646,70.837648)" />
    //     </g>
    //     </svg>
    // }};
    // let pause_svg = || {view!{}};

// <?xml version="1.0" encoding="UTF-8" standalone="no"?>
// <svg
//    height="500"
//    width="500"
//    version="1.1"
//    id="Capa_1"
//    viewBox="26.907 -0.841 177.88813 211.25875"
//    preserveAspectRatio="none"
//    sodipodi:docname="play.svg"
//    inkscape:version="1.2.2 (b0a8486541, 2022-12-01)"
//    xmlns:inkscape="http://www.inkscape.org/namespaces/inkscape"
//    xmlns:sodipodi="http://sodipodi.sourceforge.net/DTD/sodipodi-0.dtd"
//    xmlns="http://www.w3.org/2000/svg"
//    xmlns:svg="http://www.w3.org/2000/svg"
//    xmlns:bx="https://boxy-svg.com">
//   <sodipodi:namedview
//      id="namedview378"
//      pagecolor="#505050"
//      bordercolor="#eeeeee"
//      borderopacity="1"
//      inkscape:showpageshadow="0"
//      inkscape:pageopacity="0"
//      inkscape:pagecheckerboard="0"
//      inkscape:deskcolor="#505050"
//      showgrid="false"
//      inkscape:zoom="1.05625"
//      inkscape:cx="399.52663"
//      inkscape:cy="248.04734"
//      inkscape:window-width="1886"
//      inkscape:window-height="1011"
//      inkscape:window-x="0"
//      inkscape:window-y="32"
//      inkscape:window-maximized="1"
//      inkscape:current-layer="Capa_1" />
//   <defs
//      id="defs369">
//     <bx:export>
//       <bx:file
//          format="svg"
//          path="PlayButton.svg"
//          normalization="{&quot;removeBoxySVGMetadata&quot;:true}" />
//     </bx:export>
//   </defs>
//   <g
//      id="g375"
//      transform="matrix(0.62344916,0,0,0.62655892,9.7609509,0)">
//     <g
//        id="g373">
//       <path
//          style="fill:#010002"
//          d="M 27.50192,335.83075 V -1.3422521 L 312.83093,167.24425 Z"
//          id="path371"
//          sodipodi:nodetypes="cccc" />
//     </g>
//   </g>
// </svg>
