extern crate proc_macro;
extern crate quote;
use proc_macro::TokenStream;
use serde::{Serialize, Deserialize};
use quote::quote;
use proc_macro2::TokenTree;

use syn::{parse_macro_input, Item, ItemFn, Ident, Visibility, Token};
use syn::parse::{Parse, ParseStream, Result};

use rustweb_code::{JsEnc, JsAst, GlslEnc, GlslAst};

struct JavascriptModule {
    name: syn::Ident,
    file: Vec<syn::Stmt>
}

impl Parse for JavascriptModule {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: syn::Ident = input.parse()?;
        let file = syn::Block::parse_within(input)?;

        Ok(JavascriptModule { name, file })
    }
}

struct GlslModule {
    name: String
}

struct GlslParser<'a> {
    s: ParseStream<'a>
}

type Tt = TokenTree;
use TokenTree::{Group, Punct, Literal};

impl<'a> GlslParser<'a> {
    fn new(s: ParseStream) -> GlslParser {
        GlslParser { s }
    }

/*
    fn next_tt(&mut self) {
        let _: Tt = self.s.parse().unwrap();
    }

    fn test_tt(&mut self) {

    }

    fn rattribute(&mut self) -> Result<()> {
        let _: Token![#] = self.s.parse()?;
        Ok(())
    }

    fn rfuncorvar(&mut self) {

    }

    fn ritem(&mut self) -> Result<()> {
        if self.s.peek(Token![#]) {
            // Attribute
            self.rattribute()?;
        } else if self.s.peek(Ident) {
            self.rfuncorvar();
        }
        Ok(())
    }
*/

    fn parse(&mut self) -> Result<()> {
        while !self.s.is_empty() {
            //self.parse_item();
            let t: Tt = self.s.parse()?;
            println!("tt: {:?}", &t);
        }

        Ok(())
    }
}

impl Parse for GlslModule {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut parser = GlslParser::new(input);
        parser.parse().unwrap();

        Ok(GlslModule {
            name: "testglsl".to_owned()
        })
    }
}

#[proc_macro]
pub fn glsl(contents: TokenStream) -> TokenStream {

    let module = parse_macro_input!(contents as GlslModule);
    
    (quote!{}).into()
}

struct GlslModule2 {
    name: syn::Ident,
    file: syn::File
}

impl Parse for GlslModule2 {
    fn parse(input: ParseStream) -> Result<Self> {
        //let file = syn::Block::parse_within(input)?;
        let name: syn::Ident = input.parse()?;
        let file: syn::File = input.parse()?;
        
        Ok(GlslModule2 {
            name,
            file
        })
    }
}

#[proc_macro]
pub fn glsl2(contents: TokenStream) -> TokenStream {

/*    
    let module = parse_macro_input!(contents as GlslModule2);
    let input = module.file;

    let module_name = module.name.to_string();
    let module_name_upper = module_name.to_uppercase();
    let mut enc = GlslEnc::new(module_name);
    enc.parse_glsl(&input);

    let bytes = enc.arr;
    let bytes_len = bytes.len();

    let data_var_name = syn::Ident::new(&format!("GLSLDATA_{}", &module_name_upper), module.name.span());
    
    let q = quote! {
        #[link_section = "glsl"] static #data_var_name: [u8; #bytes_len] = [#(#bytes),*];
    };

    q.into()*/

    panic!("disabled");
}

#[proc_macro]
pub fn javascript(contents: TokenStream) -> TokenStream {

    let module = parse_macro_input!(contents as JavascriptModule);
    let input = module.file;
    let module_name = module.name.to_string();  
    let module_name_upper = module_name.to_uppercase();
    let mut enc = JsEnc::new(module_name);
    enc.parse_rs(&input);
    
    let bytes = enc.arr;
    let bytes_len = bytes.len();

    let data_var_name = syn::Ident::new(&format!("JSDATA_{}", &module_name_upper), module.name.span());

    if false {
        let mut exported_funcs = Vec::new();

        for (name, exported_name, item) in &enc.exported_decls {

            let sp = item.sig.ident.span();

            let mut js_name_path_segs = syn::punctuated::Punctuated::new();
            js_name_path_segs.push_value(syn::PathSegment {
                ident: syn::Ident::new(&"wasm_bindgen", item.sig.ident.span()),
                arguments: syn::PathArguments::None
            });

            let fif = syn::ForeignItemFn {
                // #[wasm_bindgen(js_name = #exported_name)]
                attrs: vec![
                    syn::Attribute {
                        pound_token: syn::token::Pound { spans: [sp] },
                        style: syn::AttrStyle::Outer,
                        bracket_token: syn::token::Bracket { span: sp },
                        path: syn::Path {
                            leading_colon: None,
                            segments: js_name_path_segs
                        },
                        tokens: quote!((js_name = #exported_name))
                    }
                ],
                vis: item.vis.clone(),
                //ident: syn::Ident::new(&name, item.ident.span()),
                sig: item.sig.clone(),
                semi_token: syn::token::Semi { spans: [sp] }
            };

            exported_funcs.push(fif);
        }
        
        let q = quote! {
            #[link_section = "js"] static #data_var_name: [u8; #bytes_len] = [#(#bytes),*];
            
            #[wasm_bindgen(raw_module = "i")]
            extern "C" {
                #(#exported_funcs)*
            }
        };
        q.into()
    } else {
        let mut exported_funcs = Vec::new();

        for (name, exported_name, item) in &enc.exported_decls {

            let sp = item.sig.ident.span();

/*
            let mut js_name_path_segs = syn::punctuated::Punctuated::new();
            js_name_path_segs.push_value(syn::PathSegment {
                ident: syn::Ident::new(&"wasm_bindgen", item.ident.span()),
                arguments: syn::PathArguments::None
            });
*/

            let fif = syn::ForeignItemFn {
                attrs: vec![],
                vis: item.vis.clone(),
                //ident: syn::Ident::new(&name, item.ident.span()),
                sig: item.sig.clone(),
                semi_token: syn::token::Semi { spans: [sp] }
            };

            exported_funcs.push(fif);
        }

        let q = quote! {
            #[link_section = "js"] static #data_var_name: [u8; #bytes_len] = [#(#bytes),*];
            
            #[link(wasm_import_module = "i")]
            extern "C" {
                #(#exported_funcs)*
            }
        };
        q.into()
    }
    
}