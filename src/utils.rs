use proc_macro2::Span;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream, Result};
use syn::token::Brace;
use syn::{braced, FnArg, Ident, LitStr, Token};

#[derive(Clone)]
pub struct DynamicBlock<T: Parse> {
    pub brace_token: Brace,
    pub items: Vec<T>,
}

impl<T: Parse> Parse for DynamicBlock<T> {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Brace) {
            let content;
            let brace_token = braced!(content in input);
            let mut items = Vec::new();
            while !content.is_empty() {
                items.push(content.parse()?);
            }
            Ok(DynamicBlock { brace_token, items })
        } else {
            Err(lookahead.error())
        }
    }
}

#[derive(Clone)]
pub struct StringedIdent {
    pub kind: Ident,
    pub ident: Ident,
}

impl Parse for StringedIdent {
    fn parse(input: ParseStream) -> Result<Self> {
        let kind: Ident = input.parse()?;
        let description: LitStr = input.parse()?;
        let ident_string = kind.to_string() + "_" + &description.value().replace(" ", "_");
        let ident = Ident::new(&ident_string, Span::call_site());
        Ok(StringedIdent { kind, ident })
    }
}

impl ToTokens for StringedIdent {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        &self.ident.to_tokens(tokens);
    }
}

#[derive(Clone)]
pub struct OptionalArg(Option<FnArg>);

impl OptionalArg {
    pub fn is_set(&self) -> bool {
        match &self.0 {
            Some(_) => true,
            _ => false,
        }
    }
}

impl Parse for OptionalArg {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookup = input.lookahead1();

        if lookup.peek(Token![|]) {
            let _: Token![|] = input.parse()?;
            let arg: FnArg = input.parse()?;
            let _: Token![|] = input.parse()?;
            Ok(OptionalArg(Some(arg)))
        } else {
            Ok(OptionalArg(None))
        }
    }
}

impl ToTokens for OptionalArg {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self.0 {
            Some(arg) => arg.to_tokens(tokens),
            _ => (),
        }
    }
}
