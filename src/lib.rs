extern crate proc_macro;
use proc_macro::TokenStream;
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
    let syn::Data::Struct(data) = data else {
        unimplemented!()
    };

    let fields_names = data.fields.iter().map(|field| {
        let field_name = &field.ident;

        quote! {
            #field_name: quote::format_ident!("_{}_final", #field_name),
        }
    });

    let fields_gd_impls = data.fields.iter().map(|field| {
        let f_name = &field.ident;
        let ty = &field.ty;

        quote! {
            let _res = <#ty as FromGodot>::try_from_godot(via.get(stringify!(#f_name)));
            let Some(quote::format_ident!("_{}_final", #f_name)) = _res.ok() else {
                return Err(godot::prelude::ConvertError::new(format!("{} convert error", stringify!(#f_name))))
            };
        }
    });

    let expanded = quote! {
        impl GodotConvert for #name {
            type Via = godot::prelude::Dictionary;
        }

        impl FromGodot for #name {
            fn try_from_godot (via: Self::via) -> std::result::Result<Self, godot::prelude::ConvertError> {
                #(#fields_gd_impls)*


                Ok(Self {
                    #(#fields_names)*
                })
            }
        }
    };

    expanded.into()
}
