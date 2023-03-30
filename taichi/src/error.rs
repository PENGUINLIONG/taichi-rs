use std::ffi::c_char;

use taichi_sys::{TiError, ti_get_last_error, ti_set_last_error};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaichiError {
    code: TiError,
    message: String,
}
impl TaichiError {
    pub fn new(code: TiError, message: String) -> Self {
        Self { code, message }
    }

    pub fn code(&self) -> TiError {
        self.code
    }
    pub fn message(&self) -> &str {
        self.message.as_str()
    }

    #[allow(non_snake_case)]
    pub fn Success() -> Self {
        Self::new(TiError::Success, Default::default())
    }
    #[allow(non_snake_case)]
    pub fn NotSupported<S: ToString>(message: S) -> Self {
        Self::new(TiError::NotSupported, message.to_string())
    }
    #[allow(non_snake_case)]
    pub fn CorruptedData<S: ToString>(message: S) -> Self {
        Self::new(TiError::CorruptedData, message.to_string())
    }
    #[allow(non_snake_case)]
    pub fn NameNotFound<S: ToString>(message: S) -> Self {
        Self::new(TiError::NameNotFound, message.to_string())
    }
    #[allow(non_snake_case)]
    pub fn InvalidArgument<S: ToString>(message: S) -> Self {
        Self::new(TiError::InvalidArgument, message.to_string())
    }
    #[allow(non_snake_case)]
    pub fn ArgumentNull<S: ToString>(message: S) -> Self {
        Self::new(TiError::ArgumentNull, message.to_string())
    }
    #[allow(non_snake_case)]
    pub fn ArgumentOutOfRange<S: ToString>(message: S) -> Self {
        Self::new(TiError::ArgumentOutOfRange, message.to_string())
    }
    #[allow(non_snake_case)]
    pub fn ArgumentNotFound<S: ToString>(message: S) -> Self {
        Self::new(TiError::ArgumentNotFound, message.to_string())
    }
    #[allow(non_snake_case)]
    pub fn InvalidInterop<S: ToString>(message: S) -> Self {
        Self::new(TiError::InvalidInterop, message.to_string())
    }
    #[allow(non_snake_case)]
    pub fn InvalidState<S: ToString>(message: S) -> Self {
        Self::new(TiError::InvalidState, message.to_string())
    }
    #[allow(non_snake_case)]
    pub fn IncompatibleModule<S: ToString>(message: S) -> Self {
        Self::new(TiError::IncompatibleModule, message.to_string())
    }
    #[allow(non_snake_case)]
    pub fn OutOfMemory<S: ToString>(message: S) -> Self {
        Self::new(TiError::OutOfMemory, message.to_string())
    }
}
impl std::fmt::Display for TaichiError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?} {}", self.code, self.message)
    }
}
impl std::error::Error for TaichiError {
    fn description(&self) -> &str {
        self.message.as_str()
    }
}

pub type TaichiResult<T> = std::result::Result<T, TaichiError>;

pub fn get_last_error() -> TaichiResult<()> {
    let mut message_size: u64 = 0;
    let error = unsafe {
        ti_get_last_error(&mut message_size as *mut u64, std::ptr::null_mut())
    };
    if error >= TiError::Success {
        Ok(())
    } else {
        if message_size > 0 {
            let mut message: Vec<u8> = Vec::with_capacity(message_size as usize);
            unsafe {
                message.set_len(message_size as usize);
                ti_get_last_error(&mut message_size as *mut u64, message.as_mut_ptr() as *mut c_char);
            }
            let message = String::from_utf8_lossy(&message).to_string();
            Err(TaichiError::new(error, message))
        } else {
            Err(TaichiError::new(error, String::new()))
        }
    }
}

pub fn set_last_error(error: TaichiError) {
    unsafe {
        ti_set_last_error(error.code(), error.message().as_ptr() as *const c_char);
    }
}
