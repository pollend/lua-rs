use std::ptr;

use crate::lua_library::LuaStateLibrary;
use crate::lua_gc::LuaStateGC;
use std::ffi::CString;



pub struct LuaState {
    main_state: *mut ffi::lua_State,
    owned: bool
}


impl LuaState {
    pub unsafe fn ctx(&self) -> *mut ffi::lua_State {
        return self.main_state;
    }

    pub fn from_ctx(state: *mut ffi::lua_State) -> LuaState {
        return LuaState {
            main_state: state,
            owned: false
        };
    }
} 

pub trait MainLuaState {
    fn new() -> LuaState;
    fn load_string(&mut self, value: &str) -> Result<(), Box<dyn std::error::Error>>;
}

impl MainLuaState for LuaState {
    fn new() -> LuaState {
        unsafe {
            return LuaState{ 
                main_state: ffi::bindings::luaL_newstate(),
                owned: true
            };
        }
    }

    fn load_string(&mut self, value: &str) -> Result<(), Box<dyn std::error::Error>> {
        let buf =  CString::new(value)?;
        unsafe { ffi::luaL_loadstring(self.ctx(), buf.as_ptr()) };
        Ok(())
    }
}

impl Drop for LuaState {
    fn drop(&mut self) {
        if self.owned {
            unsafe { ffi::lua_close(self.main_state); }
        }
    }
}
