use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(ToBytes)]
pub fn derive_tobytes(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = input.ident;

    // Generate an expression to convert each field to bytes
    let bytes = to_bytes(&input.data);

    let expanded = quote! {
        // The generated impl.
        impl struct_bytes::ToBytes for #struct_name {
            fn to_bytes(&self) -> Vec<u8> {
                #bytes
            }
        }
    };

    // Hand the output tokens back to the compiler.
    TokenStream::from(expanded)
}

// Generate an expression to convert each field to bytes
fn to_bytes(data: &Data) -> proc_macro2::TokenStream {
    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    let recurse = fields.named.iter().map(|f| {
                        let name = &f.ident;
                        quote! {
                            &self.#name.to_bytes()
                        }
                    });
                    quote! {
                        let mut bytes = Vec::<u8>::new();
                        #(bytes.extend(#recurse);)*
                        bytes
                    }
                }
                Fields::Unnamed(_) | Fields::Unit => unimplemented!()
            }
        }
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}
