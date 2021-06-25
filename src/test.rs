use quote::ToTokens;
use quote::TokenStreamExt;
use syn::parse::{Parse, ParseStream, Result};
use syn::token::Mod;
use syn::Ident;

use crate::case::contains_setup;
use crate::case::Case;
use crate::case::Setup;
use crate::utils::DynamicBlock;

pub struct Test(DynamicBlock<Case>);

impl Parse for Test {
    fn parse(input: ParseStream) -> Result<Self> {
        let block = input.parse()?;
        Ok(Test(block))
    }
}

impl ToTokens for Test {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let block = &self.0;
        Mod::default().to_tokens(tokens);

        let tests_ident: Ident = syn::parse_quote!(tests);
        tests_ident.to_tokens(tokens);

        &block.brace_token.surround(tokens, |tokens| {
            if !contains_setup(&block.items) {
                Setup::default().to_tokens(tokens);
            }
            tokens.append_all(&block.items);
        });
    }
}
