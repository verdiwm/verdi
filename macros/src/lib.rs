use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Dispatcher)]
pub fn derive_dispatcher(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;

    quote! {
        #[async_trait::async_trait]
        impl crate::Dispatcher for #ident {
            async fn dispatch(
                &self,
                object: &crate::Object,
                client: &mut crate::Client,
                message: &mut crate::Message,
            ) -> Result<()> {
                self.handle_request(object, client, message).await
            }
        }
    }
    .into()
}
