
use proc_macro::TokenStream;
use quote::quote;
use syn;
use std::fmt::Display;

#[proc_macro_derive(Display)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_display_macro(&ast) // we're in a function, so we can split up code just like in regular Rust
}

fn impl_display_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let syn::Data::Struct(s) = &ast.data else {
        return (quote!{
            compile_error!("used our display macro for an enum or union")
        }).into()
    };
    let fields = &s.fields;
    let mut tys = Vec::new();
    let mut idents = Vec::new();
    for v in fields.clone().into_iter() {
        let ty = v.ty;
        let ident = v.ident.expect("named fields are required");
        tys.push(ty);
        idents.push(ident);
    }

    let gen = quote! {
        impl ::std::fmt::Display for #name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error>{
                #( writeln!(f, "{}: {}", stringify!(#idents), stringify!(#tys))?; )*
                ; Ok(())
            }
        }
    };
    gen.into()
}


// this is similar to the one shown in class
#[proc_macro_attribute]
pub fn only_structs(attr: TokenStream, input: TokenStream, ) -> TokenStream {
    // ... find out if it’s a struct
    let ast : syn::DeriveInput = syn::parse(input).unwrap();
    let syn::Data::Struct(_) = &ast.data else {
        return (quote!{
            compile_error!("The only_structs macro only goes on structs")
        }).into()
    };
    (quote!{}).into()
}
