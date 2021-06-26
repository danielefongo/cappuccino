use crate::case::{Case, When};
use crate::utils::{DynamicBlock, StringedIdent};
use quote::ToTokens;
use syn::parse::{Parse, ParseStream, Result};
use syn::LitStr;
pub struct Test(When);

impl Parse for Test {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();

        let ident = if lookahead.peek(LitStr) {
            let tests_ident: LitStr = input.parse()?;
            StringedIdent::from(&tests_ident.value())
        } else {
            StringedIdent::from("tests")
        };
        let block: DynamicBlock<Case> = input.parse()?;
        Ok(Test(When::new(ident, block)))
    }
}

impl ToTokens for Test {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens);
    }
}
