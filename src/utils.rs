use proc_macro2::Span;
use quote::{ToTokens, TokenStreamExt};
use syn::group::parse_parens;
use syn::parse::{Parse, ParseStream, Result};
use syn::token::{Brace, RArrow};
use syn::{braced, Block, Ident, LitStr, ReturnType, Stmt};

use crate::case::Case;

pub fn to_tokens<T: ToTokens>(tokens: &mut proc_macro2::TokenStream, data: T) {
    data.to_tokens(tokens);
}

#[derive(Clone)]
pub struct CasesBlock {
    pub brace_token: Brace,
    pub items: Vec<Case>,
}

impl Parse for CasesBlock {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Brace) {
            let content;
            let brace_token = braced!(content in input);
            let mut items = Vec::new();
            while !content.is_empty() {
                let item: Case = content.parse()?;
                items.push(item);
            }
            Ok(CasesBlock { brace_token, items })
        } else {
            Err(lookahead.error())
        }
    }
}

impl ToTokens for CasesBlock {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        &self
            .brace_token
            .surround(tokens, |tokens| tokens.append_all(&self.items));
    }
}

#[derive(Clone)]
pub struct StatementsBlock(Block);

impl StatementsBlock {
    pub fn add_before(&mut self, vect: Vec<Stmt>) {
        vect.into_iter().rev().for_each(|item| {
            self.0.stmts.insert(0, item);
        });
    }
}

impl Parse for StatementsBlock {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(StatementsBlock(input.parse()?))
    }
}

impl ToTokens for StatementsBlock {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens);
    }
}

#[derive(Clone)]
pub struct StringedIdent(Ident);

impl StringedIdent {
    pub fn from(ident_string: &str) -> Self {
        StringedIdent(Ident::new(
            &ident_string.replace(" ", "_"),
            Span::call_site(),
        ))
    }
}

impl Parse for StringedIdent {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(LitStr) {
            let description: LitStr = input.parse()?;
            Ok(StringedIdent::from(&description.value()))
        } else {
            let ident = input.parse()?;
            let _ = parse_parens(input)?;

            Ok(StringedIdent(ident))
        }
    }
}

impl ToTokens for StringedIdent {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        &self.0.to_tokens(tokens);
    }
}

#[derive(Clone)]
pub struct OptionalReturnType(Option<ReturnType>);

impl Parse for OptionalReturnType {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(RArrow) {
            Ok(OptionalReturnType(Some(input.parse()?)))
        } else {
            Ok(OptionalReturnType(None))
        }
    }
}

impl ToTokens for OptionalReturnType {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        if let Some(return_type) = &self.0 {
            return_type.to_tokens(tokens);
        }
    }
}
