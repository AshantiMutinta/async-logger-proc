extern crate proc_macro;
use proc_macro::TokenStream;

use syn::ItemFn;
use quote::quote;
use quote::{ToTokens};


struct AsyncLogger;



/// Example of user-defined [derive mode macro][1]
///
/// [1]: https://doc.rust-lang.org/reference/procedural-macros.html#derive-mode-macros
#[proc_macro_attribute]
pub fn async_logger(_attr: TokenStream,item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);
    let mut fold : Box<dyn syn::fold::Fold>  = Box::new(AsyncLogger);
    let tokens = fold.fold_item_fn(input);
    tokens.to_token_stream().into()
}

impl syn::fold::Fold for AsyncLogger{

    fn fold_item_fn(&mut self, function: ItemFn)-> syn::ItemFn
    {
        let mut function = function.clone();
        let function_name = function.sig.ident.to_string();
        let block = function.block;
        let new_block = quote!{ {
            span!(#function_name,#block) }
        };
        function.block = syn::parse(TokenStream::from(new_block.clone())).expect("Could not rewrite block with procoedural logger macro");
        function

    }
}

