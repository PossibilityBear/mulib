use std::fmt::Display;

use proc_macro::TokenStream;

mod parse_svg;
use parse_svg::*;

use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use syn::{
    Expr, Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
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
    let svg = parse_macro_input!(item as Svg);
    let tokens = quote! {#svg}.into();
    // eprintln!("TOKENS: {}", tokens);
    tokens
}

struct Svg {
    // the path to the svg file
    path: FilePath,
    // the svg element parsed from the file
    svg_element: SvgElement,
    // css classes to apply to the svg
    css_classes: Vec<Class>,
}

// a list of css classes to apply to the svg
#[derive(Debug)]
struct Class(Expr);
impl Parse for Class {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _: Token![,] = input.parse()?;
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

        Ok(Svg {
            path,
            svg_element,
            css_classes,
        })
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
            let classes: Vec<&Expr> = self.css_classes.iter().map(|c| &c.0).collect();
            let class_value_string = quote! {
                vec![#(#classes),*].join(" ")
            }
            .to_string();

            // add the new classes attribute to the parsed svg
            svg.add_attribute(SvgAttribute {
                key: "class".to_string(),
                value: class_value_string,
                quote_value: false,
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

    return result;
}
