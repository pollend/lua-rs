use crate::lua_state::LuaState;
use crate::types::ByteUnit;

pub trait LuaStateGC {
    fn gc_stop(&mut self);
    fn gc_restart(&mut self);
    fn gc_collect(&mut self);
    fn gc_count(&mut self) -> ByteUnit;
    fn gc_step(&mut self);
    fn gc_pause(&mut self);
    fn gc_set_step_mul(&mut self);
    fn gc_is_running(&mut self) -> bool;
    fn gc_gen(&mut self);
    fn gc_inc(&mut self);
}

impl LuaStateGC for LuaState {
    fn gc_stop(&mut self) {
        unsafe {
            ffi::lua_gc(self.ctx(), ffi::LUA_GCSTOP as i32);
        }
    }

    fn gc_restart(&mut self) {
        unsafe {
            ffi::lua_gc(self.ctx(), ffi::LUA_GCRESTART as i32);
        }
    }

    fn gc_collect(&mut self) {
        unsafe {
            ffi::lua_gc(self.ctx(), ffi::LUA_GCCOLLECT as i32);
        }
    }

    fn gc_count(&mut self) -> ByteUnit {
        unsafe {
            return ffi::lua_gc(self.ctx(), ffi::LUA_GCCOUNTB as i32) as ByteUnit;
        }
    }

    fn gc_step(&mut self, steps: i32) {
        unsafe {
            ffi::lua_gc(self.ctx(), ffi::LUA_GCSTEP as i32, steps);
        }
    }

    fn gc_pause(&mut self) {
        unsafe {
            ffi::lua_gc(self.ctx(), ffi::LUA_GCSETPAUSE as i32, 0);
        }
    }

    fn gc_set_step_mul(&mut self) {
        unsafe {
            ffi::lua_gc(self.ctx(), ffi::LUA_GCSETSTEPMUL as i32, 0);
        }
    }

    fn gc_is_running(&mut self) -> bool {
        unsafe {
            return ffi::lua_gc(self.ctx(), ffi::LUA_GCISRUNNING as i32) > 0 ;
        }
    }

    fn gc_gen(&mut self) {
        unsafe {
            ffi::lua_gc(self.ctx(), ffi::LUA_GCGEN as i32, 0);
        }
    }

    fn gc_inc(&mut self) {
        unsafe {
            ffi::lua_gc(self.ctx(), ffi::LUA_GCINC as i32, 0);
        }
    }
}