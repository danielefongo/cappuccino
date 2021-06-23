extern crate proc_macro;

mod case;
mod stringed_box;
mod test;

use proc_macro::TokenStream;
use quote::ToTokens;
use test::Test;

#[proc_macro]
pub fn tests(item: TokenStream) -> TokenStream {
    let item: Test = syn::parse_macro_input!(item as Test);
    item.to_token_stream().into()
}
