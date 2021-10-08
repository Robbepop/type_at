use proc_macro2::{
    Literal,
    Span,
    TokenStream as TokenStream2,
};
use proc_macro_crate::{
    crate_name,
    FoundCrate,
};
use quote::{
    quote,
    quote_spanned,
};
use syn::{
    spanned::Spanned as _,
    Error,
    Ident,
};

/// Implements the derive of `#[derive(TypeAt)]` for struct types.
pub fn derive_type_at(input: TokenStream2) -> Result<TokenStream2, syn::Error> {
    let crate_ident = query_crate_ident()?;
    let input = syn::parse2::<syn::DeriveInput>(input)?;
    let struct_ident = &input.ident;
    let data_struct = match &input.data {
        syn::Data::Struct(data_struct) => data_struct,
        _ => {
            return Err(Error::new(
                Span::call_site(),
                "TypeAt derive only works on struct types.",
            ))
        }
    };
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let impls = data_struct.fields.iter().enumerate().map(|(n, field)| {
        let field_span = field.span();
        let field_type = &field.ty;
        let field_pos = Literal::usize_unsuffixed(n);
        quote_spanned!(field_span=>
            impl #impl_generics #crate_ident::TypeAt<#field_pos> for #struct_ident #ty_generics #where_clause {
                type Type = #field_type;
            }
        )
    });
    Ok(quote! {
        #( #impls )*
    })
}

/// Queries the dependencies for the `type_at` crate name and returns the identifier.
///
/// # Note
///
/// This allows to use crate aliases in `Cargo.toml` files of dependencies.
fn query_crate_ident() -> Result<TokenStream2, syn::Error> {
    let query = crate_name("type_at").map_err(|error| {
        Error::new(
            Span::call_site(),
            format!("could not find root crate for TypeAt derive: {}", error),
        )
    })?;
    match query {
        FoundCrate::Itself => Ok(quote! { crate }),
        FoundCrate::Name(name) => {
            let ident = Ident::new(&name, Span::call_site());
            Ok(quote! { ::#ident })
        }
    }
}
