// Copyright 2021 The Terasology Foundation
// SPDX-License-Identifier: Apache-2.0

#![allow(non_camel_case_types)]

pub extern crate libc;

use std::os::raw::{c_char, c_int};
use std::ptr;

pub mod bindings {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub const LUA_RIDX_GLOBALS: bindings::lua_Integer = bindings::LUA_RIDX_GLOBALS as bindings::lua_Integer;
pub const LUA_REGISTRYINDEX: c_int = bindings::LUA_REGISTRYINDEX;

pub use {
    bindings::LUA_VERSION_MAJOR,
    bindings::LUA_VERSION_MINOR,
    bindings::LUA_VERSION_RELEASE,
    bindings::LUA_VERSION_NUM,
    bindings::LUA_VERSION_RELEASE_NUM,
    bindings::LUA_VERSION,
    bindings::LUA_RELEASE,
    bindings::LUA_COPYRIGHT,
    bindings::LUA_AUTHORS,
    bindings::LUA_SIGNATURE
};


pub use {
    bindings::lua_Alloc, bindings::lua_CFunction, bindings::lua_Integer, bindings::lua_KContext,
    bindings::lua_Number, bindings::lua_State, bindings::lua_Unsigned, bindings::size_t,
    bindings::size_t as wchar_t,
};

/*
** state manipulation
*/
pub use {
    bindings::lua_atpanic, bindings::lua_close, bindings::lua_newstate, bindings::lua_newthread,
    bindings::lua_resetthread, bindings::lua_version,
};

/*
** basic stack manipulation
*/
pub use {
    bindings::lua_absindex, bindings::lua_checkstack, bindings::lua_copy, bindings::lua_gettop,
    bindings::lua_pushvalue, bindings::lua_rotate, bindings::lua_settop, bindings::lua_xmove,
};

/*
** access functions (stack -> C)
*/
pub use {
    bindings::lua_iscfunction, bindings::lua_isinteger, bindings::lua_isnumber,
    bindings::lua_isstring, bindings::lua_isuserdata, bindings::lua_rawlen,
    bindings::lua_toboolean, bindings::lua_tocfunction, bindings::lua_tointegerx,
    bindings::lua_tolstring, bindings::lua_tonumberx, bindings::lua_topointer,
    bindings::lua_tothread, bindings::lua_touserdata, bindings::lua_typename,
    bindings::lua_type,
};

/*
** push functions (C -> stack)
*/
pub use {
    bindings::lua_pushboolean, bindings::lua_pushcclosure, bindings::lua_pushfstring,
    bindings::lua_pushinteger, bindings::lua_pushlightuserdata, bindings::lua_pushlstring,
    bindings::lua_pushnil, bindings::lua_pushnumber, bindings::lua_pushstring,
    bindings::lua_pushthread, bindings::lua_pushvfstring,
};

/*
** get functions (Lua -> stack)
*/
pub use {
    bindings::lua_createtable, bindings::lua_getfield, bindings::lua_getglobal, bindings::lua_geti,
    bindings::lua_getiuservalue, bindings::lua_getmetatable, bindings::lua_gettable,
    bindings::lua_newuserdatauv, bindings::lua_rawget, bindings::lua_rawgeti,
    bindings::lua_rawgetp,
};

/*
** set functions (stack -> Lua)
*/
pub use {
    bindings::lua_rawset, bindings::lua_rawseti, bindings::lua_rawsetp, bindings::lua_setfield,
    bindings::lua_setglobal, bindings::lua_seti, bindings::lua_setiuservalue,
    bindings::lua_setmetatable, bindings::lua_settable,
};

/*
** 'load' and 'call' functions (load and run Lua code)
*/
pub use {bindings::lua_callk, bindings::lua_dump, bindings::lua_load, bindings::lua_pcallk};
pub unsafe fn lua_pcall(
    state: *mut lua_State,
    nargs: c_int,
    nresults: c_int,
    errfunc: c_int,
) -> c_int {
    return lua_pcallk(state, nargs, nresults, errfunc, 0, None);
}
pub unsafe  fn lua_call(state: *mut lua_State, nargs: c_int, nresults: c_int) {
    lua_callk(state, nargs, nresults, 0, None);
}

/*
** coroutine functions
*/
pub use {
    bindings::lua_isyieldable, bindings::lua_resume, bindings::lua_status, bindings::lua_yieldk,
};

pub unsafe fn lua_yield(state: *mut lua_State, n: c_int) -> c_int {
    return lua_yieldk(state, n, 0, None);
}

/*
** Warning-related functions
*/
pub use {bindings::lua_setwarnf, bindings::lua_warning};

/*
** garbage-collection function and options
*/
pub use {
    bindings::lua_gc
};

/*
** miscellaneous functions
*/
pub use {
    bindings::lua_closeslot, bindings::lua_concat, bindings::lua_error, bindings::lua_getallocf,
    bindings::lua_len, bindings::lua_next, bindings::lua_setallocf, bindings::lua_stringtonumber,
    bindings::lua_toclose,
};

/*
** {==============================================================
** some useful macros
** ===============================================================
*/
pub unsafe fn lua_tonumber(state: *mut lua_State, idx: c_int) -> bindings::lua_Number {
    return lua_tonumberx(state, idx, ptr::null_mut());
}
pub unsafe fn lua_tointeger(state: *mut lua_State, idx: c_int) -> bindings::lua_Number {
    return lua_tonumberx(state, idx, ptr::null_mut());
}
pub unsafe fn lua_pop(state: *mut lua_State, n: c_int) {
    lua_settop(state, -(n) - 1);
}
pub unsafe fn lua_isfunction(state: *mut lua_State, n: c_int) -> bool {
    return lua_type(state, n) == bindings::LUA_TFUNCTION as i32;
}
pub unsafe fn lua_istable(state: *mut lua_State, n: c_int) -> bool {
    return lua_type(state, n) == bindings::LUA_TTABLE as i32;
}
pub unsafe fn lua_islightuserdata(state: *mut lua_State, n: c_int) -> bool {
    return lua_type(state, n) == bindings::LUA_TLIGHTUSERDATA as i32;
}
pub unsafe fn lua_isnil(state: *mut lua_State, n: c_int) -> bool {
    return lua_type(state, n) == bindings::LUA_TNIL as i32;
}
pub unsafe fn lua_isboolean(state: *mut lua_State, n: c_int) -> bool {
    return lua_type(state, n) == bindings::LUA_TBOOLEAN as i32;
}
pub unsafe fn lua_isthread(state: *mut lua_State, n: c_int) -> bool {
    return lua_type(state, n) == bindings::LUA_TTHREAD as i32;
}
pub unsafe fn lua_isnone(state: *mut lua_State, n: c_int) -> bool {
    return lua_type(state, n) == bindings::LUA_TNONE  as i32;
}
pub unsafe fn lua_isnoneornil(state: *mut lua_State, n: c_int) -> bool {
    return lua_type(state, n) == bindings::LUA_TNONE  as i32; 
}

pub unsafe fn lua_pushliteral(state: *mut lua_State, str: *const c_char) -> *const c_char {
    return lua_pushstring(state, str);
}

pub unsafe fn lua_pushglobaltable(state: *mut lua_State) {
    lua_rawgeti(state, LUA_REGISTRYINDEX, LUA_RIDX_GLOBALS);
}

pub unsafe fn lua_newtable(state: *mut lua_State) {
    lua_createtable(state, 0, 0);
}

pub unsafe fn lua_register(state: *mut lua_State, n: *const c_char, f: lua_CFunction) {
    lua_pushcfunction(state, f);
    lua_setglobal(state, n);
}

pub unsafe fn lua_pushcfunction(state: *mut lua_State, f: lua_CFunction) {
    lua_pushcclosure(state, f, 0);
}

pub unsafe fn lua_tostring(state: *mut lua_State, i: c_int) -> *const c_char {
    return lua_tolstring(state, i, ptr::null_mut());
}

pub unsafe fn lua_insert(state: *mut lua_State, idx: c_int) {
    lua_rotate(state, idx, 1);
}

pub unsafe fn lua_remove(state: *mut lua_State, idx: c_int) {
    lua_rotate(state, idx, -1);
    lua_pop(state, 1);
}

pub unsafe fn lua_replace(state: *mut lua_State, idx: c_int) {
    lua_copy(state, -1, idx);
    lua_pop(state, 1);
}
