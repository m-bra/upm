
use std::cmp::{Ord, Ordering, PartialEq, PartialOrd};

#[test]
fn test() {
    PkgVersion::new().number("1").number("123456789").number("abcXYZ")
        .flag("a").flag("").flag("hellolollololtrollolol").finalize().ok().unwrap();
    PkgVersion::new().finalize().err().unwrap();
    PkgVersion::new().flag("hl").flag("lol").finalize().err().unwrap();
    PkgVersion::new().number("12A").number("Hello").finalize().err().unwrap();

    let version = match PkgVersion::parse("1.12.1.A.abc-debug-abc123") {
        Ok(version) => version,
        Err(err) => panic!("Failed to parse version: {:?}", err)
    };
    if version.numbers != vec![
            PkgVersionNumber("1".to_string()),
            PkgVersionNumber("12".to_string()),
            PkgVersionNumber("1".to_string()),
            PkgVersionNumber("A".to_string()),
            PkgVersionNumber("abc".to_string())] {
        panic!("version.numbers is {:?}", version.numbers);
    }
    if version.flags != vec!["debug".to_string(), "abc123".to_string()] {
        panic!("version.flags is {:?}", version.flags);
    }
}

#[derive(Debug)]
pub struct PkgVersion {
    numbers: Vec<PkgVersionNumber>,
    flags: Vec<String>,
}

#[derive(Debug)]
pub enum PkgVersionErr {
    NoNumbers,
    EmptyNumber,
    InvalidNumber,
    InvalidVersion,
}

pub type PkgVersionResult<T> = Result<T, PkgVersionErr>;

impl PkgVersion {
    pub fn parse(string: &str) -> PkgVersionResult<PkgVersion> {
        let mut builder = PkgVersion::new();

        // group n: numbers, group f: flags.
        // TODO: which character group for flags?     now having: [a-zA-Z0-9_]
        let re = regex!(r"^(?P<n>(\.([:alpha:]+|\d+))+)\.?(?P<f>(-[a-zA-Z0-9_]+)+)$");

        let prepared_string = ".".to_string() + string;

        let caps = match re.captures(prepared_string.as_slice()) {
            Some(c) => c,
            None => return Err(PkgVersionErr::InvalidVersion),
        };

        let numbers_string = match caps.name("n") {
            Some(s) => s,
            None => return Err(PkgVersionErr::NoNumbers)
        };

        // skip first because the prepared string starts with a dot
        for number_string in numbers_string.split_str(".").skip(1) {
            builder = builder.number(number_string);
        }

        if let Some(flags_string) = caps.name("f") {
            // skip first because the flag string starts with a dash
            for flag_string in flags_string.split_str("-").skip(1) {
                if !flag_string.is_empty() {
                    builder = builder.flag(flag_string);
                }
            }
        }

        builder.finalize()
    }

    pub fn new() -> PkgVersionBuilder {
        PkgVersionBuilder {
            building: PkgVersion {
                numbers: Vec::new(),
                flags: Vec::new()
            }
        }
    }
}

struct PkgVersionBuilder {
    building: PkgVersion,
}

impl PkgVersionBuilder {
    pub fn finalize(self) -> PkgVersionResult<PkgVersion> {
        if self.building.numbers.is_empty() {
            return Err(PkgVersionErr::NoNumbers)
        }

        for number in self.building.numbers.iter() {
            try!(number.check())
        }

        Ok(self.building)
    }

    pub fn number(mut self, number_string: &str) -> PkgVersionBuilder {
        self.building.numbers.push(PkgVersionNumber(number_string.to_string()));
        self
    }

    pub fn flag(mut self, flag: &str) -> PkgVersionBuilder {
        self.building.flags.push(flag.to_string());
        self
    }
}

impl PartialEq for PkgVersion {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl PartialOrd for PkgVersion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let zip_iter = self.numbers.iter().zip(other.numbers.iter());

        for (self_number, other_number) in zip_iter {
            if self_number != other_number {
                return self_number.partial_cmp(other_number)
            }
        }

        Some(Ordering::Equal)
    }
}

#[derive(Debug)]
struct PkgVersionNumber(String);

impl PkgVersionNumber {
    fn peek(&self) -> &String {
        let &PkgVersionNumber(ref result) = self;

        result
    }

    fn check(&self) -> PkgVersionResult<()> {
        let &PkgVersionNumber(ref data) = self;

        if data.is_empty() {
            return Err(PkgVersionErr::EmptyNumber)
        }
        if !data.chars().next().unwrap().is_alphanumeric() {
            return Err(PkgVersionErr::InvalidNumber)
        }

        let is_alpha = data.chars().next().unwrap().is_alphabetic();

        for c in data.chars() {
            if c.is_alphabetic() != is_alpha {
                return Err(PkgVersionErr::InvalidNumber)
            }
        }

        Ok(())
    }
}

impl PartialEq for PkgVersionNumber {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl PartialOrd for PkgVersionNumber {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let zip_iter = self.peek().chars().zip(other.peek().chars());

        for (self_char, other_char) in zip_iter {
            if self_char != other_char {
                // character can only be letter or number now!
                assert!(self_char.is_alphanumeric() && other_char.is_alphanumeric());
                // if characters are not same 'kind'
                if self_char.is_alphabetic() != other_char.is_alphabetic() {
                    return if self_char.is_alphabetic() {Some(Ordering::Less)} else {Some(Ordering::Greater)}
                }

                return Some(self_char.cmp(&other_char))
            }

            // if both chars are the same, continue
        }

        Some(Ordering::Equal)
    }
}
