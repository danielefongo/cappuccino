use crate::case::When;
use crate::utils::{CasesBlock, StringedIdent};
use quote::ToTokens;
use syn::parse::{Parse, ParseStream, Result};
pub struct Test(When);

impl Parse for Test {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident = if let Ok(_) = input.fork().parse::<StringedIdent>() {
            input.parse::<StringedIdent>()?
        } else {
            StringedIdent::from("tests")
        };
        let block: CasesBlock = input.parse()?;
        Ok(Test(When::new(ident, block)))
    }
}

impl ToTokens for Test {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens);
    }
}
