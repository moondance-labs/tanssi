// Copyright (C) Moondance Labs Ltd.
// This file is part of Tanssi.

// Tanssi is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Tanssi is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Tanssi.  If not, see <http://www.gnu.org/licenses/>

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

/// Generates the implementation of the `MessageProcessor` trait for a given type.
///
/// This function is the core implementation behind the `#[derive(MessageProcessor)]` macro.
/// It automatically generates boilerplate code for processing messages by implementing
/// the `MessageProcessor<AccountId>` trait.
///
/// # Generated Code
///
/// The macro generates an implementation with two methods:
///
/// ## `can_process_message`
/// Determines whether this processor can handle a specific message by attempting to extract it.
/// Returns `true` in three cases:
/// - Message extraction succeeds (`Ok`)
/// - Message is invalid (`InvalidMessage`) - handled by fallback processor
/// - Internal error occurs (`Other`) - handled by fallback processor
/// Returns `false` only for `UnsupportedMessage`, allowing the next processor in the chain to try.
///
/// ## `process_message`
/// Processes the message and returns a message ID on success. The flow is:
/// 1. Attempts to extract the message using `try_extract_message`
/// 2. Calculates the message ID using `calculate_message_id`
/// 3. On success: processes the extracted message
/// 4. On `InvalidMessage`: delegates to the configured `Fallback` processor
/// 5. On other errors: returns the error
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

            fn process_message(who: AccountId, message: Message) -> Result<([u8; 32], Option<sp_runtime::Weight>), snowbridge_inbound_queue_primitives::v2::MessageProcessorError> {
                // Do extraction
                let result = #name::try_extract_message(&who, &message);
                let message_id = #name::calculate_message_id(&message);
                match result {
                    Ok(extracted_message) => #name::process_extracted_message(who, extracted_message).map(|optional_processing_weight| (message_id, optional_processing_weight)),
                    Err(MessageExtractionError::InvalidMessage { .. }) => <#name #ty_generics as MessageProcessorWithFallback<AccountId>>::Fallback::handle_message(who, message).map(|optional_fallback_weight| (message_id, optional_fallback_weight)),
                    Err(message_extraction_error) => Err(message_extraction_error.into())
                }
            }

            fn worst_case_message_processor_weight() -> Weight {
                <#name #ty_generics as MessageProcessorWithFallback<AccountId>>::worst_case_message_processor_weight()
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
        ([u8; 32], Option<sp_runtime::Weight>),
        snowbridge_inbound_queue_primitives::v2::MessageProcessorError,
    > {
        let result = TestProcessor::try_extract_message(&who, &message);
        let message_id = TestProcessor::calculate_message_id(&message);
        match result {
            Ok(extracted_message) => {
                TestProcessor::process_extracted_message(who, extracted_message)
                    .map(|optional_processing_weight| (
                        message_id,
                        optional_processing_weight,
                    ))
            }
            Err(MessageExtractionError::InvalidMessage { .. }) => {
                <TestProcessor<
                    T,
                    U,
                    V,
                > as MessageProcessorWithFallback<
                    AccountId,
                >>::Fallback::handle_message(who, message)
                    .map(|optional_fallback_weight| (
                        message_id,
                        optional_fallback_weight,
                    ))
            }
            Err(message_extraction_error) => Err(message_extraction_error.into()),
        }
    }
    fn worst_case_message_processor_weight() -> Weight {
        <TestProcessor<
            T,
            U,
            V,
        > as MessageProcessorWithFallback<
            AccountId,
        >>::worst_case_message_processor_weight()
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
