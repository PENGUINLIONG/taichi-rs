use taichi_sys::*;

pub type TaichiError = TiError;

pub type TaichiResult<T> = std::result::Result<T, TaichiError>;
