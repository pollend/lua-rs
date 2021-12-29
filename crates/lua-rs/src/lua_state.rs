use std::ptr;

use crate::lua_library::LuaStateLibrary;
use crate::lua_gc::LuaStateGC;


pub struct LuaState {
    main_state: *mut ffi::lua_State,
    owned: bool
}

// trait LuaMethodTrait: LuaStateGC {}
// trait LuaMainTrait: LuaStateGC +  LuaStateLibrary {}
// pub fn test(t :&(impl LuaStateGC +  LuaStateLibrary)) {
// }
// pub fn a() {
//     let mut t = LuaState::new();
//     test(&t);
// }


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
        return LuaState{ 
            main_state: ptr::null_mut(),
            owned: true
        };
    }
} 
