use std::{fmt::Display, fs};

use quote::ToTokens;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream}, ExprLit, Lit::Str 
};


#[derive(Debug, Clone)]
pub struct SvgElement {
    tag: String,
    attributes: Vec<SvgAttribute>,
    children: Vec<SvgElement>,
    bodyless: bool
}

fn spaces(x: u32) -> String {
    let mut spaces = "".to_string();
    for _ in 0..x {
        spaces.push(' ');
    } ;
    spaces
}

impl SvgElement {
    const NUM_SPACES: u32 = 2;
    fn to_string(element: &SvgElement, depth: u32) -> String {
        let mut res = "".to_string();
        res.push_str(
            &format!("{}<{} ", spaces(depth), element.tag)
        );
        for attr in element.attributes.iter() {
            res.push_str(
                &format!("\n  {}{}", spaces(depth), attr)
            );
        }
        if element.bodyless {
            res.push_str("/>\n");
        } else {
            res.push_str(">\n");

            for child in element.children.iter() {
                res.push_str(
                    &Self::to_string(child, depth + Self::NUM_SPACES)
                );
            }

            res.push_str(
                &format!("{}</{} >", spaces(depth), element.tag)
            );
        }
        res.push('\n');
        res
    }

    pub fn add_attribute(&mut self, attr: SvgAttribute) {
        self.attributes.push(attr);
    }
}

impl Display for SvgElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "SVG ELEMENT:\n {}", Self::to_string(self, 0))?;
        std::fmt::Result::Ok(())
    }
}

impl ToTokens for SvgElement {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let tag = &self.tag;
        let attributes = &self.attributes;
        let children = &self.children;

        if self.bodyless {
            let expanded = quote!{
                <#tag #(#attributes)*//>
            };
            tokens.extend(expanded);
        } else {
            let expanded = quote!{
                <#tag #(#attributes)*>
                    #(#children)*
                </#tag>
            };
            tokens.extend(expanded);
        }
    }
}

#[derive(Debug, Clone)]
pub struct SvgAttribute{
    pub(crate) key: String,
    pub(crate) value: String,
    pub(crate) quote_value: bool,
}

impl Display for SvgAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.quote_value {
            write!(f, "{}=\"{}\"", self.key, self.value)
        } else {
            write!(f, "{}={}", self.key, self.value)
        }
    }
}

impl ToTokens for SvgAttribute {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let key = &self.key;
        let value = &self.value;
        if self.quote_value {
            let expanded = quote!{
                #key="#value"
            };
            tokens.extend(expanded);
        } else {
            let expanded = quote!{
                #key=#value
            };
            tokens.extend(expanded);
        }
    }
}
//Blacklist of attributes that will break things
//if they are not scrubbed, primarily for sizing in css
pub fn is_banned_attribute(key: &str) -> bool{
    // SVG files are technically XML which can be case sensitive 
    // for attribute names, while we are stuffing this in HTML
    // where it is not it's important for the names here 
    // to match the case expected in XML definitions
    let ban_list: Vec<&str> = vec![
        "height",
        "width",
    ];

    ban_list.contains(&key)
}

// the path to the svg file
#[derive(Debug)]
pub struct FilePath(pub String);
impl Display for FilePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Parse for FilePath {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let path = match input.parse::<ExprLit>()?.lit {
            Str(lit) => lit.value(),
            _ => panic!("error processing svg! macro: \
                        path expression must be a str literal")
        };

        Ok(Self(path))
    }
}


/// reads svg file from disk and returns the parsed svg
pub fn read_svg(path: &FilePath) -> SvgElement{
    let mut contents = match fs::read_to_string(path.0.clone()) {
        Ok(c) => c,
        Err(e) => {
            println!("{:?}", e);
            panic!("error parsing svg for svg macro");
        }
        

    };
    
    // remove xml tags that don't work within leptos HTML
    remove_xml_tags(&mut contents);

    // parse the actual svg from the string
    (*parse_elements(&mut contents, "")
        .first()
        .expect("to find some SvgElement"))
        .clone()
}

const XML_OPEN_DELIM: &str = "<?";
const XML_CLOSE_DELIM: &str = "?>";

const OPEN_TAG_OPEN_DELIM: &str = "<";
const CLOSE_TAG_OPEN_DELIM: &str = "</";

const TAG_CLOSE_DELIM: &str = ">";
const BODYLESS_TAG_CLOSE_DELIM: &str = "/>";

/// helper function to find the first of a set of patterns in a string
/// returns the matched pattern and the index matched at or None if no matches
fn first_of(contents: &str, patterns: Vec<&str>) -> Option<(String, usize)> {
    patterns.iter().filter_map(|pat| {
           contents.find(pat).map(|i| ((*pat).to_owned(), i))
        })
        .min_by(|(_, index_a), (_, index_b)| {
            index_a.cmp(index_b)
        })
}

fn find_whitespace(contents: &str) -> Option<usize> {
    for (i, c) in contents.chars().enumerate() {
        if c.is_whitespace() {
            return Some(i)
        }
    }
    None
}

fn first_of_or_whitespace(contents: &str, patterns: Vec<&str>) -> Option<(String, usize)> {
    let whitespace_index = find_whitespace(contents);
    if let Some((first_pat, pat_index)) = first_of(contents, patterns) {
        if let Some(whitespace_index) = whitespace_index && whitespace_index <= pat_index {
            return Some(("".to_string(), whitespace_index));
        }
        return Some((first_pat, pat_index));
    } 
    if let Some(whitespace_index) = whitespace_index {
        return Some(("".to_string(), whitespace_index));
    }
    None
}


/// helper fuction to pop the name of the next tag
fn pop_tag_name(contents: &mut String) -> String {
    *contents = contents.trim().to_string();
    let (_, name_end) = first_of_or_whitespace(&contents, vec!(TAG_CLOSE_DELIM))
        .expect("to have found a tag with a name when parsing svg for macro");

    let name = (&contents[..name_end]).to_owned();
    *contents = (&contents[name_end..]).to_owned();
    name
}

/// helper fuction to peek the name of the next tag
fn peek_tag_name(contents: &String) -> String {
    let contents = contents.trim().to_string();
    let (_, name_end) = first_of_or_whitespace(&contents, vec!(TAG_CLOSE_DELIM))
        .expect("to have found a tag with a name when parsing svg for macro");

    // find end index of tag open delimeter to remove it from the tag name
    let end_open_delim = match first_of(&contents, vec![
        CLOSE_TAG_OPEN_DELIM,
        OPEN_TAG_OPEN_DELIM,
    ]) {
        Some((pat, i)) => pat.len() + i,
        None => 0
    };

    let name = (contents[end_open_delim..name_end]).to_owned();
    name
}




/// helper function to pop the next set of attributes
fn pop_attributes(contents: &mut String) -> Vec<SvgAttribute> {
    // get end index of attributes
    let (_, attr_end_i) = first_of(&contents, vec![
            TAG_CLOSE_DELIM,
            BODYLESS_TAG_CLOSE_DELIM
        ])
        .expect("error mismatched opening and closing tag when parsing svg for macro");

    // split the attributes by whitespace to parse individualy
    let content_attrs = (&contents[..attr_end_i]).trim();

    enum AttrComp {
        Key(String), 
        EQ,
        OpenQt,
        Value(String),
        CloesQt,
    }

    const EQ: char = '=' ;
    // open and closed quotes have same char representation
    const QT: char = '"';

    let mut cur_comp: AttrComp = AttrComp::Key(String::new());

    let mut cur_key: String = String::new();

    let mut attrs: Vec<SvgAttribute> = Vec::new();

    for (_i, c) in content_attrs.chars().enumerate() {
        // print!("{}", c);
        match cur_comp {
            AttrComp::Key(ref s) => {
                // we can ignore whitespace for parsing keys
                if c.is_whitespace() {continue}
                match c {
                    EQ => {
                        // end of key
                        cur_key = s.to_owned();
                        cur_comp = AttrComp::EQ;
                    },
                    QT => {
                        panic!("Badly formed SVG, unexpected open quote");
                    },
                    c => {
                        // part of the key String
                        let mut s = s.clone();
                        s.push(c);
                        s = s.trim().to_owned();

                        cur_comp = AttrComp::Key(s);
                    }
                }
            } 
            AttrComp::EQ => {
                // expect any amount of whitespace then a open quote for val
                if c.is_whitespace() {continue}
                match c {
                    QT=> {
                        cur_comp = AttrComp::OpenQt
                    },
                    _c => {
                        panic!("Unexpected character when parsing SVG attrs, expected open quote")
                    }                    
                }
            },
            AttrComp::OpenQt => {
                match c {
                    QT => {
                        cur_comp = AttrComp::CloesQt
                    },
                    c => {
                        cur_comp = AttrComp::Value(c.to_string())
                    }
                }
            },
            AttrComp::Value(mut s) => {
                match c {
                    QT => {
                        if !is_banned_attribute(&cur_key) {
                            attrs.push(SvgAttribute { key: cur_key.clone(), value: s , quote_value: true})
                        }
                        
                        cur_comp = AttrComp::CloesQt
                    },
                    c => {
                        s.push(c);
                        cur_comp = AttrComp::Value(s)
                    }
                } 

            },
            AttrComp::CloesQt => {
                cur_comp = AttrComp::Key(String::new());
            },
        }
    }


    // remove the attributes from contents
    *contents = (&contents[attr_end_i..]).to_owned();

    attrs
}


// /// helper function, pass a string of an attribute to parse
// /// `key="value"`
// /// returns none if attribute is not it SvgAttributes (we are removing those)
// fn parse_attr(attr: String) -> Option<SvgAttribute> {
//     let eq_i = attr.find("=")
//         .expect("to have found '=' when parsing attribute for svg macro"); 
//     let key = &attr[..eq_i];
//     let value = &attr[eq_i+1..];

//     match key {
//         ATTR_VEIWBOX => Some(SvgAttribute::ViewBox(value.to_owned())),
//         ATTR_VERSION => Some(SvgAttribute::Version(value.to_owned())),
//         _ => None
//     }
// }

/// helper function to remove all XML tags from svg 
fn remove_xml_tags(contents: &mut String) {
    while let Some(open_i) = contents.find(XML_OPEN_DELIM) {
        match &contents[open_i..].find(XML_CLOSE_DELIM) {
            Some(close_i) => {
                contents.replace_range(open_i..*close_i+XML_CLOSE_DELIM.len(), "")
            }
            None => panic!("error parsing svg file for svg macro: \
            xml opening delimeter found but no corresponding closing delimiter ")
        }
    }
} 

/// helper function that tokenizes elements from svg file string
fn parse_elements(contents:&mut String, parent_tag: &str) -> Vec<SvgElement> {
    let mut closed = false;
    let mut tag_name = String::new();
    let mut attrs = Vec::<SvgAttribute>::new();
    let mut children = Vec::<SvgElement>::new();


    let mut elements = Vec::<SvgElement>::new();

    // iterate throught the string file contents for siblings,
    // each iteration potentially recursing to collect children
    while let Some((open_pat, _open_i)) 
        = first_of(contents.as_str(), vec![
            CLOSE_TAG_OPEN_DELIM,
            OPEN_TAG_OPEN_DELIM, 
            TAG_CLOSE_DELIM,
            BODYLESS_TAG_CLOSE_DELIM
        ]) 
    {
        match open_pat.as_str() {
            CLOSE_TAG_OPEN_DELIM => {
                // get name of tag we are closing, don't remove it in case
                // it belongs to the parent tag 
                let closing_tag_name = peek_tag_name(contents);

                // trim the delimeter off the closing_tag_name
                // check if we are closing this recursions tag or parents tag
                if tag_name == closing_tag_name && !closed {
                    // this recursions tag, flag the closing and continue parsing
                    closed = true;
                    // remove the delim - already checked existence in match arm,
                    // therefore unrwap is safe
                    let start = contents.find(CLOSE_TAG_OPEN_DELIM).unwrap();
                    contents.replace_range(..start + CLOSE_TAG_OPEN_DELIM.len(), "");
                    // remove the tag name now too since it is owned by this tag
                    _ = pop_tag_name(contents);
                } else if parent_tag == closing_tag_name {
                    // end recursion since there are no more children
                    return elements
                } else {
                    panic!("Unexepected Closing tag found");
                }
            },
            OPEN_TAG_OPEN_DELIM => {
                // remove the delim - already checked existence unrwap is save
                let start = contents.find(OPEN_TAG_OPEN_DELIM).unwrap();
                contents.replace_range(..start + OPEN_TAG_OPEN_DELIM.len(), "");
                // get name
                tag_name = pop_tag_name(contents);
                // get attributes
                attrs = pop_attributes(contents);
                // get all children once open tag closes
            },
            TAG_CLOSE_DELIM => {
                // remove the delim - already checked existence unrwap is save
                let start = contents.find(TAG_CLOSE_DELIM).unwrap();
                contents.replace_range(..start + TAG_CLOSE_DELIM.len(), "");

                if !closed {
                    // open tag was closed, get any children
                    let mut elems = parse_elements(contents, &tag_name);
                    if !elems.is_empty() && !closed {
                        children.append(&mut elems); 
                    }
                } else {
                    // end tag is closed, add it to elements
                    elements.push(SvgElement { 
                        tag: tag_name.clone(), 
                        attributes: attrs.clone(), 
                        children: children.clone(),
                        bodyless: false
                    });
                    // clear tag_name, children, and unflag close since this  
                    // is the end of that tag so future closing checks can accurately
                    // check which of parent or current tag is closing 
                    tag_name = "".to_string();
                    closed = false;
                    children = vec![];
                }
            },
            BODYLESS_TAG_CLOSE_DELIM => {
                // no possible children, add element without check for children.
                // remove the delim - already checked existence unrwap is save
                let start = contents.find(BODYLESS_TAG_CLOSE_DELIM).unwrap();
                contents.replace_range(..start + BODYLESS_TAG_CLOSE_DELIM.len(), "");
                elements.push(SvgElement{
                    tag: tag_name.clone(),
                    attributes: attrs.clone(),
                    children: children.clone(),
                    bodyless: true
                });
                // no need to clean up closing tag logic for bodyless tags
                // because they will never have a separate closing tag
            },
            // this should never happen because we've defined the potential values
            // for this match
            _ => panic!("false pattern found while parsing svg")
        }
    }
    // reached end of SVG, return parsed elements (only after iteration, recurions
    // should never reach this) 
    return elements;
}