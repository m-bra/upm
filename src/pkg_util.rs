
pub struct PkgVersionNumber: String;
pub struct PkgVersionFlag: String;

pub struct PkgVersion {
    numbers: Vec<PkgVersionNumber>,
    flags: Vec<PkgVersionFlag>,
}
