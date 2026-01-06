use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Error, ItemImpl, ItemTrait, parse_macro_input};

/// Transforms an English word to its plural form following English language rules.
fn to_plural(word: &str) -> String {
    if word.is_empty() {
        return String::new();
    }

    // Special cases and irregular plurals could be added here

    // Words ending in 'y' preceded by a consonant: change 'y' to 'ies'
    if word.ends_with('y') && word.len() > 1 {
        let second_last = word.chars().nth(word.len() - 2).unwrap();
        if !"aeiou".contains(second_last) {
            return format!("{}ies", &word[..word.len() - 1]);
        }
    }

    // Words ending in 's', 'x', 'z', 'ch', 'sh': add 'es'
    if word.ends_with('s')
        || word.ends_with('x')
        || word.ends_with('z')
        || word.ends_with("ch")
        || word.ends_with("sh")
    {
        return format!("{}es", word);
    }

    // Default case: add 's'
    format!("{}s", word)
}

pub fn uow_action_impl(args: TokenStream, input: TokenStream) -> TokenStream {
    enum ItemType {
        Trait(ItemTrait),
        Impl(ItemImpl),
    }

    fn create_action(
        entity_ident: &syn::Ident,
        entity_snake_ident: &syn::Ident,
        function_ident: &syn::Ident,
        item_type: &ItemType,
        thread_safe: bool,
    ) -> proc_macro2::TokenStream {
        let create_entity_repo = format_ident!("create_{}_repository", entity_snake_ident);

        match item_type {
            ItemType::Trait(_) => {
                quote! {
                    fn #function_ident(&self, #entity_snake_ident: &#entity_ident) -> Result<#entity_ident>;
                }
            }
            ItemType::Impl(_) => {
                if thread_safe {
                    quote! {
                        fn #function_ident(&self, #entity_snake_ident: &#entity_ident) -> Result<#entity_ident> {
                            use common::entities::#entity_ident;
                            use common::types::EntityId;
                            use common::direct_access::repository_factory;

                            let borrowed_transaction = self.transaction.lock().unwrap();
                            let mut repo = repository_factory::write::#create_entity_repo(
                                &borrowed_transaction.as_ref().expect("Transaction not started"),
                            );
                            let #entity_snake_ident = repo.create(&self.event_hub, #entity_snake_ident)?;
                            Ok(#entity_snake_ident)
                        }
                    }
                } else {
                    quote! {
                        fn #function_ident(&self, #entity_snake_ident: &#entity_ident) -> Result<#entity_ident> {
                            use common::entities::#entity_ident;
                            use common::types::EntityId;
                            use common::direct_access::repository_factory;
                            use std::borrow::Borrow;

                            let borrowed_transaction = self.transaction.borrow();
                            let mut repo = repository_factory::write::#create_entity_repo(
                                &borrowed_transaction.as_ref().expect("Transaction not started"),
                            );
                            let #entity_snake_ident = repo.create(&self.event_hub, #entity_snake_ident)?;
                            Ok(#entity_snake_ident)
                        }
                    }
                }
            }
        }
    }

    fn create_multi_action(
        entity_ident: &syn::Ident,
        entity_snake_ident: &syn::Ident,
        function_ident: &syn::Ident,
        item_type: &ItemType,
        thread_safe: bool,
    ) -> proc_macro2::TokenStream {
        let create_entity_repo = format_ident!("create_{}_repository", entity_snake_ident);
        let entity_snake_ident_str = entity_snake_ident.to_string();
        let entity_snake_ident_plural = format_ident!("{}", to_plural(&entity_snake_ident_str));

        match item_type {
            ItemType::Trait(_) => {
                quote! {
                    fn #function_ident(&self, #entity_snake_ident_plural: &[#entity_ident]) -> Result<Vec<#entity_ident>>;
                }
            }
            ItemType::Impl(_) => {
                if thread_safe {
                    quote! {
                        fn #function_ident(&self, #entity_snake_ident_plural: &[#entity_ident]) -> Result<Vec<#entity_ident>> {
                            use common::entities::#entity_ident;
                            use common::types::EntityId;
                            use common::direct_access::repository_factory;

                            let borrowed_transaction = self.transaction.lock().unwrap();
                            let mut repo = repository_factory::write::#create_entity_repo(
                                &borrowed_transaction.as_ref().expect("Transaction not started"),
                            );
                            let #entity_snake_ident_plural = repo.create_multi(&self.event_hub, #entity_snake_ident_plural)?;
                            Ok(#entity_snake_ident_plural)
                        }
                    }
                } else {
                    quote! {
                        fn #function_ident(&self, #entity_snake_ident_plural: &[#entity_ident]) -> Result<Vec<#entity_ident>> {
                            use common::entities::#entity_ident;
                            use common::types::EntityId;
                            use common::direct_access::repository_factory;
                            use std::borrow::Borrow;

                            let borrowed_transaction = self.transaction.borrow();
                            let mut repo = repository_factory::write::#create_entity_repo(
                                &borrowed_transaction.as_ref().expect("Transaction not started"),
                            );
                            let #entity_snake_ident_plural = repo.create_multi(&self.event_hub, #entity_snake_ident_plural)?;
                            Ok(#entity_snake_ident_plural)
                        }
                    }
                }
            }
        }
    }

    fn get_action(
        entity_ident: &syn::Ident,
        entity_snake_ident: &syn::Ident,
        function_ident: &syn::Ident,
        item_type: &ItemType,
        thread_safe: bool,
    ) -> proc_macro2::TokenStream {
        let create_entity_repo = format_ident!("create_{}_repository", entity_snake_ident);

        match item_type {
            ItemType::Trait(_) => {
                quote! {
                    fn #function_ident(&self, id: &EntityId) -> Result<Option<#entity_ident>>;
                }
            }
            ItemType::Impl(_) => {
                if thread_safe {
                    quote! {
                        fn #function_ident(&self, id: &EntityId) -> Result<Option<#entity_ident>> {
                            use common::entities::#entity_ident;
                            use common::types::EntityId;
                            use common::direct_access::repository_factory;

                            let borrowed_transaction = self.transaction.lock().unwrap();
                            let repo = repository_factory::write::#create_entity_repo(
                                &borrowed_transaction.as_ref().expect("Transaction not started"),
                            );
                            let #entity_snake_ident = repo.get(id)?;
                            Ok(#entity_snake_ident)
                        }
                    }
                } else {
                    quote! {
                        fn #function_ident(&self, id: &EntityId) -> Result<Option<#entity_ident>> {
                            use common::entities::#entity_ident;
                            use common::types::EntityId;
                            use common::direct_access::repository_factory;
                            use std::borrow::Borrow;

                            let borrowed_transaction = self.transaction.borrow();
                            let repo = repository_factory::write::#create_entity_repo(
                                &borrowed_transaction.as_ref().expect("Transaction not started"),
                            );
                            let #entity_snake_ident = repo.get(id)?;
                            Ok(#entity_snake_ident)
                        }
                    }
                }
            }
        }
    }

    fn update_action(
        entity_ident: &syn::Ident,
        entity_snake_ident: &syn::Ident,
        function_ident: &syn::Ident,
        item_type: &ItemType,
        thread_safe: bool,
    ) -> proc_macro2::TokenStream {
        let create_entity_repo = format_ident!("create_{}_repository", entity_snake_ident);

        match item_type {
            ItemType::Trait(_) => {
                quote! {
                    fn #function_ident(&self, #entity_snake_ident: &#entity_ident) -> Result<#entity_ident>;
                }
            }
            ItemType::Impl(_) => {
                if thread_safe {
                    quote! {
                        fn #function_ident(&self, #entity_snake_ident: &#entity_ident) -> Result<#entity_ident> {
                            use common::entities::#entity_ident;
                            use common::types::EntityId;
                            use common::direct_access::repository_factory;

                            let borrowed_transaction = self.transaction.lock().unwrap();
                            let mut repo = repository_factory::write::#create_entity_repo(
                                &borrowed_transaction.as_ref().expect("Transaction not started"),
                            );
                            let #entity_snake_ident = repo.update(&self.event_hub, #entity_snake_ident)?;
                            Ok(#entity_snake_ident)
                        }
                    }
                } else {
                    quote! {
                        fn #function_ident(&self, #entity_snake_ident: &#entity_ident) -> Result<#entity_ident> {
                            use common::entities::#entity_ident;
                            use common::types::EntityId;
                            use common::direct_access::repository_factory;
                            use std::borrow::Borrow;

                            let borrowed_transaction = self.transaction.borrow();
                            let mut repo = repository_factory::write::#create_entity_repo(
                                &borrowed_transaction.as_ref().expect("Transaction not started"),
                            );
                            let #entity_snake_ident = repo.update(&self.event_hub, #entity_snake_ident)?;
                            Ok(#entity_snake_ident)
                        }
                    }
                }
            }
        }
    }

    fn delete_action(
        _entity_ident: &syn::Ident,
        entity_snake_ident: &syn::Ident,
        function_ident: &syn::Ident,
        item_type: &ItemType,
        thread_safe: bool,
    ) -> proc_macro2::TokenStream {
        let create_entity_repo = format_ident!("create_{}_repository", entity_snake_ident);

        match item_type {
            ItemType::Trait(_) => {
                quote! {
                    fn #function_ident(&self, id: &EntityId) -> Result<()>;
                }
            }
            ItemType::Impl(_) => {
                if thread_safe {
                    quote! {
                        fn #function_ident(&self, id: &EntityId) -> Result<()> {
                            use common::types::EntityId;
                            use common::direct_access::repository_factory;

                            let borrowed_transaction = self.transaction.lock().unwrap();
                            let mut repo = repository_factory::write::#create_entity_repo(
                                &borrowed_transaction.as_ref().expect("Transaction not started"),
                            );
                            repo.delete(&self.event_hub, id)?;
                            Ok(())
                        }
                    }
                } else {
                    quote! {
                        fn #function_ident(&self, id: &EntityId) -> Result<()> {
                            use common::types::EntityId;
                            use common::direct_access::repository_factory;
                            use std::borrow::Borrow;

                            let borrowed_transaction = self.transaction.borrow();
                            let mut repo = repository_factory::write::#create_entity_repo(
                                &borrowed_transaction.as_ref().expect("Transaction not started"),
                            );
                            repo.delete(&self.event_hub, id)?;
                            Ok(())
                        }
                    }
                }
            }
        }
    }

    fn get_multi_action(
        entity_ident: &syn::Ident,
        entity_snake_ident: &syn::Ident,
        function_ident: &syn::Ident,
        item_type: &ItemType,
        thread_safe: bool,
    ) -> proc_macro2::TokenStream {
        let create_entity_repo = format_ident!("create_{}_repository", entity_snake_ident);

        match item_type {
            ItemType::Trait(_) => {
                quote! {
                    fn #function_ident(&self, ids: &[EntityId]) -> Result<Vec<Option<#entity_ident>>>;
                }
            }
            ItemType::Impl(_) => {
                if thread_safe {
                    quote! {
                        fn #function_ident(&self, ids: &[EntityId]) -> Result<Vec<Option<#entity_ident>>> {
                            use common::entities::#entity_ident;
                            use common::types::EntityId;
                            use common::direct_access::repository_factory;

                            let borrowed_transaction = self.transaction.lock().unwrap();
                            let repo = repository_factory::write::#create_entity_repo(
                                &borrowed_transaction.as_ref().expect("Transaction not started"),
                            );
                            let value = repo.get_multi(ids)?;
                            Ok(value)
                        }
                    }
                } else {
                    quote! {
                        fn #function_ident(&self, ids: &[EntityId]) -> Result<Vec<Option<#entity_ident>>> {
                            use common::entities::#entity_ident;
                            use common::types::EntityId;
                            use common::direct_access::repository_factory;
                            use std::borrow::Borrow;

                            let borrowed_transaction = self.transaction.borrow();
                            let repo = repository_factory::write::#create_entity_repo(
                                &borrowed_transaction.as_ref().expect("Transaction not started"),
                            );
                            let value = repo.get_multi(ids)?;
                            Ok(value)
                        }
                    }
                }
            }
        }
    }

    fn update_multi_action(
        entity_ident: &syn::Ident,
        entity_snake_ident: &syn::Ident,
        function_ident: &syn::Ident,
        item_type: &ItemType,
        thread_safe: bool,
    ) -> proc_macro2::TokenStream {
        let create_entity_repo = format_ident!("create_{}_repository", entity_snake_ident);
        let entity_snake_ident_str = entity_snake_ident.to_string();
        let entity_snake_ident_plural = format_ident!("{}", to_plural(&entity_snake_ident_str));

        match item_type {
            ItemType::Trait(_) => {
                quote! {
                    fn #function_ident(&self, #entity_snake_ident_plural: &[#entity_ident]) -> Result<Vec<#entity_ident>>;
                }
            }
            ItemType::Impl(_) => {
                if thread_safe {
                    quote! {
                        fn #function_ident(&self, #entity_snake_ident_plural: &[#entity_ident]) -> Result<Vec<#entity_ident>> {
                            use common::entities::#entity_ident;
                            use common::types::EntityId;
                            use std::borrow::Borrow;
                            use common::direct_access::repository_factory;

                            let borrowed_transaction = self.transaction.lock().unwrap();
                            let mut repo = repository_factory::write::#create_entity_repo(
                                &borrowed_transaction.as_ref().expect("Transaction not started"),
                            );
                            let #entity_snake_ident_plural = repo.update_multi(&self.event_hub, #entity_snake_ident_plural)?;
                            Ok(#entity_snake_ident_plural)
                        }
                    }
                } else {
                    quote! {
                        fn #function_ident(&self, #entity_snake_ident_plural: &[#entity_ident]) -> Result<Vec<#entity_ident>> {
                            use common::entities::#entity_ident;
                            use common::types::EntityId;
                            use std::borrow::Borrow;
                            use common::direct_access::repository_factory;

                            let borrowed_transaction = self.transaction.borrow();
                            let mut repo = repository_factory::write::#create_entity_repo(
                                &borrowed_transaction.as_ref().expect("Transaction not started"),
                            );
                            let #entity_snake_ident_plural = repo.update_multi(&self.event_hub, #entity_snake_ident_plural)?;
                            Ok(#entity_snake_ident_plural)
                        }
                    }
                }
            }
        }
    }

    fn delete_multi_action(
        _entity_ident: &syn::Ident,
        entity_snake_ident: &syn::Ident,
        function_ident: &syn::Ident,
        item_type: &ItemType,
        thread_safe: bool,
    ) -> proc_macro2::TokenStream {
        let create_entity_repo = format_ident!("create_{}_repository", entity_snake_ident);

        match item_type {
            ItemType::Trait(_) => {
                quote! {
                    fn #function_ident(&self, ids: &[EntityId]) -> Result<()>;
                }
            }
            ItemType::Impl(_) => {
                if thread_safe {
                    quote! {
                        fn #function_ident(&self, ids: &[EntityId]) -> Result<()> {
                            use common::types::EntityId;
                            use common::direct_access::repository_factory;

                            let borrowed_transaction = self.transaction.lock().unwrap();
                            let mut repo = repository_factory::write::#create_entity_repo(
                                &borrowed_transaction.as_ref().expect("Transaction not started"),
                            );
                            repo.delete_multi(&self.event_hub, ids)?;
                            Ok(())
                        }
                    }
                } else {
                    quote! {
                        fn #function_ident(&self, ids: &[EntityId]) -> Result<()> {
                            use common::types::EntityId;
                            use common::direct_access::repository_factory;
                            use std::borrow::Borrow;

                            let borrowed_transaction = self.transaction.borrow();
                            let mut repo = repository_factory::write::#create_entity_repo(
                                &borrowed_transaction.as_ref().expect("Transaction not started"),
                            );
                            repo.delete_multi(&self.event_hub, ids)?;
                            Ok(())
                        }
                    }
                }
            }
        }
    }

    fn get_ro_action(
        entity_ident: &syn::Ident,
        entity_snake_ident: &syn::Ident,
        function_ident: &syn::Ident,
        item_type: &ItemType,
        thread_safe: bool,
    ) -> proc_macro2::TokenStream {
        let create_entity_repo = format_ident!("create_{}_repository", entity_snake_ident);

        match item_type {
            ItemType::Trait(_) => {
                quote! {
                    fn #function_ident(&self, id: &EntityId) -> Result<Option<#entity_ident>>;
                }
            }
            ItemType::Impl(_) => {
                if thread_safe {
                    quote! {
                        fn #function_ident(&self, id: &EntityId) -> Result<Option<#entity_ident>> {
                            use common::entities::#entity_ident;
                            use common::types::EntityId;
                            use common::direct_access::repository_factory;
                            use std::borrow::Borrow;

                            let borrowed_transaction = self.transaction.lock().unwrap();
                            let repo = repository_factory::read::#create_entity_repo(
                                &borrowed_transaction.as_ref().expect("Transaction not started"),
                            );
                            let #entity_snake_ident = repo.get(id)?;
                            Ok(#entity_snake_ident)
                        }
                    }
                } else {
                    quote! {
                        fn #function_ident(&self, id: &EntityId) -> Result<Option<#entity_ident>> {
                            use common::entities::#entity_ident;
                            use common::types::EntityId;
                            use common::direct_access::repository_factory;
                            use std::borrow::Borrow;

                            let borrowed_transaction = self.transaction.borrow();
                            let repo = repository_factory::read::#create_entity_repo(
                                &borrowed_transaction.as_ref().expect("Transaction not started"),
                            );
                            let #entity_snake_ident = repo.get(id)?;
                            Ok(#entity_snake_ident)
                        }
                    }
                }
            }
        }
    }

    fn get_multi_ro_action(
        entity_ident: &syn::Ident,
        entity_snake_ident: &syn::Ident,
        function_ident: &syn::Ident,
        item_type: &ItemType,
        thread_safe: bool,
    ) -> proc_macro2::TokenStream {
        let create_entity_repo = format_ident!("create_{}_repository", entity_snake_ident);

        match item_type {
            ItemType::Trait(_) => {
                quote! {
                    fn #function_ident(&self, ids: &[EntityId]) -> Result<Vec<Option<#entity_ident>>>;
                }
            }
            ItemType::Impl(_) => {
                if thread_safe {
                    quote! {
                        fn #function_ident(&self, ids: &[EntityId]) -> Result<Vec<Option<#entity_ident>>> {
                            use common::entities::#entity_ident;
                            use common::types::EntityId;
                            use common::direct_access::repository_factory;

                            let borrowed_transaction = self.transaction.lock().unwrap();
                            let repo = repository_factory::read::#create_entity_repo(
                                &borrowed_transaction.as_ref().expect("Transaction not started"),
                            );
                            let value = repo.get_multi(ids)?;
                            Ok(value)
                        }
                    }
                } else {
                    quote! {
                        fn #function_ident(&self, ids: &[EntityId]) -> Result<Vec<Option<#entity_ident>>> {
                            use common::entities::#entity_ident;
                            use common::types::EntityId;
                            use common::direct_access::repository_factory;
                            use std::borrow::Borrow;

                            let borrowed_transaction = self.transaction.borrow();
                            let repo = repository_factory::read::#create_entity_repo(
                                &borrowed_transaction.as_ref().expect("Transaction not started"),
                            );
                            let value = repo.get_multi(ids)?;
                            Ok(value)
                        }
                    }
                }
            }
        }
    }

    fn get_relationship_action(
        entity_ident: &syn::Ident,
        entity_snake_ident: &syn::Ident,
        function_ident: &syn::Ident,
        item_type: &ItemType,
        thread_safe: bool,
    ) -> proc_macro2::TokenStream {
        let create_entity_repo = format_ident!("create_{}_repository", entity_snake_ident);
        let relationship_field_type = format_ident!("{}RelationshipField", entity_ident);

        match item_type {
            ItemType::Trait(_) => {
                quote! {
                    fn #function_ident(
                        &self,
                        id: &EntityId,
                        field: &common::direct_access::#entity_snake_ident::#relationship_field_type,
                    ) -> Result<Vec<EntityId>>;
                }
            }
            ItemType::Impl(_) => {
                if thread_safe {
                    quote! {
                        fn #function_ident(
                            &self,
                            id: &EntityId,
                            field: &common::direct_access::#entity_snake_ident::#relationship_field_type,
                        ) -> Result<Vec<EntityId>> {
                            use common::types::EntityId;
                            use common::direct_access::repository_factory;
                            use common::direct_access::#entity_snake_ident::#relationship_field_type;

                            let borrowed_transaction = self.transaction.lock().unwrap();
                            let mut repo = repository_factory::write::#create_entity_repo(
                                &borrowed_transaction.as_ref().expect("Transaction not started"),
                            );
                            let value = repo.get_relationship(id, field)?;
                            Ok(value)
                        }
                    }
                } else {
                    quote! {
                        fn #function_ident(
                            &self,
                            id: &EntityId,
                            field: &common::direct_access::#entity_snake_ident::#relationship_field_type,
                        ) -> Result<Vec<EntityId>> {
                            use common::types::EntityId;
                            use common::direct_access::repository_factory;
                            use common::direct_access::#entity_snake_ident::#relationship_field_type;
                            use std::borrow::Borrow;

                            let borrowed_transaction = self.transaction.borrow();
                            let mut repo = repository_factory::write::#create_entity_repo(
                                &borrowed_transaction.as_ref().expect("Transaction not started"),
                            );
                            let value = repo.get_relationship(id, field)?;
                            Ok(value)
                        }
                    }
                }
            }
        }
    }

    fn get_relationships_from_right_ids_action(
        entity_ident: &syn::Ident,
        entity_snake_ident: &syn::Ident,
        function_ident: &syn::Ident,
        item_type: &ItemType,
        thread_safe: bool,
    ) -> proc_macro2::TokenStream {
        let create_entity_repo = format_ident!("create_{}_repository", entity_snake_ident);
        let relationship_field_type = format_ident!("{}RelationshipField", entity_ident);

        match item_type {
            ItemType::Trait(_) => {
                quote! {
                    fn #function_ident(
                        &self,
                        field: &common::direct_access::#entity_snake_ident::#relationship_field_type,
                        right_ids: &[EntityId],
                    ) -> Result<Vec<(EntityId, Vec<EntityId>)>>;
                }
            }
            ItemType::Impl(_) => {
                if thread_safe {
                    quote! {
                        fn #function_ident(
                            &self,
                            field: &common::direct_access::#entity_snake_ident::#relationship_field_type,
                            right_ids: &[EntityId],
                        ) -> Result<Vec<(EntityId, Vec<EntityId>)>> {
                            use common::types::EntityId;
                            use common::direct_access::repository_factory;
                            use common::direct_access::#entity_snake_ident::#relationship_field_type;

                            let borrowed_transaction = self.transaction.lock().unwrap();
                            let repo = repository_factory::write::#create_entity_repo(
                                &borrowed_transaction.as_ref().expect("Transaction not started"),
                            );
                            let value = repo.get_relationships_from_right_ids(field, right_ids)?;
                            Ok(value)
                        }
                    }
                } else {
                    quote! {
                        fn #function_ident(
                            &self,
                            field: &common::direct_access::#entity_snake_ident::#relationship_field_type,
                            right_ids: &[EntityId],
                        ) -> Result<Vec<(EntityId, Vec<EntityId>)>> {
                            use common::types::EntityId;
                            use common::direct_access::repository_factory;
                            use common::direct_access::#entity_snake_ident::#relationship_field_type;
                            use std::borrow::Borrow;

                            let borrowed_transaction = self.transaction.borrow();
                            let repo = repository_factory::write::#create_entity_repo(
                                &borrowed_transaction.as_ref().expect("Transaction not started"),
                            );
                            let value = repo.get_relationships_from_right_ids(field, right_ids)?;
                            Ok(value)
                        }
                    }
                }
            }
        }
    }

    fn get_relationship_ro_action(
        entity_ident: &syn::Ident,
        entity_snake_ident: &syn::Ident,
        function_ident: &syn::Ident,
        item_type: &ItemType,
        thread_safe: bool,
    ) -> proc_macro2::TokenStream {
        let create_entity_repo = format_ident!("create_{}_repository", entity_snake_ident);
        let relationship_field_type = format_ident!("{}RelationshipField", entity_ident);

        match item_type {
            ItemType::Trait(_) => {
                quote! {
                    fn #function_ident(
                        &self,
                        id: &EntityId,
                        field: &common::direct_access::#entity_snake_ident::#relationship_field_type,
                    ) -> Result<Vec<EntityId>>;
                }
            }
            ItemType::Impl(_) => {
                if thread_safe {
                    quote! {
                        fn #function_ident(
                            &self,
                            id: &EntityId,
                            field: &common::direct_access::#entity_snake_ident::#relationship_field_type,
                        ) -> Result<Vec<EntityId>> {
                            use common::types::EntityId;
                            use common::direct_access::repository_factory;
                            use common::direct_access::#entity_snake_ident::#relationship_field_type;

                            let borrowed_transaction = self.transaction.lock().unwrap();
                            let repo = repository_factory::read::#create_entity_repo(
                                &borrowed_transaction.as_ref().expect("Transaction not started"),
                            );
                            let value = repo.get_relationship(id, field)?;
                            Ok(value)
                        }
                    }
                } else {
                    quote! {
                        fn #function_ident(
                            &self,
                            id: &EntityId,
                            field: &common::direct_access::#entity_snake_ident::#relationship_field_type,
                        ) -> Result<Vec<EntityId>> {
                            use common::types::EntityId;
                            use common::direct_access::repository_factory;
                            use common::direct_access::#entity_snake_ident::#relationship_field_type;
                            use std::borrow::Borrow;

                            let borrowed_transaction = self.transaction.borrow();
                            let repo = repository_factory::read::#create_entity_repo(
                                &borrowed_transaction.as_ref().expect("Transaction not started"),
                            );
                            let value = repo.get_relationship(id, field)?;
                            Ok(value)
                        }
                    }
                }
            }
        }
    }

    fn get_relationships_from_right_ids_ro_action(
        entity_ident: &syn::Ident,
        entity_snake_ident: &syn::Ident,
        function_ident: &syn::Ident,
        item_type: &ItemType,
        thread_safe: bool,
    ) -> proc_macro2::TokenStream {
        let create_entity_repo = format_ident!("create_{}_repository", entity_snake_ident);
        let relationship_field_type = format_ident!("{}RelationshipField", entity_ident);

        match item_type {
            ItemType::Trait(_) => {
                quote! {
                    fn #function_ident(
                        &self,
                        field: &common::direct_access::#entity_snake_ident::#relationship_field_type,
                        right_ids: &[EntityId],
                    ) -> Result<Vec<(EntityId, Vec<EntityId>)>>;
                }
            }
            ItemType::Impl(_) => {
                if thread_safe {
                    quote! {
                        fn #function_ident(
                            &self,
                            field: &common::direct_access::#entity_snake_ident::#relationship_field_type,
                            right_ids: &[EntityId],
                        ) -> Result<Vec<(EntityId, Vec<EntityId>)>> {
                            use common::types::EntityId;
                            use common::direct_access::repository_factory;
                            use common::direct_access::#entity_snake_ident::#relationship_field_type;

                            let borrowed_transaction = self.transaction.lock().unwrap();
                            let repo = repository_factory::read::#create_entity_repo(
                                &borrowed_transaction.as_ref().expect("Transaction not started"),
                            );
                            let value = repo.get_relationships_from_right_ids(field, right_ids)?;
                            Ok(value)
                        }
                    }
                } else {
                    quote! {
                        fn #function_ident(
                            &self,
                            field: &common::direct_access::#entity_snake_ident::#relationship_field_type,
                            right_ids: &[EntityId],
                        ) -> Result<Vec<(EntityId, Vec<EntityId>)>> {
                            use common::types::EntityId;
                            use common::direct_access::repository_factory;
                            use common::direct_access::#entity_snake_ident::#relationship_field_type;
                            use std::borrow::Borrow;

                            let borrowed_transaction = self.transaction.borrow();
                            let repo = repository_factory::read::#create_entity_repo(
                                &borrowed_transaction.as_ref().expect("Transaction not started"),
                            );
                            let value = repo.get_relationships_from_right_ids(field, right_ids)?;
                            Ok(value)
                        }
                    }
                }
            }
        }
    }

    fn set_relationship_action(
        entity_ident: &syn::Ident,
        entity_snake_ident: &syn::Ident,
        function_ident: &syn::Ident,
        item_type: &ItemType,
        thread_safe: bool,
    ) -> proc_macro2::TokenStream {
        let create_entity_repo = format_ident!("create_{}_repository", entity_snake_ident);
        let relationship_field_type = format_ident!("{}RelationshipField", entity_ident);

        match item_type {
            ItemType::Trait(_) => {
                quote! {
                    fn #function_ident(
                        &self,
                        id: &EntityId,
                        field: &common::direct_access::#entity_snake_ident::#relationship_field_type,
                        right_ids: &[EntityId],
                    ) -> Result<()>;
                }
            }
            ItemType::Impl(_) => {
                if thread_safe {
                    quote! {
                        fn #function_ident(
                            &self,
                            id: &EntityId,
                            field: &common::direct_access::#entity_snake_ident::#relationship_field_type,
                            right_ids: &[EntityId],
                        ) -> Result<()> {
                            use common::types::EntityId;
                            use common::direct_access::repository_factory;
                            use common::direct_access::#entity_snake_ident::#relationship_field_type;

                            let borrowed_transaction = self.transaction.lock().unwrap();
                            let mut repo = repository_factory::write::#create_entity_repo(
                                &borrowed_transaction.as_ref().expect("Transaction not started"),
                            );
                            repo.set_relationship(&self.event_hub, id, field, right_ids)?;
                            Ok(())
                        }
                    }
                } else {
                    quote! {
                        fn #function_ident(
                            &self,
                            id: &EntityId,
                            field: &common::direct_access::#entity_snake_ident::#relationship_field_type,
                            right_ids: &[EntityId],
                        ) -> Result<()> {
                            use common::types::EntityId;
                            use common::direct_access::repository_factory;
                            use common::direct_access::#entity_snake_ident::#relationship_field_type;
                            use std::borrow::Borrow;

                            let borrowed_transaction = self.transaction.borrow();
                            let mut repo = repository_factory::write::#create_entity_repo(
                                &borrowed_transaction.as_ref().expect("Transaction not started"),
                            );
                            repo.set_relationship(&self.event_hub, id, field, right_ids)?;
                            Ok(())
                        }
                    }
                }
            }
        }
    }

    fn set_relationship_multi_action(
        entity_ident: &syn::Ident,
        entity_snake_ident: &syn::Ident,
        function_ident: &syn::Ident,
        item_type: &ItemType,
        thread_safe: bool,
    ) -> proc_macro2::TokenStream {
        let create_entity_repo = format_ident!("create_{}_repository", entity_snake_ident);
        let relationship_field_type = format_ident!("{}RelationshipField", entity_ident);

        match item_type {
            ItemType::Trait(_) => {
                quote! {
                    fn #function_ident(
                        &self,
                        field: &common::direct_access::#entity_snake_ident::#relationship_field_type,
                        relationships: Vec<(EntityId, Vec<EntityId>)>,
                    ) -> Result<()>;
                }
            }
            ItemType::Impl(_) => {
                if thread_safe {
                    quote! {
                        fn #function_ident(
                            &self,
                            field: &common::direct_access::#entity_snake_ident::#relationship_field_type,
                            relationships: Vec<(EntityId, Vec<EntityId>)>,
                        ) -> Result<()> {
                            use common::types::EntityId;
                            use common::direct_access::repository_factory;
                            use common::direct_access::#entity_snake_ident::#relationship_field_type;

                            let borrowed_transaction = self.transaction.lock().unwrap();
                            let mut repo = repository_factory::write::#create_entity_repo(
                                &borrowed_transaction.as_ref().expect("Transaction not started"),
                            );
                            repo.set_relationship_multi(&self.event_hub, field, relationships)?;
                            Ok(())
                        }
                    }
                } else {
                    quote! {
                        fn #function_ident(
                            &self,
                            field: &common::direct_access::#entity_snake_ident::#relationship_field_type,
                            relationships: Vec<(EntityId, Vec<EntityId>)>,
                        ) -> Result<()> {
                            use common::types::EntityId;
                            use common::direct_access::repository_factory;
                            use common::direct_access::#entity_snake_ident::#relationship_field_type;
                            use std::borrow::Borrow;

                            let borrowed_transaction = self.transaction.borrow();
                            let mut repo = repository_factory::write::#create_entity_repo(
                                &borrowed_transaction.as_ref().expect("Transaction not started"),
                            );
                            repo.set_relationship_multi(&self.event_hub, field, relationships)?;
                            Ok(())
                        }
                    }
                }
            }
        }
    }

    // Parse les arguments de l'attribut avec syn v2
    let args = parse_macro_input!(args with syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated);

    let mut entity_name = None;
    let mut action = None;
    let mut thread_safe = false;

    for arg in args.iter() {
        if let syn::Meta::NameValue(nv) = arg {
            if nv.path.is_ident("entity") {
                if let syn::Expr::Lit(expr_lit) = &nv.value {
                    if let syn::Lit::Str(litstr) = &expr_lit.lit {
                        entity_name = Some(litstr.value());
                    }
                }
            }
            if nv.path.is_ident("action") {
                if let syn::Expr::Lit(expr_lit) = &nv.value {
                    if let syn::Lit::Str(litstr) = &expr_lit.lit {
                        action = Some(litstr.value());
                    }
                }
            }
            if nv.path.is_ident("thread_safe") {
                if let syn::Expr::Lit(expr_lit) = &nv.value {
                    if let syn::Lit::Bool(litbool) = &expr_lit.lit {
                        thread_safe = litbool.value();
                    }
                }
            }
        } else if let syn::Meta::Path(path) = arg {
            if path.is_ident("thread_safe") {
                thread_safe = true;
            }
        }
    }

    let entity_name = match entity_name {
        Some(e) => e,
        None => {
            return Error::new_spanned(args.first().unwrap(), "Missing 'entity' argument")
                .to_compile_error()
                .into();
        }
    };
    let action = match action {
        Some(a) => a,
        None => {
            return Error::new_spanned(args.first().unwrap(), "Missing 'action' argument")
                .to_compile_error()
                .into();
        }
    };

    // Parse l'impl ou le trait d'entrée
    let item_type = if let Ok(item_trait) = syn::parse::<ItemTrait>(input.clone()) {
        ItemType::Trait(item_trait)
    } else {
        ItemType::Impl(parse_macro_input!(input as ItemImpl))
    };

    // Convertit le nom d'entité en snake_case
    let entity_name_snake_case = heck::AsSnakeCase(&entity_name).to_string();

    // Nom de la fonction à générer
    let function_name = match action.as_str() {
        "Create" => format!("create_{}", entity_name_snake_case),
        "CreateMulti" => format!("create_{}_multi", entity_name_snake_case),
        "Get" => format!("get_{}", entity_name_snake_case),
        "GetMulti" => format!("get_{}_multi", entity_name_snake_case),
        "Update" => format!("update_{}", entity_name_snake_case),
        "UpdateMulti" => format!("update_{}_multi", entity_name_snake_case),
        "Delete" => format!("delete_{}", entity_name_snake_case),
        "DeleteMulti" => format!("delete_{}_multi", entity_name_snake_case),
        "GetRO" => format!("get_{}", entity_name_snake_case),
        "GetMultiRO" => format!("get_{}_multi", entity_name_snake_case),
        "GetRelationship" => format!("get_{}_relationship", entity_name_snake_case),
        "GetRelationshipRO" => format!("get_{}_relationship", entity_name_snake_case),
        "GetRelationshipsFromRightIds" => {
            format!(
                "get_{}_relationships_from_right_ids",
                entity_name_snake_case
            )
        }
        "GetRelationshipsFromRightIdsRO" => {
            format!(
                "get_{}_relationships_from_right_ids",
                entity_name_snake_case
            )
        }
        "SetRelationship" => format!("set_{}_relationship", entity_name_snake_case),
        "SetRelationshipMulti" => format!("set_{}_relationship_multi", entity_name_snake_case),
        _ => {
            return Error::new_spanned(args.first().unwrap(), "Unknown action")
                .to_compile_error()
                .into();
        }
    };
    let function_ident = syn::Ident::new(&function_name, proc_macro2::Span::call_site());
    let entity_ident = syn::Ident::new(&entity_name, proc_macro2::Span::call_site());
    let entity_snake_ident =
        syn::Ident::new(&entity_name_snake_case, proc_macro2::Span::call_site());

    // Génère la méthode correspondante
    let method_tokens = match action.as_str() {
        "Create" => create_action(
            &entity_ident,
            &entity_snake_ident,
            &function_ident,
            &item_type,
            thread_safe,
        ),
        "CreateMulti" => create_multi_action(
            &entity_ident,
            &entity_snake_ident,
            &function_ident,
            &item_type,
            thread_safe,
        ),
        "Get" => get_action(
            &entity_ident,
            &entity_snake_ident,
            &function_ident,
            &item_type,
            thread_safe,
        ),
        "GetMulti" => get_multi_action(
            &entity_ident,
            &entity_snake_ident,
            &function_ident,
            &item_type,
            thread_safe,
        ),
        "Update" => update_action(
            &entity_ident,
            &entity_snake_ident,
            &function_ident,
            &item_type,
            thread_safe,
        ),
        "UpdateMulti" => update_multi_action(
            &entity_ident,
            &entity_snake_ident,
            &function_ident,
            &item_type,
            thread_safe,
        ),
        "Delete" => delete_action(
            &entity_ident,
            &entity_snake_ident,
            &function_ident,
            &item_type,
            thread_safe,
        ),
        "DeleteMulti" => delete_multi_action(
            &entity_ident,
            &entity_snake_ident,
            &function_ident,
            &item_type,
            thread_safe,
        ),
        "GetRO" => get_ro_action(
            &entity_ident,
            &entity_snake_ident,
            &function_ident,
            &item_type,
            thread_safe,
        ),
        "GetMultiRO" => get_multi_ro_action(
            &entity_ident,
            &entity_snake_ident,
            &function_ident,
            &item_type,
            thread_safe,
        ),
        "GetRelationship" => get_relationship_action(
            &entity_ident,
            &entity_snake_ident,
            &function_ident,
            &item_type,
            thread_safe,
        ),
        "GetRelationshipRO" => get_relationship_ro_action(
            &entity_ident,
            &entity_snake_ident,
            &function_ident,
            &item_type,
            thread_safe,
        ),
        "GetRelationshipsFromRightIds" => get_relationships_from_right_ids_action(
            &entity_ident,
            &entity_snake_ident,
            &function_ident,
            &item_type,
            thread_safe,
        ),
        "GetRelationshipsFromRightIdsRO" => get_relationships_from_right_ids_ro_action(
            &entity_ident,
            &entity_snake_ident,
            &function_ident,
            &item_type,
            thread_safe,
        ),
        "SetRelationship" => set_relationship_action(
            &entity_ident,
            &entity_snake_ident,
            &function_ident,
            &item_type,
            thread_safe,
        ),
        "SetRelationshipMulti" => set_relationship_multi_action(
            &entity_ident,
            &entity_snake_ident,
            &function_ident,
            &item_type,
            thread_safe,
        ),
        _ => unreachable!(),
    };

    // Ajoute la méthode au trait ou à l'implémentation
    let item_impl = match item_type {
        ItemType::Trait(mut item_trait) => {
            // Ajoute la méthode au trait
            item_trait
                .items
                .push(syn::TraitItem::Verbatim(method_tokens));
            quote!(#item_trait)
        }
        ItemType::Impl(mut item_impl) => {
            // Ajoute la méthode à l'implémentation
            item_impl.items.push(syn::ImplItem::Verbatim(method_tokens));
            quote!(#item_impl)
        }
    };

    // Retourne le code modifié
    quote!(#item_impl).into()
}
