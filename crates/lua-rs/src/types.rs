// use bitflags::bitflags;

#[repr(i32)]
#[derive(PartialEq)]
pub enum LuaTypes {
    Nil = ffi::LUA_TNIL as i32,
    Boolean = ffi::LUA_TBOOLEAN as i32,
    LightUserData = ffi::LUA_TLIGHTUSERDATA as i32,
    Number = ffi::LUA_TNUMBER as i32,
    String = ffi::LUA_TSTRING as i32,
    Table = ffi::LUA_TTABLE as i32,
    Function = ffi::LUA_TFUNCTION as i32,
    UserData = ffi::LUA_TUSERDATA as i32,
    Thread = ffi::LUA_TTHREAD as i32,
}


pub const LUA_RIDX_MAINTHREAD: u32 = ffi::LUA_RIDX_MAINTHREAD;
pub const LUA_RIDX_GLOBALS: u32 = ffi::LUA_RIDX_GLOBALS;
pub const LUA_RIDX_LAST: u32 = ffi::LUA_RIDX_LAST;