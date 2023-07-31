
pub mod filesystem {
    use std::os::raw::{c_char, c_void, c_longlong};
    use libc::FILE;

    extern {
        pub fn _ftelli64(file: *mut FILE) -> c_longlong;
    }

    // rust fs does not seem to have correct file size with emscripten so add method to call cstdio functions directly
    pub fn read_text_file_immediate(filename: &str) -> std::io::Result<String> {
        let filename_str = std::ffi::CString::new(filename).unwrap();
        let filename_ptr: *const c_char = filename_str.as_ptr() as *const c_char;

        let args_str = std::ffi::CString::new("rb").unwrap();
        let args_ptr: *const c_char = args_str.as_ptr() as *const c_char;

        let file = unsafe { libc::fopen(filename_ptr, args_ptr) };
        
        if file.is_null() {
            let err_msg = format!("Unable to open file '{}'.", filename);
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, err_msg))
        }

        unsafe { libc::fseek(file, 0, libc::SEEK_END) };

        // ftell64 is unix only and _ftelli64 is windows only
        #[cfg(not(windows))]
        let size = unsafe { libc::ftello64(file) };
        #[cfg(windows)]
        let size = unsafe { _ftelli64(file) };

        unsafe { libc::rewind(file) };

        if size == 0 {
            unsafe { libc::fclose(file) };
            let err_msg = format!("File '{}' has a size of zero.", filename);
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, err_msg))
        }

        //read buffer with extra byte for null character
        let mut buffer: Vec<c_char> = vec![0; 1 + size as usize];
        let ptr = buffer.as_mut_ptr();

        let read_size = unsafe { libc::fread(ptr as *mut c_void, 1, size as usize, file) };

        if read_size != size as usize{
            unsafe { libc::fclose(file) };
            let err_msg = format!("Read size for '{}' does not equal file size.", filename);
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, err_msg))
        }

        std::mem::forget(buffer);

        let string = unsafe {
            let c_str = std::ffi::CStr::from_ptr(ptr);
            c_str.to_string_lossy().into_owned()
        };

        println!("Len {}, Size {}", size, string.len());

        unsafe { libc::fclose(file) };

        return Ok(string);
    }
}
