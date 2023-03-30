use taichi_sys::ti_get_version;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Version(u32);
impl Version {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self(major * 1000000 + minor * 1000 + patch)
    }

    pub fn major(&self) -> u32 {
        self.0 / 1000000
    }
    pub fn minor(&self) -> u32 {
        (self.0 / 1000) % 1000
    }
    pub fn patch(&self) -> u32 {
        self.0 % 1000
    }
}

impl From<u32> for Version {
    fn from(v: u32) -> Self {
        Self(v)
    }
}
impl From<Version> for u32 {
    fn from(v: Version) -> Self {
        v.0
    }
}

pub fn get_version() -> Version {
    let version = unsafe { ti_get_version() };
    Version::from(version)
}
