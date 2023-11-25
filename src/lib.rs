use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro_derive(FieldNames)]
pub fn field_names(stream: TokenStream) -> TokenStream {
    let input = parse_macro_input!(stream as syn::DeriveInput);
    let name = input.ident;
    if let syn::Data::Struct(s) = input.data {
        let fields = s.fields;
        let mut funcs = Vec::new();
        for field in fields {
            if let Some(ident) = field.ident {
                funcs.push(quote::quote! {
                    pub fn #ident() -> String {
                        stringify!(#ident).into()
                    }
                });
            }
        }
        return quote::quote! {
            impl #name {
                #(#funcs)*
            }
        }
        .into();
    }
    panic!("Only structs are supported");
}
