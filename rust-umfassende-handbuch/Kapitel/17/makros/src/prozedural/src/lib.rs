use proc_macro::TokenStream;

use proc_macro2::Span;
use quote::{quote, quote_spanned};

#[proc_macro]
pub fn einfuegen(_: TokenStream) -> TokenStream {
    quote!(
        let mut ausgabe = String::new();

        println!("Hallo, aus dem Makro");
    )
    .into()
}

#[proc_macro]
pub fn sauber_einfuegen(_: TokenStream) -> TokenStream {
    // Die Hygiene auf mixed setzen, das entspricht dem
    // Verhalten von deklarativen Makros.
    let span :Span = Span::mixed_site();

    quote_spanned!( span =>
        let mut ausgabe = String::new();

        println!("Hallo, aus dem Makro");
    ).into()
}

#[proc_macro_attribute]
pub fn nur_auf(attribut: TokenStream, element: TokenStream) -> TokenStream {
    let item = syn::parse_macro_input!(element as syn::Item);

    // Lit = Literal
    let attribut = syn::parse_macro_input!(attribut as syn::Lit);

    TokenStream::from(quote! {
        #[cfg(target_os = #attribut)]
        #[derive(Debug)]
        #item
    })
}

#[proc_macro_derive(PersonIdentifikation, attributes(helper))]
pub fn person_identifikation(stream: TokenStream) -> TokenStream {
    let person = syn::parse_macro_input!(stream as syn::DeriveInput);
    let data = person.data;

    let fn_name = format!("id_{}", person.ident.to_string().to_lowercase());
    let fn_ident = proc_macro2::Ident::new(&fn_name, Span::mixed_site());

    TokenStream::from(quote! {
        fn #fn_ident() -> String {
            // ...
            format!("ID: {}", rand::random::<usize>())
        }
    })
}
