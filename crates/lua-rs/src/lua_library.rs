use crate::lua_state::LuaState;
use std::ffi::CString;
// use std::ffi::CStr;

pub struct LuaLibrary<'a> {
    pub name: &'a str,
    pub func: ffi::lua_CFunction,
}

pub const BASE_LIBRARY: LuaLibrary = LuaLibrary {
    name: "_G", 
    func: Some(ffi::luaopen_base)
}; 
pub const PACKAGE_LIBRARY: LuaLibrary = LuaLibrary {
    name: "package", 
    func: Some(ffi::luaopen_package)
}; 
pub const COROUTINE_LIBRARY: LuaLibrary = LuaLibrary {
    name: "coroutine", 
    func: Some(ffi::luaopen_coroutine)
};
pub const TABLE_LIBRARY : LuaLibrary = LuaLibrary {
    name: "table", 
    func: Some(ffi::luaopen_table)
};
pub const IO_LIBRARY : LuaLibrary = LuaLibrary {
    name: "io", 
    func: Some(ffi::luaopen_io)
};
pub const OS_LIBRARY : LuaLibrary = LuaLibrary {
    name: "os", 
    func: Some(ffi::luaopen_os),
};
pub const STR_LIBRARY : LuaLibrary = LuaLibrary {
    name: "string", 
    func: Some(ffi::luaopen_string)
};
pub const MATH_LIBRARY : LuaLibrary = LuaLibrary {
    name: "math", 
    func: Some(ffi::luaopen_math)
};
pub const UTF8_LIBRARY : LuaLibrary = LuaLibrary {
    name: "utf8", 
    func: Some(ffi::luaopen_utf8)
};
pub const DEBUG_LIBRARY : LuaLibrary = LuaLibrary {
    name: "debug", 
    func: Some(ffi::luaopen_debug)
};

pub trait LuaStateLibrary {
    fn load_c_library(&mut self, reg: &LuaLibrary) -> Result<(), Box<dyn std::error::Error>>;
    // fn load_library(&mut self, reg: &LuaRegisteryLibrary) -> Result<(), Box<dyn std::error::Error>>;
    fn load_standard_libraries(&mut self) -> Result<(), Box<dyn std::error::Error>>  {
        self.load_c_library(&BASE_LIBRARY)?;
        self.load_c_library(&PACKAGE_LIBRARY)?;
        self.load_c_library(&COROUTINE_LIBRARY)?;
        self.load_c_library(&TABLE_LIBRARY)?;
        self.load_c_library(&IO_LIBRARY)?;
        self.load_c_library(&OS_LIBRARY)?;
        self.load_c_library(&STR_LIBRARY)?;
        self.load_c_library(&MATH_LIBRARY)?;
        self.load_c_library(&UTF8_LIBRARY)?;
        self.load_c_library(&DEBUG_LIBRARY)?;
        Ok(())
    }
}


impl LuaStateLibrary for LuaState  {
    fn load_c_library(&mut self, reg: &LuaLibrary) -> Result<(), Box<dyn std::error::Error>> {
        let name = CString::new(reg.name)?;
        unsafe {
            ffi::luaL_requiref(self.ctx(), name.as_ptr(), reg.func, 1);
            ffi::lua_pop(self.ctx(), 1);
        }
        Ok(())
    }

    // fn load_library(&mut self, reg: &LuaRegisteryLibrary) -> Result<(), Box<dyn std::error::Error>> {
    //     let mut method_names: Vec<CString> = Vec::new();
    //     let mut lua_register: Vec<ffi::luaL_Reg> = Vec::new();
    //     method_names.reserve(reg.register.len());
    //     for r in &reg.register {
    //         let name  = CString::new(r.name)?;
    //         lua_register.push(ffi::luaL_Reg {
    //             name: name.as_ptr(),
    //             func: r.func
    //         });
    //         method_names.push(name);
    //     }
    //     lua_register.push(ffi::luaL_Reg {
    //         name: std::ptr::null_mut(),
    //         func: None
    //     });
    //     unsafe { 
    //         ffi::lua_createtable(self.ctx(), 0, reg.register.len() as i32); 
    //         ffi::luaL_setfuncs(self.ctx(), lua_register.as_ptr(), 0 );
    //     }
    //     Ok(())
    // }
}