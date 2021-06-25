use crate::stringed_box::StringedBlock;
use quote::ToTokens;
use quote::TokenStreamExt;
use syn::parse::{Parse, ParseStream, Result};
use syn::parse_quote;
use syn::Type;
use syn::{token, Block, Ident, Item, ItemFn, Stmt};

#[derive(Clone)]
pub enum Case {
    It(It),
    When(When),
    Setup(Setup),
    Item(Item),
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
            Case::Setup(fun) => fun.to_tokens(tokens),
            Case::Item(item) => item.to_tokens(tokens),
        }
    }
}

#[derive(Clone)]
pub struct When {
    pub block: StringedBlock<Case>,
}

impl Parse for When {
    fn parse(input: ParseStream) -> Result<Self> {
        let block = StringedBlock::parse(input)?;
        Ok(When { block })
    }
}

impl ToTokens for When {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let magic = self.block.clone();

        let has_setup = contains_setup(&magic.items);

        token::Mod::default().to_tokens(tokens);
        magic.ident.to_tokens(tokens);

        &magic.brace_token.surround(tokens, |tokens| {
            if !has_setup {
                let use_stmt: Stmt = parse_quote!(
                    use super::before;
                );
                use_stmt.to_tokens(tokens);
            }
            tokens.append_all(&magic.items);
        });
    }
}

#[derive(Clone)]
pub struct It {
    pub block: StringedBlock<Stmt>,
}

impl Parse for It {
    fn parse(input: ParseStream) -> Result<Self> {
        let block = StringedBlock::parse(input)?;
        Ok(It { block })
    }
}

impl ToTokens for It {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let magic = &self.block;

        let ident = magic.ident.clone();
        let arg = magic.arg.clone();
        let block = Block {
            brace_token: magic.brace_token,
            stmts: magic.items.clone(),
        };

        let test_body: Block = if let Some(_) = arg {
            syn::parse_quote!({
              let runner = |#arg| #block;
              runner(before())
            })
        } else {
            syn::parse_quote!(#block)
        };

        let test: ItemFn = syn::parse_quote! {
            #[test]
            fn #ident() {
                #test_body
            }
        };
        test.to_tokens(tokens);
    }
}

#[derive(Clone)]

pub struct Setup {
    pub block: Block,
    pub output: Box<Type>,
}

impl Setup {
    pub fn default() -> Self {
        let block = parse_quote!({});
        let output = parse_quote!(());
        Setup { block, output }
    }
}

impl Parse for Setup {
    fn parse(input: ParseStream) -> Result<Self> {
        let _: Ident = input.parse()?;
        let output: Type = input.parse()?;
        let block: Block = input.parse()?;
        Ok(Setup {
            output: Box::new(output),
            block,
        })
    }
}

impl ToTokens for Setup {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let output = &self.output.clone();
        let block = &self.block.clone();

        let my_setup: ItemFn = syn::parse_quote! {
            fn before() -> #output {
                #block
            }
        };
        my_setup.to_tokens(tokens);
    }
}

pub fn contains_setup(items: &Vec<Case>) -> bool {
    items.clone().into_iter().any(|case| match case {
        Case::Setup(_) => true,
        _ => false,
    })
}
