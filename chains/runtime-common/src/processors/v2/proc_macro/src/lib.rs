use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{DeriveInput, GenericParam, WhereClause, parse_macro_input, parse_quote};

#[proc_macro_derive(MessageProcessor)]
pub fn message_processor_trait_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a DeriveInput struct
    let ast = parse_macro_input!(input as DeriveInput);
    message_processor_trait_derive_impl(ast).into()
}

fn message_processor_trait_derive_impl(ast: DeriveInput) -> proc_macro2::TokenStream {
    // Get the name of the struct/enum
    let name = &ast.ident;
    let mut generics_for_impl = ast.generics.clone();
    generics_for_impl
        .params
        .push(GenericParam::Type(parse_quote!(AccountId)));
    let (impl_generics, _ty_generics, _where_clause) = generics_for_impl.split_for_impl();

    let generic_for_type = ast.generics.clone();
    let (_impl_generics, ty_generics, where_clause) = generic_for_type.split_for_impl();

    // Modify where clause to indicate trait impl restriction
    let modified_where_clause = if let Some(where_clause) = where_clause {
        let mut modified_where_clause = where_clause.clone();
        modified_where_clause
            .predicates
            .push(parse_quote!(#name #ty_generics: MessageProcessorWithFallback<AccountId>));
        modified_where_clause
    } else {
        WhereClause {
            where_token: syn::token::Where(Span::call_site()),
            predicates: parse_quote!(#name #ty_generics: MessageProcessorWithFallback<AccountId>),
        }
    };

    // Generate the implementation of MessageProcessor for the struct/enum
    let expanded = quote! {
        impl #impl_generics snowbridge_inbound_queue_primitives::v2::MessageProcessor<AccountId> for #name #ty_generics #modified_where_clause  {
            fn can_process_message(who: &AccountId, message: &Message) -> bool {
                let result = #name::try_extract_message(who, message);
                match result {
                    Ok(_) => true,
                    // We want to return true in case of invalid message as it means that
                    // while the message matches this processor's criteria, it is not correctly
                    // formed, which implies that it must be handled by the fallback processor for this
                    // message processor.
                    Err(MessageExtractionError::InvalidMessage { .. }) => true,
                    // We want to return true in case of other message as it typically means that something
                    // internal has failed in this message processor, and it does not mean that we should
                    // pass message to other processor down the line
                    Err(MessageExtractionError::Other { .. }) => true,
                    // Message is unsupported by this processor, let's forward it to next one
                    Err(MessageExtractionError::UnsupportedMessage { .. }) => false,
                }
            }

            fn process_message(who: AccountId, message: Message) -> Result<[u8; 32], snowbridge_inbound_queue_primitives::v2::MessageProcessorError> {
                // Do extraction
                let result = #name::try_extract_message(&who, &message);
                match result {
                    Ok(extracted_message) => #name::process_extracted_message(who, extracted_message),
                    Err(MessageExtractionError::InvalidMessage { .. }) => <#name #ty_generics as MessageProcessorWithFallback<AccountId>>::Fallback::handle_message(who, message),
                    Err(message_extraction_error) => Err(message_extraction_error.into())
                }
            }
        }
    };

    // Return the generated code as a TokenStream
    expanded.into()
}

#[cfg(test)]
fn derive_macro_for_test(
    item: proc_macro2::TokenStream,
) -> Result<proc_macro2::TokenStream, syn::Error> {
    let item = syn::parse2::<DeriveInput>(item)?;
    Ok(message_processor_trait_derive_impl(item))
}

#[cfg(test)]
mod tests {
    use super::derive_macro_for_test;
    use quote::quote;

    #[test]
    fn test_macro_expansion() {
        let input = quote! {
            #[derive(MessageProcessorDerive)]
            struct TestProcessor<T, U, V>(PhantomData<T, U, V>);
        };

        let expected_output = r##"
impl<
    T,
    U,
    V,
    AccountId,
> snowbridge_inbound_queue_primitives::v2::MessageProcessor<AccountId>
for TestProcessor<T, U, V>
where
    TestProcessor<T, U, V>: MessageProcessorWithFallback<AccountId>,
{
    fn can_process_message(who: &AccountId, message: &Message) -> bool {
        let result = TestProcessor::try_extract_message(who, message);
        match result {
            Ok(_) => true,
            Err(MessageExtractionError::InvalidMessage { .. }) => true,
            Err(MessageExtractionError::Other { .. }) => true,
            Err(MessageExtractionError::UnsupportedMessage { .. }) => false,
        }
    }
    fn process_message(
        who: AccountId,
        message: Message,
    ) -> Result<
        [u8; 32],
        snowbridge_inbound_queue_primitives::v2::MessageProcessorError,
    > {
        let result = TestProcessor::try_extract_message(&who, &message);
        match result {
            Ok(extracted_message) => {
                TestProcessor::process_extracted_message(who, extracted_message)
            }
            Err(MessageExtractionError::InvalidMessage { .. }) => {
                <TestProcessor<
                    T,
                    U,
                    V,
                > as MessageProcessorWithFallback<
                    AccountId,
                >>::Fallback::handle_message(who, message)
            }
            Err(message_extraction_error) => Err(message_extraction_error.into()),
        }
    }
}
"##;
        let out = derive_macro_for_test(input.into()).unwrap();

        let as_file = syn::parse_file(&out.to_string()).unwrap();

        // format it in a pretty way
        let formatted = prettyplease::unparse(&as_file);

        assert_eq!(expected_output.strip_prefix('\n').unwrap(), formatted);
    }
}
