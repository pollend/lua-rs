
use lua_rs::{LuaState, LuaRegistry, prelude::*};
use lua_macro::{lua_raw_method};
        

lua_raw_method!(handler, |ctx| { 
    return 0;
});

#[test] 
fn test_registery() {
    let _h = LuaRegistry::new("test", Some(handler));
}