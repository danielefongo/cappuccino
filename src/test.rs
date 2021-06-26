use crate::case::{Case, When};
use crate::utils::{DynamicBlock, StringedIdent};
use quote::ToTokens;
use syn::parse::{Parse, ParseStream, Result};
pub struct Test(When);

impl Parse for Test {
    fn parse(input: ParseStream) -> Result<Self> {
        let tests_ident: StringedIdent = StringedIdent::from("tests");
        let block: DynamicBlock<Case> = input.parse()?;
        Ok(Test(When::new(tests_ident, block)))
    }
}

impl ToTokens for Test {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens);
    }
}
