use crate::utils::{DynamicBlock, StringedIdent};
use quote::{ToTokens, TokenStreamExt};
use syn::parse::{Parse, ParseStream, Result};
use syn::token::Mod;
use syn::{Block, Ident, Item, ItemFn, Stmt};

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
        let forked_input = input.fork();
        let lookahead = input.lookahead1();

        if lookahead.peek(Ident) {
            let kind: Ident = forked_input.parse()?;

            match kind.to_string().as_str() {
                "it" => Ok(Case::It(It::parse(input)?)),
                "when" => Ok(Case::When(When::parse(input)?)),
                "before" => Ok(Case::Before(Before::parse(input)?)),
                _ => Err(lookahead.error()),
            }
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
    pub block: DynamicBlock<Case>,
    pub ident: StringedIdent,
    pub before: Option<Before>,
}

impl When {
    pub fn new(ident: StringedIdent, block: DynamicBlock<Case>) -> Self {
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
        let ident = input.parse()?;
        let block: DynamicBlock<Case> = input.parse()?;

        Ok(When::new(ident, block))
    }
}

impl ToTokens for When {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let mut block = self.block.clone();
        block.items.add_before(&self.before);

        Mod::default().to_tokens(tokens);
        self.ident.to_tokens(tokens);
        &block
            .brace_token
            .surround(tokens, |tokens| tokens.append_all(&block.items));
    }
}

#[derive(Clone)]
pub struct It {
    pub ident: StringedIdent,
    pub block: DynamicBlock<Stmt>,
    pub before: Option<Before>,
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
        let ident = input.parse()?;
        let block = input.parse()?;
        Ok(It {
            ident,
            block,
            before: None,
        })
    }
}

impl ToTokens for It {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let mut block = self.block.clone();

        if let Some(before) = &self.before {
            block.append_on_start(before.block.stmts.clone());
        };

        let ident = self.ident.clone();
        let block = Block {
            brace_token: block.brace_token,
            stmts: block.items.clone(),
        };

        let test: ItemFn = syn::parse_quote! {
            #[test]
            fn #ident() {
                #[allow(unused)]
                #block;
            }
        };
        test.to_tokens(tokens);
    }
}

#[derive(Clone)]

pub struct Before {
    pub block: Block,
}

impl Parse for Before {
    fn parse(input: ParseStream) -> Result<Self> {
        let _: Ident = input.parse()?;
        let block: Block = input.parse()?;
        Ok(Before { block })
    }
}
