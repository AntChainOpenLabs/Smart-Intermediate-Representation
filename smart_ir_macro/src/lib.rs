// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

use quote::ToTokens;
use syn::{Data, DataStruct, Fields, Type};
extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

#[proc_macro_derive(MetaDataNode)]
pub fn derive_md_node(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = proc_macro2::TokenStream::from(input);

    let output = derive_md_node_impl(input);

    proc_macro::TokenStream::from(output)
}

fn derive_md_node_impl(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let ast: syn::DeriveInput = syn::parse2(input).unwrap();
    let name = ast.ident;
    quote! {
        impl MetaDataNode for #name {
            fn get_metadata(&self) -> &indexmap::IndexMap<String, u32> {
                &self.metadata
            }

            fn get_metadata_mut(&mut self) -> &mut indexmap::IndexMap<String, u32> {
                &mut self.metadata
            }
        }
    }
}

fn map_fields<F>(fields: &Fields, mapper: F) -> proc_macro2::TokenStream
where
    F: FnMut((&proc_macro2::Ident, &Type)) -> proc_macro2::TokenStream,
{
    proc_macro2::TokenStream::from_iter(
        fields
            .iter()
            .map(|field| (field.ident.as_ref().unwrap(), &field.ty))
            .map(mapper),
    )
}

/// metadata definition derive macro
/// attribute MetaDataKey must be set
/// This macro will generate getters and setters for all fields of the struct,
/// defining the conversion and registration of the struct to metadata
///
/// # Example
///
/// ```rust
/// use smart_ir::cfg::ir::{MetaData, IRContext, Literal, IntValue};
/// use smart_ir_macro::{MetadataDefinition, MetaDataKey};
///
/// #[derive(MetadataDefinition, PartialEq, Eq, Default)]
/// #[MetaDataKey(smart_foo_bar)]
/// struct FooBar {
///     foo: i32,
///     bar: String,
/// };
///
/// fn foo() {
///     let mut ctx = IRContext::default();
///     let mut instr = Instr::new(InstrDescription::br(0));
///     let foo_bar = FooBar::default()
///         .foo(10)
///         .bar("hello world".to_string());
///     FooBar::add_to_context(&mut ctx,&mut instr, &foo_bar);
///     let metadata = FooBar::get_from_context(&ctx, &instr);
///     assert!(metadata.is_none());
///     assert_eq!(foo_bar, metadata.unwrap());
/// }
///
/// ```
/// expanded rust code will be like:
/// ```rust
/// struct FooBar {
///     foo: i32,
///     bar: String,
/// };
///
/// impl FooBar {
///     pub fn foo(mut self, value: i32) -> Self {
///         self.foo = value;
///         self
///     }
///     pub fn get_foo(mut self) -> i32 {
///         self.foo
///     }
///     pub fn bar(mut self, value: String) -> Self {
///         self.bar = value;
///         self
///     }
///     pub fn get_bar(mut self) -> String {
///         self.bar
///     }
///     pub fn from(metadata: &MetaData) -> Result<Self, String> {
///         Ok(Self {
///             foo: metadata.get_operand(0).get_i32()?,
///             bar: metadata.get_operand(1).get_string()?,
///         })
///     }
///     pub fn to_metadata(&self) -> MetaData {
///     let mut metadata = MetaData::default();
///         metadata.push_field(Literal::Int(IntLiteral::I32(self.foo)));
///         metadata.push_field(Literal::Str(self.bar.clone()));
///         metadata
///     }
///     pub fn get_metadata_key() -> String {
///        "smart_foo_bar".to_string()
///     }
///     pub fn add_to_context(
///         ctx: &mut IRContext,
///         md_node: &mut dyn MetaDataNode,
///         loc: &DebugLocation,
///        ) {
///         let md_idx = ctx.add_metadata(loc.to_metadata());
///         let metadata = md_node.get_metadata_mut();
///         metadata.insert("smart_foo_bar".to_string(), md_idx);
///     }
///     pub fn get_from_context(ctx: &IRContext, md_node: &dyn MetaDataNode) -> Option<DebugLocation> {
///         let metadata = md_node.get_metadata();
///         let md_idx = metadata.get("smart_foo_bar")?;
///         DebugLocation::from(ctx.get_metadata(md_idx)?).ok()
///     }
/// }
/// ```
///
#[proc_macro_derive(MetadataDefinition, attributes(MetaDataKey))]
pub fn derive_md_definition(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = proc_macro2::TokenStream::from(input);

    let output = derive_md_definition_impl(input);
    proc_macro::TokenStream::from(output)
}

fn derive_md_definition_impl(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let ast: syn::DeriveInput = syn::parse2(input).unwrap();

    let struct_name = ast.ident;
    let mut meta_data_key = None;
    for attr in &ast.attrs {
        if attr.path().is_ident("MetaDataKey") {
            //#[MetaDataKey(...)]
            attr.parse_nested_meta(|meta| match meta.path.get_ident() {
                Some(id) => {
                    meta_data_key = Some(id.clone());
                    Ok(())
                }
                None => Err(syn::Error::new(
                    proc_macro2::Span::call_site(),
                    "expand MetadataDefinition failed: MetaDataKey isn't set",
                )),
            })
            .unwrap();
        }
    }
    match ast.data {
        Data::Struct(DataStruct { fields, .. }) => {
            let fields_access = map_fields(&fields, |(ident, ty)| {
                let getter_name = format_ident!("get_{}", ident);
                quote!(pub fn #ident(mut self, value: #ty) -> Self {
                        self.#ident = value;
                        self
                    }
                    pub fn #getter_name(&self) -> #ty {
                        self.#ident.clone()
                    }
                )
            });

            let fields_to_literals = map_fields(&fields, |(ident, ty)| field_to_literal(ident, ty));
            let mut fields_count = 0;
            let literals_to_fields = map_fields(&fields, |(ident, ty)| {
                let result = literal_to_field(fields_count, ident, ty);
                fields_count += 1;
                result
            });
            let meta_data_key = meta_data_key
                .unwrap_or_else(|| {
                    panic!("expand MetadataDefinition failed: MetaDataKey isn't set")
                })
                .to_string();
            let meta_data_key =
                syn::LitStr::new(meta_data_key.as_str(), proc_macro2::Span::call_site());

            quote!(
                impl #struct_name {
                    // getter and setter
                    #fields_access

                    pub fn from(metadata: &MetaData) -> Result<Self, String> {
                        Ok(Self {
                            #literals_to_fields
                        })
                    }

                    pub fn to_metadata(&self) -> MetaData {
                        let mut metadata = MetaData::default();
                        #fields_to_literals
                        metadata
                    }

                    pub fn get_metadata_key() -> String {
                        #meta_data_key.to_string()
                    }

                    pub fn add_to_context(
                        ctx: &IRContext,
                        md_node: &mut dyn MetaDataNode,
                        loc: &#struct_name,
                    ) {
                        let md_idx = ctx.add_metadata(loc.to_metadata());
                        let metadata = md_node.get_metadata_mut();
                        metadata.insert(#meta_data_key.to_string(), md_idx);
                    }
                    pub fn get_from_context(ctx: &IRContext, md_node: &dyn MetaDataNode) -> Option<#struct_name> {
                        let metadata = md_node.get_metadata();
                        let md_idx = metadata.get(#meta_data_key)?;
                        #struct_name::from(ctx.get_metadata(md_idx).as_ref()?).ok()
                    }
                }
            )
        }
        _ => panic!(),
    }
}

fn field_to_literal(ident: &proc_macro2::Ident, ty: &Type) -> proc_macro2::TokenStream {
    match ty {
        Type::Path(val) => {
            let ty_str = val.path.get_ident().unwrap().to_string();
            return match ty_str.as_str() {
                "String" => quote!(
                    metadata.push_field(Literal::Str(self.#ident.clone()));
                ),
                "bool" => quote!(
                    metadata.push_field(Literal::Bool(self.#ident));
                ),
                "i8" => quote!(
                    metadata.push_field(Literal::Int(IntLiteral::I8(self.#ident)));
                ),
                "i16" => quote!(
                    metadata.push_field(Literal::Int(IntLiteral::I16(self.#ident)));
                ),
                "i32" => quote!(
                    metadata.push_field(Literal::Int(IntLiteral::I32(self.#ident)));
                ),
                "i64" => quote!(
                    metadata.push_field(Literal::Int(IntLiteral::I64(self.#ident)));
                ),
                "i128" => quote!(
                    metadata.push_field(Literal::Int(IntLiteral::I128(self.#ident)));
                ),
                "u8" => quote!(
                    metadata.push_field(Literal:Int(IntLiteral:::U8(self.#ident)));
                ),
                "u16" => quote!(
                    metadata.push_field(Literal::Int(IntLiteral::U16(self.#ident)));
                ),
                "u32" => quote!(
                    metadata.push_field(Literal::Int(IntLiteral::U32(self.#ident)));
                ),
                "u64" => quote!(
                    metadata.push_field(Literal::Int(IntLiteral::U64(self.#ident)));
                ),
                "u128" => quote!(
                    metadata.push_field(Literal::Int(IntLiteral::U128(self.#ident)));
                ),
                _ => {
                    let token_stream = ty.to_token_stream();
                    panic!(
                        "{}",
                        format!(
                        "MetadataDefinition expand failed: unsupported field ty: {token_stream}"
                    )
                    )
                }
            };
        }
        _ => {
            let token_stream = ty.to_token_stream();
            panic!(
                "{}",
                format!("MetadataDefinition expand failed: unsupported field ty: {token_stream}",)
            )
        }
    }
}

fn literal_to_field(
    field_idx: i32,
    ident: &proc_macro2::Ident,
    ty: &Type,
) -> proc_macro2::TokenStream {
    let field_idx_lit = syn::LitInt::new(
        field_idx.to_string().as_str(),
        proc_macro2::Span::call_site(),
    );
    match ty {
        Type::Path(val) => {
            let ty_str = val.path.get_ident().unwrap().to_string();
            let result = match ty_str.as_str() {
                "String" => quote!(
                    #ident: metadata.get_operand(#field_idx_lit).get_string()?,
                ),
                "bool" => quote!(
                    #ident: metadata.get_operand(#field_idx_lit).get_bool()?,
                ),
                "i8" => quote!(
                    #ident: metadata.get_operand(#field_idx_lit).get_i8()?,
                ),
                "i16" => quote!(
                    #ident: metadata.get_operand(#field_idx_lit).get_i16()?,
                ),
                "i32" => quote!(
                    #ident: metadata.get_operand(#field_idx_lit).get_i32()?,
                ),
                "i64" => quote!(
                    #ident: metadata.get_operand(#field_idx_lit).get_i64()?,
                ),
                "i128" => quote!(
                    #ident: metadata.get_operand(#field_idx_lit).get_i128()?,
                ),
                "u8" => quote!(
                    #ident: metadata.get_operand(#field_idx_lit).get_u8()?,
                ),
                "u16" => quote!(
                    #ident: metadata.get_operand(#field_idx_lit).get_u16()?,
                ),
                "u32" => quote!(
                    #ident: metadata.get_operand(#field_idx_lit).get_u32()?,
                ),
                "u64" => quote!(
                    #ident: metadata.get_operand(#field_idx_lit).get_u64()?,
                ),
                "u128" => quote!(
                    #ident: metadata.get_operand(#field_idx_lit).get_u128()?,
                ),
                _ => {
                    let token_stream = ty.to_token_stream();
                    panic!(
                        "{}",
                        format!(
                        "MetadataDefinition expand failed: unsupported field ty: {token_stream}",
                    )
                    )
                }
            };
            result
        }
        _ => {
            let token_stream = ty.to_token_stream();
            panic!(
                "{}",
                format!("MetadataDefinition expand failed: unsupported field ty:  {token_stream}")
            )
        }
    }
}
