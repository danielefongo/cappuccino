extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use quote::ToTokens;
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, AttributeArgs, Item, Meta, NestedMeta, Path};

#[proc_macro_attribute]
pub fn setup(attr: TokenStream, input: TokenStream) -> TokenStream {
    let default_method: Path = syn::parse_str("before").unwrap();

    let args: AttributeArgs = parse_macro_input!(attr as AttributeArgs);

    let meta = args.iter().find_map(|meta| match meta {
        NestedMeta::Meta(Meta::Path(val)) => Some(val),
        _ => panic!("invalid meta"),
    });
    let setup_method = match meta {
        None => &default_method,
        Some(val) => val,
    };

    let mut item: Item = parse_macro_input!(input as Item);
    let fn_item = match &mut item {
        Item::Fn(fn_item) => fn_item,
        _ => panic!("expected fn"),
    };

    let inputs = fn_item.sig.inputs.clone();
    let block = fn_item.block.clone();

    fn_item.block = syn::parse(
        quote!({
          let runner = |#inputs| #block;
          runner(#setup_method());
        })
        .into(),
    )
    .unwrap();
    fn_item.sig.inputs = Punctuated::new();

    item.into_token_stream().into()
}
