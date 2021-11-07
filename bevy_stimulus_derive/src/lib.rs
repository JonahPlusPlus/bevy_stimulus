extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn stimulus(_: TokenStream, item: TokenStream) -> TokenStream {
    let body = syn::parse_macro_input!(item as syn::ItemEnum);
    let ident = body.ident.clone();
    let variants = body.variants.clone();
    let mut display_tokens = quote! {};
    let mut try_from_tokens = quote! {};
    for variant in variants {
        let variant_ident = variant.ident.clone();
        display_tokens.extend(quote! {
            #ident::#variant_ident => {
                write!(f, stringify!(#variant_ident))
            }
        });
        try_from_tokens.extend(quote! {
            stringify!(#variant_ident) => Ok(#ident::#variant_ident),
        });
    }

    let output = quote! {
        #[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
        #[serde(try_from = "String")]
        #body

        impl std::fmt::Display for #ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #display_tokens
                }
            }
        }

        impl TryFrom<String> for #ident {
            type Error = &'static str;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                match value.as_str() {
                    #try_from_tokens
                    _ => Err("String wasn't a variant")
                }
            }
        }
    };
    output.into()
}