use proc_macro2::Span;
use syn::parse::{Parse, ParseStream, Result};
use syn::token::Brace;
use syn::{braced, Ident, LitStr};

pub struct StringedBlock<T: Parse> {
    pub kind: Ident,
    pub ident: Ident,
    pub brace_token: Brace,
    pub items: Vec<T>,
}

impl<T: Parse> Parse for StringedBlock<T> {
    fn parse(input: ParseStream) -> Result<Self> {
        let kind: Ident = input.parse()?;
        let description: LitStr = input.parse()?;
        let ident_string = kind.to_string() + "_" + &description.value().replace(" ", "_");
        let ident = Ident::new(&ident_string, Span::call_site());

        let lookahead = input.lookahead1();
        if lookahead.peek(Brace) {
            let content;
            let brace_token = braced!(content in input);

            let mut items = Vec::new();
            while !content.is_empty() {
                items.push(content.parse()?);
            }

            Ok(StringedBlock {
                kind,
                ident,
                brace_token,
                items,
            })
        } else {
            Err(lookahead.error())
        }
    }
}
