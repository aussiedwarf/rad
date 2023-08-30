use std::ffi::{CString};
use libc::{c_char};

// Get the pointers to the validation layers names.
// Also return the corresponding `CString` to avoid dangling pointers.
pub fn get_names_and_pointers(a_names: &std::vec::Vec<&str>) -> (Vec<CString>, Vec<*const c_char>) {
  let names = a_names
    .iter()
    .map(|name| CString::new(*name).unwrap())
    .collect::<Vec<_>>();
  let names_ptrs = names
    .iter()
    .map(|name| name.as_ptr())
    .collect::<Vec<_>>();
  (names, names_ptrs)
}
