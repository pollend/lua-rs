extern crate lua_sys as ffi;
extern crate lua_macro;

// pub use lua_sys as ffi::lua

mod types;
mod lua_state;
mod lua_gc;
mod lua_library;
mod lua_stack;

pub use lua_state::{
   LuaState
};

pub use lua_library::{
    LuaLibrary,
    // LuaRegister,
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
    LUA_RIDX_LAST,
    ByteUnit
};


pub mod prelude {
    pub use std::os::raw::c_int;
    pub use lua_sys as ffi;
    pub use std::ffi::CString;
    pub use std::error::Error;
    pub use std::boxed::Box;
}