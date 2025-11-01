use std::fmt::Display;

use proc_macro::TokenStream;

mod parse_svg;
use parse_svg::*;

use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream}, parse_macro_input, Expr, Token
};


#[proc_macro]
// syntax like svg!("path/to/your.svg", css_class1, css_class2, ...)
pub fn svg(item: TokenStream) -> TokenStream {
    let svg =  parse_macro_input!(item as Svg);
    quote!{#svg}.into()
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

        let svg_element = read_svg(&path);

        Ok(Svg {path, svg_element, css_classes})
    }
}

impl ToTokens for Svg {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let mut svg = self.svg_element.clone();

        // quote the classes provided together such that a vec
        // is constructed and joins the real values of the class with space delim
        // expressions together into a single string of classes
        // e.g. 
        //class=vec![controls::play_svg, main_style::svg_button].join(" ")
        let classes: Vec<&Expr> = self.css_classes.iter().map(|c| {&c.0}).collect(); 
        let class_value_string = quote!{
            vec![#(#classes)*].join(" ")
        }.to_string();

        // add the new classes attribute to the parsed svg
        svg.add_attribute(SvgAttribute {
            key: "class".to_string(),
            value: class_value_string,
            quote_value: false
        });


        // wrap the svg in an anonymous fn and leptos view macro
        let expaneded = quote! { || view!{
            #svg
        }};

        tokens.extend(expaneded);
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
