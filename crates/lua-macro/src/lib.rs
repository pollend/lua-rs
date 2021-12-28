// print_caller_location/src/lib.rs
use proc_macro::TokenStream;
use quote::{quote};
use syn::{Ident, ExprClosure, Token};
use syn::parse::{Parse, ParseStream, Result};

// use syn::spanned::Spanned;

// Create a procedural attribute macro
//
// Notably, this must be placed alone in its own crate
// #[proc_macro_attribute]
// pub fn lua_raw_method(_args: TokenStream, input: TokenStream) -> TokenStream {
//     // // let output = TokenStream::clone(&item);
//     // // Parse the passed item as a function
//     let func = syn::parse_macro_input!(input as syn::ItemFn);

//     // Break the function down into its parts
//     let syn::ItemFn {
//         attrs,
//         vis,
//         sig,
//         block,
//     } = func;
   
//     let handler_name =  format_ident!("lua_c_{}", sig.ident);
//     let name =  format_ident!("{}", sig.ident);
//     let output = quote! {
//         #(#attrs)* #vis #sig #block
//         unsafe extern "C" fn #handler_name(state: *mut lua_sys::lua_State) -> lua_rs::prelude::c_int {
//             let state = lua_rs::LuaState::from_ctx(state); 
//             return (#name(state) as c_int);
//         }
//     };
//     // println!("{}", output);
//     // TokenStream::from(output)
//     return TokenStream::from(output);
// }


struct RawMethodExpr {
    name: Ident,
    closure: ExprClosure
}

impl Parse for RawMethodExpr {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![,]>()?;
        let closure: ExprClosure =  input.parse()?;
        Ok(RawMethodExpr{
            name: name,
            closure: closure
        })
    }
}


#[proc_macro]
pub fn lua_raw_method(item: TokenStream) -> TokenStream {
    println!("{}", item);
    let expr = syn::parse_macro_input!(item as RawMethodExpr);
    let RawMethodExpr {
        name, 
        closure
    } = expr;
    
    let output = quote! {
        unsafe extern "C" fn #name(state: *mut lua_sys::lua_State) -> lua_rs::prelude::c_int {
            let state = lua_rs::LuaState::from_ctx(state);
            let cl = (#closure); 
            return (cl(state) as c_int);
        }
    };
    println!("{}", output);
    return TokenStream::from(output);
}