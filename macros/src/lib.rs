use proc_macro::TokenStream;
use quote::quote;
use syn::{Expr, ExprLit, Lit};

#[proc_macro]
pub fn parse_color(input:TokenStream) -> TokenStream {
    let Expr::Lit(ExprLit { lit: Lit::Str(string), .. }) = syn::parse_macro_input!(input as Expr) else {
        return quote!(compile_error!("Macro input should be a string literal")).into();
    };
    match color::parse_color(&string.value()) {
        Ok(color) => {
            let [r, g, b, a] = color.to_alpha_color::<color::Srgb>().components;
            quote!(vello::peniko::Color::new([#r, #g, #b, #a])).into()
        },
        Err(err) => {
            let err = format!("{err:?}");
            quote!(compile_error!(#err)).into()
        }
    }
}
