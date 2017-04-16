
#![recursion_limit="200"]

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use quote::{Ident, Tokens};
use syn::{DeriveInput, Body, VariantData, Field, Ty};

#[proc_macro_derive(DynamicLayout)]
pub fn derive_dynamiclayout(input: TokenStream) -> TokenStream {
    let input_string = input.to_string();
    let ast = syn::parse_macro_input(&input_string).unwrap();
    let output = make_types(&ast);
    output.parse().unwrap()
}

fn make_types(ast: &DeriveInput) -> Tokens {
    if let Body::Struct(VariantData::Struct(ref fields)) = ast.body {
        let original_name = &ast.ident;
        let layout_name = Ident::new(original_name.to_string() + "Layout");
        let layout_fields = layout_fields(fields);
        let layout_field_spans = layout_field_spans(fields);
        let layout_init = layout_init(fields);
        let accessor_name = Ident::new(original_name.to_string() + "Accessor");
        let accessor_fields = accessor_fields(fields);
        let accessor_init = accessor_init(fields);
        quote! {

            impl dynamiclayout::DynamicLayout for #original_name {
                #[allow(dead_code)]
                fn load_layout(layout: &dynamiclayout::LoadStructLayout) -> Result<#layout_name, ()> {
                    <Self as dynamiclayout::LayoutDynamicField>::make_layout(&dynamiclayout::LayoutInfo::StructField(layout))
                }
            }

            pub struct #layout_name {
                #(#layout_fields),*
            }

            impl #layout_name {
                #[allow(dead_code)]
                pub fn accessor<'a>(&'a self, bytes: &'a mut[u8]) -> #accessor_name<'a> {
                    unsafe {
                        <#original_name as dynamiclayout::AccessDynamicField>::accessor_from_layout(self, bytes.as_mut_ptr())
                    }
                }
            }

            pub struct #accessor_name<'a> {
                #(#accessor_fields),*
            }

            impl dynamiclayout::LayoutDynamicField for #original_name {
                type Layout = #layout_name;

                fn make_layout(layout: &dynamiclayout::LayoutInfo) -> Result<Self::Layout, ()> {
                    if let dynamiclayout::LayoutInfo::StructField(ref layout) = *layout {
                        Ok(#layout_name {
                            #(#layout_init),*
                        })
                    } else {
                        Err(())
                    }
                }

                fn get_field_spans(layout: &Self::Layout) -> Box<Iterator<Item = dynamiclayout::FieldSpan>> {
                    Box::new(
                        ::std::iter::empty()
                        #(#layout_field_spans)*
                    )
                }
            }

            impl dynamiclayout::LayoutArrayDynamicField for #original_name {
                type Layout = Vec<#layout_name>;

                fn make_layout(layout: &dynamiclayout::LayoutInfo, _: usize) -> Result<Self::Layout, ()> {
                    if let $crate::LayoutInfo::StructArrayField(ref layouts) = *layout {
                        let mut output = Vec::with_capacity(layouts.len());
                        for input in layouts.iter() {
                            let layout_field = LayoutInfo::StructField(*input);
                            output.push(try!(<#original_name as LayoutDynamicField>::make_layout(&layout_field)));
                        }
                        Ok(output)
                    } else {
                        Err(())
                    }
                }

                fn get_field_spans(layout: &Self::Layout, _: usize) -> Box<Iterator<Item = dynamiclayout::FieldSpan>> {
                    let spans: Vec<_> = layout.iter().flat_map(|l| <#original_name as LayoutDynamicField>::get_field_spans(l)).collect();
                    Box::new(spans.into_iter())
                }
            }

            impl<'a> dynamiclayout::AccessDynamicField<'a> for #original_name {
                type Accessor = #accessor_name<'a>;

                unsafe fn accessor_from_layout(layout: &'a Self::Layout, bytes: *mut u8) -> Self::Accessor {
                    #accessor_name {
                        #(#accessor_init),*
                    }
                }
            }

            impl<'a> dynamiclayout::AccessArrayDynamicField<'a> for #original_name {
                type Accessor = Vec<#accessor_name<'a>>;

                unsafe fn accessor_from_layout(layout: &'a Self::Layout, bytes: *mut u8, _: usize) -> Self::Accessor {
                    layout.iter().map(|l| <#original_name as dynamiclayout::AccessDynamicField>::accessor_from_layout(l, bytes)).collect()
                }
            }
        }
    } else {
        panic!("Only structs with named fields are supported")
    }
}

fn layout_fields(fields: &Vec<Field>) -> Vec<Tokens> {
    fields.iter().map(|field| {
        let name = field.ident.clone().unwrap();
        let ty = &field.ty;
        match *ty {
            Ty::Array(ref inner_ty, _) => quote! { #name: <#inner_ty as dynamiclayout::LayoutArrayDynamicField>::Layout },
            _ => quote! { #name: <#ty as dynamiclayout::LayoutDynamicField>::Layout }
        }
    }).collect()
}

fn layout_init(fields: &Vec<Field>) -> Vec<Tokens> {
    fields.iter().map(|field| {
        let name = field.ident.clone().unwrap();
        let ty = &field.ty;
        match *ty {
            Ty::Array(ref inner_ty, ref size) => {
                quote! {
                    #name: layout
                        .get_field_layout(stringify!(#name))
                        .ok_or(())
                        .and_then(|l| <#inner_ty as dynamiclayout::LayoutArrayDynamicField>::make_layout(l, #size))?
                }},
            _ =>
                quote! {
                    #name: layout
                        .get_field_layout(stringify!(#name))
                        .ok_or(())
                        .and_then(<#ty as dynamiclayout::LayoutDynamicField>::make_layout)?
                }
        }
    }).collect()
}

fn layout_field_spans(fields: &Vec<Field>) -> Vec<Tokens> {
    fields.iter().map(|field| {
        let name = field.ident.clone().unwrap();
        let ty = &field.ty;
        match *ty {
            Ty::Array(ref inner_ty, ref size) => quote! { .chain(<#inner_ty as dynamiclayout::LayoutArrayDynamicField>::get_field_spans(&layout.#name, #size)) },
            _ => quote! { .chain(<#ty as dynamiclayout::LayoutDynamicField>::get_field_spans(&layout.#name)) }
        }
    }).collect()
}

fn accessor_fields(fields: &Vec<Field>) -> Vec<Tokens> {
    fields.iter().map(|field| {
        let name = field.ident.clone().unwrap();
        let ty = &field.ty;
        match *ty {
            Ty::Array(ref inner_ty, _) => quote! { #name : <#inner_ty as dynamiclayout::AccessArrayDynamicField<'a>>::Accessor },
            _ => quote! { #name : <#ty as dynamiclayout::AccessDynamicField<'a>>::Accessor }
        }
    }).collect()
}

fn accessor_init(fields: &Vec<Field>) -> Vec<Tokens> {
    fields.iter().map(|field| {
        let name = field.ident.clone().unwrap();
        let ty = &field.ty;
        match *ty {
            Ty::Array(ref inner_ty, ref size) => quote! { #name : <#inner_ty as dynamiclayout::AccessArrayDynamicField<'a>>::accessor_from_layout(&layout.#name, bytes, #size) },
            _ => quote! { #name : <#ty as dynamiclayout::AccessDynamicField<'a>>::accessor_from_layout(&layout.#name, bytes) }
        }
    }).collect()
}
