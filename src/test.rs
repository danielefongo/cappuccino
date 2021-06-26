use quote::{ToTokens, TokenStreamExt};
use syn::parse::{Parse, ParseStream, Result};
use syn::token::Mod;
use syn::Ident;

use crate::case::{Case, Setup, Setuppable};
use crate::utils::DynamicBlock;
pub struct Test {
    block: DynamicBlock<Case>,
    setup: Option<Setup>,
}

impl Parse for Test {
    fn parse(input: ParseStream) -> Result<Self> {
        let block: DynamicBlock<Case> = input.parse()?;
        let setup = block.items.get_setup();
        Ok(Test { block, setup })
    }
}

impl ToTokens for Test {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let mut block = self.block.clone();
        block.items.add_setup(&self.setup);

        Mod::default().to_tokens(tokens);
        let tests_ident: Ident = syn::parse_quote!(tests);
        tests_ident.to_tokens(tokens);
        &block
            .brace_token
            .surround(tokens, |tokens| tokens.append_all(&block.items));
    }
}
