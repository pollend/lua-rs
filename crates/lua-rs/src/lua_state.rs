use std::ptr;
// use std::os::raw::{c_int};

pub struct LuaState {
    main_state: *mut ffi::lua_State
}

type luaCFunction = fn(ctx: LuaState) -> i32;

pub struct LuaRegistry {
    
    #[allow(dead_code)]
    pub key: &'static str,

    #[allow(dead_code)]
    pub func: ffi::lua_CFunction,
}


impl LuaRegistry {
    pub fn new(key: &'static str, func: ffi::lua_CFunction) -> LuaRegistry{
        return LuaRegistry {
            key: key,
            func: func
        }
        
    }
}


impl LuaState {
    pub unsafe fn ctx(&self) -> *mut ffi::lua_State {
        return self.main_state;
    }

    pub fn from_ctx(s: *mut ffi::lua_State) -> LuaState {
        return LuaState {
            main_state: s
        };
    }

    pub fn new() -> LuaState{
        return LuaState{ main_state: ptr::null_mut() };
    }

    pub fn gc_stop(&mut self) {
        unsafe {
            ffi::lua_gc(self.ctx(), ffi::LUA_GCSTOP as i32, 0);
        }
    }


    pub fn load_library(&mut self, libraries: &[LuaRegistry]) {
        unsafe {
            ffi::lua_createtable(self.ctx(), 0, libraries.len() as i32);

        }
    }

    // pub fn open_base(&mut self) {
    //     unsafe { ffi::luaopen_base(self.ctx()); }
    // }
    // pub fn open_coroutine(&mut self) {
    //     unsafe { ffi::luaopen_coroutine(self.ctx()); }
    // }
    // pub fn open_table(&mut self) {
    //     unsafe { ffi::luaopen_table(self.ctx()); }
    // }
    // pub fn open_io(&mut self) {
    //     unsafe { ffi::luaopen_io(self.ctx()); }
    // }
    // pub fn open_os(&mut self) {
    //     unsafe { ffi::luaopen_os(self.ctx()); }
    // }
    // pub fn open_string(&mut self) {
    //     unsafe { ffi::luaopen_string(self.ctx()); }
    // }
    // pub fn open_utf8(&mut self) {
    //     unsafe { ffi::luaopen_utf8(self.ctx()); }
    // }
    // pub fn open_math(&mut self) {
    //     unsafe { ffi::luaopen_math(self.ctx()); }
    // }
    // pub fn open_debug(&mut self) {
    //     unsafe { ffi::luaopen_debug(self.ctx()); }
    // }
    // pub fn open_package(&mut self) {
    //     unsafe { ffi::luaopen_package(self.ctx()); }
    // }
} 
