use crate::stringed_box::StringedBlock;
use quote::ToTokens;
use quote::TokenStreamExt;
use syn::parse::{Parse, ParseStream, Result};
use syn::{token, Block, Ident, ItemFn, Stmt};

pub enum Case {
    It(It),
    When(When),
}

impl Parse for Case {
    fn parse(input: ParseStream) -> Result<Self> {
        let forked_input = input.fork();
        let lookahead = input.lookahead1();
        let kind: Ident = forked_input.parse()?;

        match kind.to_string().as_str() {
            "it" => Ok(Case::It(It::parse(input).unwrap())),
            "when" => Ok(Case::When(When::parse(input).unwrap())),
            _ => Err(lookahead.error()),
        }
    }
}

impl ToTokens for Case {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self {
            Case::It(it) => it.to_tokens(tokens),
            Case::When(when) => when.to_tokens(tokens),
        }
    }
}

pub struct When {
    pub block: StringedBlock<Case>,
}

impl Parse for When {
    fn parse(input: ParseStream) -> Result<Self> {
        let block = StringedBlock::parse(input).unwrap();
        Ok(When { block })
    }
}

impl ToTokens for When {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let magic = &self.block;

        token::Mod::default().to_tokens(tokens);
        magic.ident.to_tokens(tokens);

        &magic.brace_token.surround(tokens, |tokens| {
            // let use_stmt: Stmt = syn::parse_quote! { use setup::describe; };
            // use_stmt.to_tokens(tokens);
            tokens.append_all(&magic.items);
        });
    }
}

pub struct It {
    pub block: StringedBlock<Stmt>,
}

impl Parse for It {
    fn parse(input: ParseStream) -> Result<Self> {
        let block = StringedBlock::parse(input).unwrap();
        Ok(It { block })
    }
}

impl ToTokens for It {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let magic = &self.block;

        let ident = magic.ident.clone();
        let block = Block {
            brace_token: magic.brace_token,
            stmts: magic.items.clone(),
        };

        let my_test: ItemFn = syn::parse_quote! {
            #[test]
            fn #ident() {
                #block
            }
        };
        my_test.to_tokens(tokens);
    }
}
