extern crate lua_rs;


unsafe extern fn alloc_func(_: *mut c_void, ptr: *mut c_void, old_size: size_t, new_size: size_t) -> *mut c_void {
    // In GCC and MSVC, malloc uses an alignment calculated roughly by:
    //   max(2 * sizeof(size_t), alignof(long double))
    // The stable high-level API used here does not expose alignment directly, so
    // we get as close as possible by using usize to determine alignment. Lua
    // seems unlikely to require 16-byte alignment for any of its purposes.
  
    #[inline]
    fn divide_size(size: size_t) -> usize {
      1 + (size - 1) / mem::size_of::<usize>()
    }
  
    let ptr = ptr as *mut usize;
    if new_size == 0 {
      // if new_size is 0, act like free()
      if !ptr.is_null() {
        // Lua promises to provide the correct old_size
        drop(Vec::<usize>::from_raw_parts(ptr, 0, divide_size(old_size)));
      }
      ptr::null_mut()
    } else {
      // otherwise, act like realloc()
      let mut vec;
      if ptr.is_null() {
        // old_size is a type indicator, not used here
        vec = Vec::<usize>::with_capacity(divide_size(new_size));
      } else {
        // Lua promises to provide the correct old_size
        if new_size > old_size {
          // resulting capacity should be new_size
          vec = Vec::<usize>::from_raw_parts(ptr, 0, divide_size(old_size));
          vec.reserve_exact(divide_size(new_size));
        } else {
          // Lua assumes this will never fail
          vec = Vec::<usize>::from_raw_parts(ptr, divide_size(new_size), divide_size(old_size));
          vec.shrink_to_fit();
        }
      }
      let res = vec.as_mut_ptr();
      mem::forget(vec); // don't deallocate
      res as *mut c_void
    }
  }

#[test]
fn test_configurating() {
    lua_newstate(Some(alloc_func), ptr::null_mut())
}