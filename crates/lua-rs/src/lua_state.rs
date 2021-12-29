use std::ptr;

pub struct LuaState {
    main_state: *mut ffi::lua_State,
    owned: bool
}

trait MethodTrait: 

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

    pub fn new() -> LuaState {
        return LuaState{ main_state: ptr::null_mut() };
    }
} 
