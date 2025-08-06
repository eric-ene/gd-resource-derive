extern crate proc_macro;
use proc_macro::{ TokenStream };
use quote::quote;
use syn;

#[proc_macro_derive(GDResource)]
pub fn derive_resource_fn(_item: TokenStream) -> TokenStream {
    let ast = syn::parse(_item).unwrap();

    return impl_macro(&ast);
}

fn impl_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let data = &ast.data;
    let syn::Data::Struct(data) = data else { unimplemented!() };

    let fields_names = data.fields.iter().map(|field| {
        let field_name = &field.ident;

        quote! {
            #field_name,
        }
    });

    let fields_gd_impls = data.fields.iter().map(|field| {
        let f_name = &field.ident;
        let ty = &field.ty;

        quote! {
            let res: Result<#ty, _> = self.get(stringify!(#f_name)).try_to();
            let Some(#f_name) = res.ok() else {
                return #name::default(); 
            };
        }
    });

    let expanded =
        quote! {
            impl Into<#name> for godot::prelude::Resource {
                fn into(self) -> #name {
                    #(#fields_gd_impls)*
                

                    #name {
                        #(#fields_names)*
                    }
                }
            } 
        };

    expanded.into()
}
