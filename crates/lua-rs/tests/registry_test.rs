
use lua_rs::{LuaState, prelude::*};
use lua_macro::{lua_raw_method};
        

lua_raw_method!(handler, |ctx|  { 
    return Ok(45);
});


#[test] 
fn test_registery() {
    // let _h = LuaRegistry::new("test", Some(handler));
}