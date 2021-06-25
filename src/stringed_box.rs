use proc_macro2::Span;
use syn::parse::{Parse, ParseStream, Result};
use syn::token::{Brace};
use syn::{braced, FnArg, Ident, LitStr, Token};

pub struct StringedBlock<T: Parse> {
    pub kind: Ident,
    pub ident: Ident,
    pub brace_token: Brace,
    pub items: Vec<T>,
    pub arg: Option<FnArg>,
}

impl<T: Parse> Parse for StringedBlock<T> {
    fn parse(input: ParseStream) -> Result<Self> {
        let kind: Ident = input.parse()?;
        let description: LitStr = input.parse()?;
        let ident_string = kind.to_string() + "_" + &description.value().replace(" ", "_");
        let ident = Ident::new(&ident_string, Span::call_site());

        let lookahead = input.lookahead1();
        if lookahead.peek(Brace) {
            let (brace_token, items) = block_data(input)?;

            Ok(StringedBlock {
                kind,
                ident,
                brace_token,
                items,
                arg: None,
            })
        } else if lookahead.peek(Token![|]) {
            let arg = arg_data(input)?;
            let (brace_token, items) = block_data(input)?;

            Ok(StringedBlock {
                kind,
                ident,
                brace_token,
                items,
                arg: Some(arg),
            })
        } else {
            Err(lookahead.error())
        }
    }
}

fn arg_data(input: &syn::parse::ParseBuffer) -> Result<FnArg> {
    let _: Token![|] = input.parse()?;
    let arg: FnArg = input.parse()?;
    let _: Token![|] = input.parse()?;
    Ok(arg)
}

fn block_data<T: Parse>(input: ParseStream) -> Result<(Brace, Vec<T>)> {
    let content;
    let brace_token = braced!(content in input);
    let mut items = Vec::new();
    while !content.is_empty() {
        items.push(content.parse()?);
    }

    Ok((brace_token, items))
}
