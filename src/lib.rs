use std::fs;
use std::path::PathBuf;
use proc_macro::TokenStream;
use quote::quote;
use syn::token::Comma;
use syn::punctuated::Punctuated;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Result, LitStr};
use syn::ItemFn;


struct Write {
    filename: PathBuf,
    content: Option<String>,
}

impl Parse for Write {
    fn parse(input: ParseStream) -> Result<Self> {
        type Inner = Punctuated<LitStr, Comma>;
        let args = Inner::parse_terminated(input)?.into_iter().collect::<Vec<_>>();
        let filename = args[0].value().into();
        let content = args.get(1).map(|s| s.value());
        Ok(Self {
            filename,
            content,
        })
    }
}

#[proc_macro]
pub fn write(tokens: TokenStream) -> TokenStream {
    let write = parse_macro_input!(tokens as Write);
    if let Some(parent) = write.filename.parent() {
        fs::create_dir_all(parent).expect("Failed to create parent directory.");
    }
    fs::write(write.filename, write.content.unwrap_or_default()).expect("Failed to write file.");
    TokenStream::new()
}

#[proc_macro_attribute]
pub fn skip(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    TokenStream::new()
}