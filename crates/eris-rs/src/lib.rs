extern crate lua_sys as lua;
extern crate libc;

// use lua_sys::{lua_State};
use std::os::raw::{c_int};

pub unsafe fn eris_persist(state: *mut lua::lua_State, perms: c_int, values: c_int) {
    let _perms = lua::lua_absindex(state, perms);
    let _value = lua::lua_absindex(state, values);

}

// pub fn eris_unpersist(state: mut* lua_State, perms: c_int, value: c_int) {
// }