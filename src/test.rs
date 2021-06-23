use quote::ToTokens;
use quote::TokenStreamExt;
use syn::parse::{Parse, ParseStream, Result};
use syn::{braced, token, Ident};

use crate::case::Case;

pub struct Test {
    pub brace_token: token::Brace,
    pub items: Vec<Case>,
}

impl Test {
    fn new(brace_token: token::Brace, items: Vec<Case>) -> Test {
        Test { brace_token, items }
    }
}

impl Parse for Test {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(token::Brace) {
            let content;
            let brace_token = braced!(content in input);

            let mut items = Vec::new();
            while !content.is_empty() {
                items.push(content.parse()?);
            }

            Ok(Test::new(brace_token, items))
        } else {
            Err(lookahead.error())
        }
    }
}

impl ToTokens for Test {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        token::Mod::default().to_tokens(tokens);

        let tests_ident: Ident = syn::parse_quote! { tests };
        tests_ident.to_tokens(tokens);

        &self.brace_token.surround(tokens, |tokens| {
            // let use_stmt: Stmt = syn::parse_quote! { use setup::describe; };
            // use_stmt.to_tokens(tokens);
            tokens.append_all(&self.items);
        });
    }
}
