mod wildcard;

pub struct PkgVersion {
    numbers: Vec<PkgVersionNumber>,
    flags: Vec<PkgVersionFlag>,
}

/// One number in the version of a package
///
/// For example:
/// The version `1.2a.13` consists of the numbers 1, 2a, and 13.
///
/// A version number is an alphanumeric string.
/// It may also just be a border, as in `1.>2.0`
/// which means the version number in the middle must be bigger than two.
pub enum PkgVersionNumber {

}

pub struct PkgVersionFlag: String;

impl PkgVersion {
    pub fn from_string(s: String) -> PkgVersion {
        let mut pkg_v: PkgVersion;


    }

    pub fn is_full() -> bool {

    }
}
