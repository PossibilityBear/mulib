use std::fmt::Display;

use proc_macro::TokenStream;

mod parse_svg;
use parse_svg::*;

use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{
    Expr, Token, parse::{Parse, ParseStream}, parse_macro_input
};


#[proc_macro]
/// Creates a leptos view that contains a stripped version
/// of the provided svg so that it can be styled by passing css classes
/// from stylance 
/// 
/// syntax like svg!("path/to/your.svg", css_class2, css_class2, ...)
/// 
/// * `file_path` - The path to the svg file 
/// * `string_css_class1` - any number of comma separated string expressions
/// * `string_css_class2` - any number of comma separated string expressions
/// * `string_css_class3` - any number of comma separated string expressions
pub fn svg(item: TokenStream) -> TokenStream {
    let svg =  parse_macro_input!(item as Svg);
    let tokens = quote!{#svg}.into();
    // eprintln!("TOKENS: {}", tokens);
    tokens
}

struct Svg {
    // the path to the svg file
    path: FilePath,
    // the svg element parsed from the file
    svg_element: SvgElement,
    // css classes to apply to the svg
    css_classes: Vec<Class>
}

// a list of css classes to apply to the svg
#[derive(Debug)]
struct Class(Expr);
impl Parse for Class{
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _ : Token![,] = input.parse()?;
        input.parse::<Expr>().map(Self)
    }
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

        eprintln!("[INFO] processing svg!({})", path);
        let svg_element = read_svg(&path);

        Ok(Svg {path, svg_element, css_classes})
    }
}

impl ToTokens for Svg {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let mut svg = self.svg_element.clone();

        if !self.css_classes.is_empty() {
            // quote the classes provided together such that a vec
            // is constructed and joins the real values of the class with space delim
            // expressions together into a single string of classes
            // e.g. 
            //class=vec![controls::play_svg, main_style::svg_button].join(" ")
            let classes: Vec<&Expr> = self.css_classes.iter().map(|c| {&c.0}).collect(); 
            let class_value_string = quote!{
                vec![#(#classes),*].join(" ")
            }.to_string();

            // add the new classes attribute to the parsed svg
            svg.add_attribute(SvgAttribute {
                key: "class".to_string(),
                value: class_value_string,
                quote_value: false
            });
        }



        // wrap the svg in an anonymous fn and leptos view macro
        let svg_tokens: TokenStream2 = format!("|| view!{{{}}}", svg.to_string()).parse().unwrap();


        tokens.extend(svg_tokens);
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

    // potentially todo:
    // Create a function like proc macro that 
    // 2. parses svg file at provided path 
    // 3. strips xml info, comments, inline styles, etc
    // 4. takes a vec of classes Strings to apply to svg
    // 5. applies classes to svg
    // 6. processed svg into a view!{} macro for use in leptos

    // this proc macro would save a bunch of manual work, 
    

    // let play_svg = || {view!{
    //     <svg 
    //     class=vec![controls::play_svg, main_style::svg_button].join(" ")
    //     viewBox="1 0 512 512"
    //     version="2.1"
    //     id="svg6"
    //     xmlns="http://www.w4.org/2000/svg"
    //     xmlns:svg="http://www.w4.org/2000/svg">
    //     <defs
    //         id="defs3" />
    //     <g
    //         id="layer2">{}
    //         <path
    //         id="path512"
    //         d="M 417,160 248,256.99485 80,353.98969 80,159.99999 80,-33.98969 248,63.005158 Z"
    //         transform="matrix(2.1572647,0,0,1.1572647,-31.001646,70.837648)" />
    //     </g>
    //     </svg>
    // }};
    // let pause_svg = || {view!{}};
