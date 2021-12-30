use crate::lua_state::LuaState;
use std::os::raw::{c_char, c_int};
use std::ffi::CString;

pub trait LuaStackPush {
    fn pushboolean(&mut self, value: bool);
    fn pushclosure(&mut self, handler: ffi::lua_CFunction, nargs: i32);

    fn pushnil(&mut self);
    fn pushnumber(&mut self, value: f64);
    fn pushinteger(&mut self, value: i64);
    fn pushstring(&mut self, value: &str);
}

pub trait LuaStackGet {
    // pub fn pushboolean(&mut self, bool value);
}


impl LuaStackPush for LuaState {
    fn pushboolean(&mut self, value: bool) {
        unsafe {
            ffi::lua_pushboolean(self.ctx(), value as c_int);
        }
    }    

    fn pushclosure(&mut self, handler: ffi::lua_CFunction, nargs: i32) {
        unsafe {
            ffi::lua_pushcclosure(self.ctx(), handler, nargs);
        }
    }

    fn pushnil(&mut self) {
        unsafe {
            ffi::lua_pushnil(self.ctx());
        }
    }
    fn pushnumber(&mut self, value: f64) {
        unsafe {
            ffi::lua_pushnumber(self.ctx(), value);
        }
    }
    fn pushinteger(&mut self, value: i64) {
        unsafe {
            ffi::lua_pushinteger(self.ctx(), value);
        }
    }
    fn pushstring(&mut self, value: &str) {
        unsafe {
            // ffi::lua_pushlstring(self.ctx(), value.as_ptr(), (value.len() as i64).try_into().unwrap());
        }
    }
}
