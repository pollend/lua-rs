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

// #[proc_macro]
// pub fn lua_library(item: TokenStream) -> TokenStream {

// }


#[proc_macro]
pub fn lua_raw_method(item: TokenStream) -> TokenStream {
    let expr = syn::parse_macro_input!(item as RawMethodExpr);
    let RawMethodExpr {
        name, 
        closure
    } = expr;
    
    let output = quote! {
        extern "C" fn #name(l: *mut lua_sys::lua_State) -> lua_rs::prelude::c_int {
            // use a wrapper method to ensure the stack is clear
            fn __wrapper(l: *mut lua_sys::lua_State, cl: fn(rs: lua_rs::LuaState) -> 
                Result<i32, Box<dyn lua_rs::prelude::Error>>) -> Option<lua_rs::prelude::c_int> {
                let err = cl(lua_rs::LuaState::from_ctx(l));
                return match err {
                    Ok(res) => { Some(res) }
                    Err(e) => {
                        match lua_rs::prelude::CString::new(e.to_string()) {
                            Ok(error_str) => {
                                unsafe {
                                    ffi::lua_pushliteral(l, error_str.as_ptr());
                                }
                            }
                            Err(e) => {
                                unsafe {
                                    let raw = b"Unknown Error".to_vec();
                                    let unkonwn_lit = CString::from_vec_unchecked(raw);
                                    ffi::lua_pushliteral(l, unkonwn_lit.as_ptr());
                                }
                            }
                        }
                        None
                    }
                };
            }
            let result = __wrapper(l, #closure);
            if result.is_none() {
                unsafe { lua_rs::prelude::ffi::lua_error(l); }
            }
            return result.unwrap();
        }
    };
    return TokenStream::from(output);
}