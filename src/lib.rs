// Copyright 2021 The Terasology Foundation
// SPDX-License-Identifier: Apache-2.0

#![allow(non_camel_case_types)]

pub extern crate libc;

use std::os::raw::{c_char, c_int};
use std::{ptr, str};

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

#[repr(i32)]
#[derive(PartialEq)]
pub enum LUA_BASE_TYPE {
    LUA_TNONE = bindings::LUA_TNONE as i32,
    LUA_TNIL = bindings::LUA_TNIL as i32,
    LUA_TBOOLEAN = bindings::LUA_TBOOLEAN as i32,
    LUA_TLIGHTUSERDATA = bindings::LUA_TLIGHTUSERDATA as i32,
    LUA_TNUMBER = bindings::LUA_TNUMBER as i32,
    LUA_TSTRING = bindings::LUA_TSTRING as i32,
    LUA_TTABLE = bindings::LUA_TTABLE as i32,
    LUA_TFUNCTION = bindings::LUA_TFUNCTION as i32,
    LUA_TUSERDATA = bindings::LUA_TUSERDATA as i32,
    LUA_TTHREAD = bindings::LUA_TTHREAD as i32,
    LUA_NUMTYPES = bindings::LUA_NUMTYPES as i32,
}

impl LUA_BASE_TYPE {
    pub fn to_typename(&self) -> Option<&'static str> {
        match self {
            LUA_BASE_TYPE::LUA_TNIL => return Some("no value"),
            LUA_BASE_TYPE::LUA_TBOOLEAN => return Some("boolean"),
            LUA_BASE_TYPE::LUA_TLIGHTUSERDATA => return Some("userdata"),
            LUA_BASE_TYPE::LUA_TNUMBER => return Some("number"),
            LUA_BASE_TYPE::LUA_TSTRING => return Some("string"),
            LUA_BASE_TYPE::LUA_TTABLE => return Some("table"),
            LUA_BASE_TYPE::LUA_TFUNCTION => return Some("function"),
            LUA_BASE_TYPE::LUA_TUSERDATA => return Some("userdata"),
            LUA_BASE_TYPE::LUA_TTHREAD => return Some("thread"),
            _ => return None,
        };
    }
}

/*
** Comparison and arithmetic functions
*/
#[repr(i32)]
#[derive(PartialEq)]
pub enum LUA_ARITH_OP {
    LUA_OPADD = bindings::LUA_OPADD as i32,
    LUA_OPSUB = bindings::LUA_OPSUB as i32,
    LUA_OPMUL = bindings::LUA_OPMUL as i32,
    LUA_OPMOD = bindings::LUA_OPMOD as i32,
    LUA_OPPOW = bindings::LUA_OPPOW as i32,
    LUA_OPDIV = bindings::LUA_OPDIV as i32,
    LUA_OPIDIV = bindings::LUA_OPIDIV as i32,
    LUA_OPBAND = bindings::LUA_OPBAND as i32,
    LUA_OPBOR = bindings::LUA_OPBOR as i32,
    LUA_OPBXOR = bindings::LUA_OPBXOR as i32,
    LUA_OPSHL = bindings::LUA_OPSHL as i32,
    LUA_OPSHR = bindings::LUA_OPSHR as i32,
    LUA_OPUNM = bindings::LUA_OPUNM as i32,
    LUA_OPBNOT = bindings::LUA_OPBNOT as i32,
}

#[repr(i32)]
#[derive(PartialEq)]
pub enum LUA_GC {
    LUA_GCSTOP = bindings::LUA_GCSTOP as i32,
    LUA_GCRESTART = bindings::LUA_GCRESTART as i32,
    LUA_GCCOLLECT = bindings::LUA_GCCOLLECT as i32,
    LUA_GCCOUNT = bindings::LUA_GCCOUNT as i32,
    LUA_GCCOUNTB = bindings::LUA_GCCOUNTB as i32,
    LUA_GCSTEP = bindings::LUA_GCSTEP as i32,
    LUA_GCSETPAUSE = bindings::LUA_GCSETPAUSE as i32,
    LUA_GCSETSTEPMUL = bindings::LUA_GCSETSTEPMUL as i32,
    LUA_GCISRUNNING = bindings::LUA_GCISRUNNING as i32,
    LUA_GCGEN = bindings::LUA_GCGEN as i32,
    LUA_GCINC = bindings::LUA_GCINC as i32,
}

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
};

pub unsafe fn lua_type(state: *mut lua_State, idx: c_int) -> LUA_BASE_TYPE {
    match bindings::lua_type(state, idx) {
        x if x == bindings::LUA_TNIL as i32 => LUA_BASE_TYPE::LUA_TNIL,
        x if x == bindings::LUA_TBOOLEAN as i32 => LUA_BASE_TYPE::LUA_TBOOLEAN,
        x if x == bindings::LUA_TLIGHTUSERDATA as i32 => LUA_BASE_TYPE::LUA_TLIGHTUSERDATA,
        x if x == bindings::LUA_TNUMBER as i32 => LUA_BASE_TYPE::LUA_TNUMBER,
        x if x == bindings::LUA_TSTRING as i32 => LUA_BASE_TYPE::LUA_TSTRING,
        x if x == bindings::LUA_TTABLE as i32 => LUA_BASE_TYPE::LUA_TTABLE,
        x if x == bindings::LUA_TFUNCTION as i32 => LUA_BASE_TYPE::LUA_TFUNCTION,
        x if x == bindings::LUA_TUSERDATA as i32 => LUA_BASE_TYPE::LUA_TUSERDATA,
        x if x == bindings::LUA_TTHREAD as i32 => LUA_BASE_TYPE::LUA_TTHREAD,
        _ => LUA_BASE_TYPE::LUA_TNONE,
    }
}

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
pub unsafe fn lua_gc(state: *mut lua_State, what: LUA_GC) -> c_int {
    return bindings::lua_gc(state, what as i32);
}

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
    return lua_type(state, n) == LUA_BASE_TYPE::LUA_TFUNCTION;
}
pub unsafe fn lua_istable(state: *mut lua_State, n: c_int) -> bool {
    return lua_type(state, n) == LUA_BASE_TYPE::LUA_TTABLE;
}
pub unsafe fn lua_islightuserdata(state: *mut lua_State, n: c_int) -> bool {
    return lua_type(state, n) == LUA_BASE_TYPE::LUA_TLIGHTUSERDATA;
}
pub unsafe fn lua_isnil(state: *mut lua_State, n: c_int) -> bool {
    return lua_type(state, n) == LUA_BASE_TYPE::LUA_TNIL;
}
pub unsafe fn lua_isboolean(state: *mut lua_State, n: c_int) -> bool {
    return lua_type(state, n) == LUA_BASE_TYPE::LUA_TBOOLEAN;
}
pub unsafe fn lua_isthread(state: *mut lua_State, n: c_int) -> bool {
    return lua_type(state, n) == LUA_BASE_TYPE::LUA_TTHREAD;
}
pub unsafe fn lua_isnone(state: *mut lua_State, n: c_int) -> bool {
    return lua_type(state, n) == LUA_BASE_TYPE::LUA_TNONE;
}
pub unsafe fn lua_isnoneornil(state: *mut lua_State, n: c_int) -> bool {
    return lua_type(state, n) == LUA_BASE_TYPE::LUA_TNONE;
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
