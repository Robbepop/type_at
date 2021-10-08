use proc_macro2::{
    Ident,
    TokenStream as TokenStream2,
};
use quote::{
    format_ident,
    quote,
};

/// Implements the `TypeAt` trait for tuples of up to 26 members.
pub fn impl_type_at_for_tuples() -> TokenStream2 {
    let index_names = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .map(char::from)
        .map(|name| format_ident!("{}", name));
    let len_index_names = index_names.len();
    let impls = (0..=len_index_names)
        .map(|n| &index_names[0..n])
        .map(impl_type_at_for_subtuple)
        .flatten();

    fn impl_type_at_for_subtuple(
        sub_tuple: &[Ident],
    ) -> impl Iterator<Item = TokenStream2> + '_ {
        sub_tuple.iter().enumerate().map(|(at, type_at)| {
            quote! {
                impl< #( #sub_tuple ),* > TypeAt<#at> for ( #( #sub_tuple , )* ) {
                    type Type = #type_at;
                }
            }
        })
    }

    quote! {
        #( #impls )*
    }
}
