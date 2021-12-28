extern crate lua_sys as ffi;
extern crate lua_macro;
// pub use lua_sys as ffi::lua

mod types;
mod lua_state;

pub use lua_state::{
    LuaState,
    LuaRegistry
};


pub use types::{
    LuaTypes,
    LUA_RIDX_MAINTHREAD,
    LUA_RIDX_GLOBALS,
    LUA_RIDX_LAST
};


pub mod prelude {
    pub use std::os::raw::c_int;
}