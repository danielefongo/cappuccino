use crate::utils::{DynamicBlock, StringedIdent};
use quote::{ToTokens, TokenStreamExt};
use syn::parse::{Parse, ParseStream, Result};
use syn::token::Mod;
use syn::{Block, Ident, Item, ItemFn, Stmt};

pub trait Setuppable {
    fn add_setup(&mut self, setup: &Option<Setup>);
    fn get_setup(&self) -> Option<Setup>;
}

#[derive(Clone)]
pub enum Case {
    It(It),
    When(When),
    Setup(Setup),
    Item(Item),
}

impl Setuppable for Vec<Case> {
    fn add_setup(&mut self, setup: &Option<Setup>) {
        self.iter_mut().for_each(|case| case.add_setup(setup));
    }
    fn get_setup(&self) -> Option<Setup> {
        self.into_iter().find_map(|case| match case {
            Case::Setup(a) => Some(a.clone()),
            _ => None,
        })
    }
}

impl Setuppable for Case {
    fn add_setup(&mut self, setup: &Option<Setup>) {
        match self {
            Case::It(it) => it.add_setup(setup),
            Case::When(when) => when.add_setup(setup),
            _ => (),
        }
    }
    fn get_setup(&self) -> Option<Setup> {
        match self {
            Case::It(it) => it.get_setup(),
            Case::When(when) => when.get_setup(),
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
                "before" => Ok(Case::Setup(Setup::parse(input)?)),
                _ => Err(lookahead.error()),
            }
        } else {
            let item: Item = input.parse()?;
            Ok(Case::Item(item))
        }
    }
}

impl ToTokens for Case {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self {
            Case::It(it) => it.to_tokens(tokens),
            Case::When(when) => when.to_tokens(tokens),
            Case::Item(item) => item.to_tokens(tokens),
            Case::Setup(_) => (),
        }
    }
}

#[derive(Clone)]
pub struct When {
    pub block: DynamicBlock<Case>,
    pub ident: StringedIdent,
    pub setup: Option<Setup>,
}

impl Setuppable for When {
    fn add_setup(&mut self, setup: &Option<Setup>) {
        if let None = self.setup {
            self.setup = setup.clone();
        }
    }
    fn get_setup(&self) -> Option<Setup> {
        self.setup.clone()
    }
}

impl Parse for When {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident = input.parse()?;
        let block: DynamicBlock<Case> = input.parse()?;
        let setup = block.items.get_setup();

        Ok(When {
            ident,
            block,
            setup,
        })
    }
}

impl ToTokens for When {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let mut block = self.block.clone();
        block.items.add_setup(&self.setup);

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
    pub setup: Option<Setup>,
}

impl Setuppable for It {
    fn add_setup(&mut self, setup: &Option<Setup>) {
        self.setup = setup.clone();
    }
    fn get_setup(&self) -> Option<Setup> {
        self.setup.clone()
    }
}

impl Parse for It {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident = input.parse()?;
        let block = input.parse()?;
        Ok(It {
            ident,
            block,
            setup: None,
        })
    }
}

impl ToTokens for It {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let mut block = self.block.clone();

        if let Some(setup) = &self.setup {
            block.append_on_start(setup.block.stmts.clone());
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

pub struct Setup {
    pub block: Block,
}

impl Parse for Setup {
    fn parse(input: ParseStream) -> Result<Self> {
        let _: Ident = input.parse()?;
        let block: Block = input.parse()?;
        Ok(Setup { block })
    }
}
