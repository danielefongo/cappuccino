use crate::utils::{to_tokens, CasesBlock, OptionalReturnType, StatementsBlock, StringedIdent};
use quote::ToTokens;
use syn::parse::{Parse, ParseStream, Result};
use syn::token::{Async, Mod};
use syn::{Attribute, Block, Item, Signature, Token};

mod case_ident {
    syn::custom_keyword!(it);
    syn::custom_keyword!(when);
    syn::custom_keyword!(before);
}

pub trait Setuppable {
    fn add_before(&mut self, before: &Option<Before>);
    fn get_before(&self) -> Option<Before>;
}

#[derive(Clone)]
pub enum Case {
    It(It),
    When(When),
    Before(Before),
    Item(Item),
}

impl Setuppable for Vec<Case> {
    fn add_before(&mut self, before: &Option<Before>) {
        self.iter_mut().for_each(|case| case.add_before(before));
    }
    fn get_before(&self) -> Option<Before> {
        self.into_iter().find_map(|case| match case {
            Case::Before(a) => Some(a.clone()),
            _ => None,
        })
    }
}

impl Setuppable for Case {
    fn add_before(&mut self, before: &Option<Before>) {
        match self {
            Case::It(it) => it.add_before(before),
            Case::When(when) => when.add_before(before),
            _ => (),
        }
    }
    fn get_before(&self) -> Option<Before> {
        match self {
            Case::It(it) => it.get_before(),
            Case::When(when) => when.get_before(),
            _ => None,
        }
    }
}

impl Parse for Case {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(case_ident::when) {
            Ok(Case::When(When::parse(input)?))
        } else if input.peek(case_ident::it) {
            Ok(Case::It(It::parse(input)?))
        } else if input.peek(case_ident::before) {
            Ok(Case::Before(Before::parse(input)?))
        } else {
            Ok(Case::Item(input.parse()?))
        }
    }
}

impl ToTokens for Case {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self {
            Case::It(it) => it.to_tokens(tokens),
            Case::When(when) => when.to_tokens(tokens),
            Case::Item(item) => item.to_tokens(tokens),
            Case::Before(_) => (),
        }
    }
}

#[derive(Clone)]
pub struct When {
    pub ident: StringedIdent,
    pub block: CasesBlock,
    pub before: Option<Before>,
}

impl When {
    pub fn new(ident: StringedIdent, block: CasesBlock) -> Self {
        let before = block.items.get_before();
        When {
            ident,
            block,
            before,
        }
    }
}

impl Setuppable for When {
    fn add_before(&mut self, before: &Option<Before>) {
        if let None = self.before {
            self.before = before.clone();
        }
    }
    fn get_before(&self) -> Option<Before> {
        self.before.clone()
    }
}

impl Parse for When {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<case_ident::when>()?;
        let ident = input.parse()?;
        let block: CasesBlock = input.parse()?;
        Ok(When::new(ident, block))
    }
}

impl ToTokens for When {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let mut block = self.block.clone();
        block.items.insert(
            0,
            syn::parse_quote!(
                use super::*;
            ),
        );
        block.items.add_before(&self.before);

        Mod::default().to_tokens(tokens);
        self.ident.to_tokens(tokens);
        block.to_tokens(tokens);
    }
}

#[derive(Clone)]
pub struct It {
    pub ident: StringedIdent,
    pub block: StatementsBlock,
    pub before: Option<Before>,
    pub output: OptionalReturnType,
    #[cfg(feature = "async")]
    pub async_token: Option<Token![async]>,
}

impl Setuppable for It {
    fn add_before(&mut self, before: &Option<Before>) {
        self.before = before.clone();
    }
    fn get_before(&self) -> Option<Before> {
        self.before.clone()
    }
}

impl Parse for It {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<case_ident::it>()?;
        let ident = input.parse()?;
        #[cfg(feature = "async")]
        let async_token = {
            if input.peek(Async) {
                Some(input.parse::<Async>()?)
            } else {
                None
            }
        };
        let output = input.parse()?;
        let block = input.parse()?;
        Ok(It {
            #[cfg(feature = "async")]
            async_token,
            ident,
            block,
            output,
            before: None,
        })
    }
}

impl ToTokens for It {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let mut block = self.block.clone();
        #[cfg(feature = "async")]
        if let Some(_) = &self.async_token {
            block = syn::parse_quote!({futures::executor::block_on(async #block)});
        };
        if let Some(before) = &self.before {
            block.add_before(before.block.stmts.clone())
        };
        let ident = self.ident.clone();
        let output = self.output.clone();

        to_tokens::<Attribute>(tokens, syn::parse_quote!(#[test]));
        to_tokens::<Attribute>(tokens, syn::parse_quote!(#[allow(unused)]));
        to_tokens::<Signature>(tokens, syn::parse_quote!(fn #ident() #output));
        to_tokens(tokens, block);
    }
}

#[derive(Clone)]

pub struct Before {
    pub block: Block,
}

impl Parse for Before {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<case_ident::before>()?;
        let block: Block = input.parse()?;
        Ok(Before { block })
    }
}
