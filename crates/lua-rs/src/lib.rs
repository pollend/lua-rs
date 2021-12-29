extern crate lua_sys as ffi;
extern crate lua_macro;
extern crate byte_unit;

// pub use lua_sys as ffi::lua

mod types;
mod lua_state;

pub use lua_state::{
    LuaState,
    LuaLibrary,
    BASE_LIBRARY,
    PACKAGE_LIBRARY,
    COROUTINE_LIBRARY,
    TABLE_LIBRARY,
    IO_LIBRARY,
    OS_LIBRARY,
    STR_LIBRARY,
    MATH_LIBRARY,
    UTF8_LIBRARY,
    DEBUG_LIBRARY
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