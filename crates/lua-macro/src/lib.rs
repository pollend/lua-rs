// print_caller_location/src/lib.rs
use proc_macro::TokenStream;
use quote::{quote};
use syn::{Ident, ExprClosure, Token};
use syn::parse::{Parse, ParseStream, Result};


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
    return TokenStream::from(output);
}