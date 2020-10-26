use crate::args;
use crate::utils::{get_crate_name, get_rustdoc, GeneratorResult};
use proc_macro::TokenStream;
use quote::quote;

pub fn generate(desc_args: &args::Description) -> GeneratorResult<TokenStream> {
    let crate_name = get_crate_name(desc_args.internal);
    let ident = &desc_args.ident;
    let generics = &desc_args.generics;
    let doc = get_rustdoc(&desc_args.attrs)?.unwrap_or_default();
    let expanded = quote! {
        impl #crate_name::Description for #ident #generics {
            fn description() -> &'static str {
                #doc
            }
        }
    };
    Ok(expanded.into())
}
