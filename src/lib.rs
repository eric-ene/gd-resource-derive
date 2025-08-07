extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn;

macro_rules! field_fmt {
    () => {
        "_{}_field"
    };
}

#[proc_macro_derive(GDResource)]
pub fn derive_resource_fn(_item: TokenStream) -> TokenStream {
    let ast = syn::parse(_item).unwrap();

    return impl_macro(&ast);
}

fn impl_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let data = &ast.data;

    let syn::Data::Struct(data) = data else {
        unimplemented!()
    };

    let fields_names = data.fields.iter().map(|field| {
        let field_name = &field.ident.clone().unwrap();
        let field_final = quote::format_ident!(field_fmt!(), field_name);

        quote! {
            #field_name: #field_final,
        }
    });

    let fields_gd_impls = data.fields.iter().map(|field| {
        let f_name = &field.ident.clone().unwrap();
        let ty = &field.ty;

        let ident_output = format_ident!(field_fmt!(), f_name);

        quote! {
            let _res = via.get(stringify!(#f_name));
            let Some(_res) = _res else {
                return Err(godot::prelude::ConvertError::new(format!("{} not a field on Self::Via", stringify!(#f_name))));
            }; 
            let _res = <#ty as FromGodot>::try_from_variant(&_res);
            let Some(#ident_output) = _res.ok() else {
                return Err(godot::prelude::ConvertError::new(format!("{} convert error", stringify!(#f_name))));
            };
        }
    });

    let fields_resource_impls = data.fields.iter().map(|field| {
        let f_name = &field.ident.clone().unwrap();
        let ty = &field.ty;

        let ident_output = format_ident!(field_fmt!(), f_name);

        quote! {
            let res: Result<#ty, _> = self.get(stringify!(#f_name)).try_to();
            let Some(#ident_output) = res.ok() else {
                return #name::default(); 
            };
        }
    });

    let fields_names_2 = fields_names.clone();

    let expanded = quote! {
        impl Into<#name> for godot::prelude::Resource {
                fn into(self) -> #name {
                    #(#fields_resource_impls)*
                

                    #name {
                        #(#fields_names_2)*
                    }
                }
        }

        impl godot::prelude::GodotConvert for #name {
            type Via = godot::prelude::Dictionary;
        }

        impl godot::prelude::FromGodot for #name {
            fn try_from_godot (via: Self::Via) -> std::result::Result<Self, godot::prelude::ConvertError> {
                #(#fields_gd_impls)*


                Ok(Self {
                    #(#fields_names)*
                })
            }
        }
    };

    expanded.into()
}
