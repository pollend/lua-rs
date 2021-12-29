use crate::lua_state::LuaState;
use std::ffi::CString;

pub struct LuaRegister {

}

pub struct LuaLibrary<'a> {
    pub name: &'a str,
    pub func: ffi::lua_CFunction,
    pub register: Option<Vec<LuaRegister>>
}


pub const BASE_LIBRARY: LuaLibrary = LuaLibrary {
    name: "_G", 
    func: Some(ffi::luaopen_base),
    register: None
}; 
pub const PACKAGE_LIBRARY: LuaLibrary = LuaLibrary {
    name: "package", 
    func: Some(ffi::luaopen_package),
    register: None
}; 
pub const COROUTINE_LIBRARY: LuaLibrary = LuaLibrary {
    name: "coroutine", 
    func: Some(ffi::luaopen_coroutine),
    register: None
};
pub const TABLE_LIBRARY : LuaLibrary = LuaLibrary {
    name: "table", 
    func: Some(ffi::luaopen_table),
    register: None
};
pub const IO_LIBRARY : LuaLibrary = LuaLibrary {
    name: "io", 
    func: Some(ffi::luaopen_io),
    register: None
};
pub const OS_LIBRARY : LuaLibrary = LuaLibrary {
    name: "os", 
    func: Some(ffi::luaopen_os),
    register: None
};
pub const STR_LIBRARY : LuaLibrary = LuaLibrary {
    name: "string", 
    func: Some(ffi::luaopen_string),
    register: None
};
pub const MATH_LIBRARY : LuaLibrary = LuaLibrary {
    name: "math", 
    func: Some(ffi::luaopen_math),
    register: None
};
pub const UTF8_LIBRARY : LuaLibrary = LuaLibrary {
    name: "utf8", 
    func: Some(ffi::luaopen_utf8),
    register: None
};
pub const DEBUG_LIBRARY : LuaLibrary = LuaLibrary {
    name: "debug", 
    func: Some(ffi::luaopen_debug),
    register: None
};

pub trait LuaStateLibrary {
    fn load_library(&mut self, reg: &LuaLibrary) -> Result<(), Box<dyn std::error::Error>>;
    fn load_standard_libraries(&mut self) -> Result<(), Box<dyn std::error::Error>>  {
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


impl LuaStateLibrary for LuaState  {
    fn load_library(&mut self, reg: &LuaLibrary) -> Result<(), Box<dyn std::error::Error>> {
        let name = CString::new(reg.name)?;
        unsafe {
            ffi::luaL_requiref(self.ctx(), name.as_ptr(), reg.func, 1);
            ffi::lua_pop(self.ctx(), 1);
        }
        Ok(())
    }
}