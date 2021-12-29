use std::ptr;
use std::ffi::CString;
use byte_unit::{Byte, ByteUnit, ByteError};

// type luaCFunction = fn(ctx: LuaState) -> i32;

pub struct LuaLibrary<'a> {
    pub name: &'a str,
    pub func: ffi::lua_CFunction,
}

pub static BASE_LIBRARY: LuaLibrary = LuaLibrary {
    name: "_G", 
    func: Some(ffi::luaopen_base)
}; 
pub static PACKAGE_LIBRARY: LuaLibrary = LuaLibrary {
    name: "package", 
    func: Some(ffi::luaopen_package)
}; 
pub static COROUTINE_LIBRARY: LuaLibrary = LuaLibrary {
    name: "coroutine", 
    func: Some(ffi::luaopen_coroutine)
};
pub static TABLE_LIBRARY : LuaLibrary = LuaLibrary {
    name: "table", 
    func: Some(ffi::luaopen_table)
};
pub static IO_LIBRARY : LuaLibrary = LuaLibrary {
    name: "io", 
    func: Some(ffi::luaopen_io)
};
pub static OS_LIBRARY : LuaLibrary = LuaLibrary {
    name: "os", 
    func: Some(ffi::luaopen_os)
};
pub static STR_LIBRARY : LuaLibrary = LuaLibrary {
    name: "string", 
    func: Some(ffi::luaopen_string)
};
pub static MATH_LIBRARY : LuaLibrary = LuaLibrary {
    name: "math", 
    func: Some(ffi::luaopen_math)
};
pub static UTF8_LIBRARY : LuaLibrary = LuaLibrary {
    name: "utf8", 
    func: Some(ffi::luaopen_utf8)
};
pub static DEBUG_LIBRARY : LuaLibrary = LuaLibrary {
    name: "debug", 
    func: Some(ffi::luaopen_debug)
};

pub struct LuaState {
    main_state: *mut ffi::lua_State
}

trait MainLuaState {

}

// trait LuaState {
// }

trait LuaGC {

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
            ffi::lua_gc(self.ctx(), ffi::LUA_GCSTOP as i32);
        }
    }

    pub fn gc_restart(&mut self) {
        unsafe {
            ffi::lua_gc(self.ctx(), ffi::LUA_GCRESTART as i32);
        }
    }

    pub fn gc_collect(&mut self) {
        unsafe {
            ffi::lua_gc(self.ctx(), ffi::LUA_GCCOLLECT as i32);
        }
    }


    pub fn gc_count(&mut self) -> Result<Byte, ByteError> {
        unsafe {
            return Byte::from_unit(ffi::lua_gc(self.ctx(), ffi::LUA_GCCOUNTB as i32), ByteUnit::B)?;
        }
    }

    pub fn gc_step(&mut self) {
        unsafe {
            ffi::lua_gc(self.ctx(), ffi::LUA_GCSTEP as i32, 0);
        }
    }

    pub fn gc_pause(&mut self) {
        unsafe {
            ffi::lua_gc(self.ctx(), ffi::LUA_GCSETPAUSE as i32, 0);
        }
    }

    pub fn gc_set_step_mul(&mut self) {
        unsafe {
            ffi::lua_gc(self.ctx(), ffi::LUA_GCSETSTEPMUL as i32, 0);
        }
    }

    pub fn gc_is_running(&mut self) -> bool {
        unsafe {
            return ffi::lua_gc(self.ctx(), ffi::LUA_GCISRUNNING as i32) > 0 ;
        }
    }

    pub fn gc_gen(&mut self) {
        unsafe {
            ffi::lua_gc(self.ctx(), ffi::LUA_GCGEN as i32, 0);
        }
    }

    pub fn gc_inc(&mut self) {
        unsafe {
            ffi::lua_gc(self.ctx(), ffi::LUA_GCINC as i32, 0);
        }
    }

    pub fn load_library(&mut self, reg: &LuaLibrary) -> Result<(), Box<dyn std::error::Error>> {
        let name = CString::new(reg.name)?;
        unsafe {
            ffi::luaL_requiref(self.ctx(), name.as_ptr(), reg.func, 1);
            ffi::lua_pop(self.ctx(), 1);
        }
        Ok(())
    }

    pub fn load_standard_libraries(&mut self) -> Result<(), Box<dyn std::error::Error>>  {
        self.load_library(&BASE_LIBRARY)?;
        self.load_library(&PACKAGE_LIBRARY)?;
        self.load_library(&COROUTINE_LIBRARY)?;
        self.load_library(&TABLE_LIBRARY)?;
        self.load_library(&IO_LIBRARY)?;
        self.load_library(&OS_LIBRARY)?;
        self.load_library(&STR_LIBRARY)?;
        self.load_library(&MATH_LIBRARY)?;
        self.load_library(&UTF8_LIBRARY)?;
        self.load_library(&DEBUG_LIBRARY)?;
        Ok(())
    }

} 
