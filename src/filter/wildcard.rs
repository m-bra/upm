
use filter::Filter;

#[test]
fn test() {
    {
        let wildcard = WildCard::<isize>
            ::new(Condition::Lt(10))
            .combine(LogicComb::And, Condition::Ge(4))
            .combine(LogicComb::Or, Condition::Eq(20));
        assert!(wildcard.matches(&20).ok().unwrap());
        assert!(wildcard.matches(&5).ok().unwrap());
        assert!(wildcard.matches(&9).ok().unwrap());
        assert!(wildcard.matches(&4).ok().unwrap());
        assert!(!wildcard.matches(&10).ok().unwrap());
        assert!(!wildcard.matches(&3).ok().unwrap());
        assert!(!wildcard.matches(&12).ok().unwrap());
    }
    {
        let wildcard = WildCard::<f32>
            ::new(Condition::Any)
            .combine(LogicComb::And, Condition::Any)
            .combine(LogicComb::Or, Condition::Any);
        assert!(wildcard.matches(&1f32).ok().unwrap());
        assert!(wildcard.matches(&50.07f32).ok().unwrap());
        assert!(wildcard.matches(&-23f32).ok().unwrap());
    }
}

pub struct WildCard<T>(Condition<T>, Option<(LogicComb, Box<WildCard<T>>)>);

#[derive(Copy)]
pub enum LogicComb {And, Or, Xor}

/// Define a range of values
///
/// For example:
/// Eq(t): range of values is all values equal to t
/// Lt(t): range of values less than t
pub enum Condition<T> {
    Any,
    Eq(T), Not(T),
    Lt(T), Le(T),
    Gt(T), Ge(T),
}

pub enum WildCardErr {
    UnmatchedBracket,
}

pub type WildCardResult<T> = Result<T, WildCardErr>;

impl<T> WildCard<T> {
    pub fn parse(string: &str) -> WildCardResult<Self> {
        // remove optional brackets
        let mut prepared = String::new();
        let mut bracket_counter = 0;
        for c in string.chars() {
            match c {
                '(' => bracket_counter+= 1,
                ')' => bracket_counter-= 1,
                _ => prepared.push(c),
            }
        }

        if bracket_counter != 0 {
            return Err(WildCardErr::UnmatchedBracket)
        }

        // groups: lt: less than, le: less or equal, gt, ge, eq, ne: not equal, x: the value, and, or, xor, nxt: the next wildcard
        let re = regex!(r"^(?P<lt><)|(?P<le><=)|(?P<gt>>)|(?P<ge>>=)|(?P<eq>=?)|(?P<ne>!)(?P<x>[^&\|\^]+)(?P<and>&)|(?P<or>\|)|(?P<xor>\^)(P<nxt>.*)$");
        unimplemented!();
    }

    pub fn new(cond: Condition<T>) -> Self {
        WildCard (cond, None)
    }

    pub fn combine(self, conj: LogicComb, cond: Condition<T>) -> Self {
        WildCard (cond, Some((conj, Box::new(self))))
    }
}

impl<T> Filter<T> for WildCard<T> where T: PartialEq + PartialOrd {
    fn matches(&self, x: &T) -> Result<bool, &str> {
        let &WildCard::<T> (ref cond, ref next_comb) = self;

        let matches_part = match *cond {
            Condition::Any => true,
            Condition::Eq(ref y) => PartialEq::eq(x, y),
            Condition::Not(ref y) => PartialEq::ne(x, y),
            Condition::Lt(ref y) => PartialOrd::lt(x, y),
            Condition::Le(ref y) => PartialOrd::le(x, y),
            Condition::Gt(ref y) => PartialOrd::gt(x, y),
            Condition::Ge(ref y) => PartialOrd::ge(x, y),
        };

        Ok(match *next_comb {
            Some((LogicComb::And, ref next_wildcard)) => try!((**next_wildcard).matches(x)) && matches_part,
            Some((LogicComb::Or,  ref next_wildcard)) => try!((**next_wildcard).matches(x)) || matches_part,
            Some((LogicComb::Xor, ref next_wildcard)) => try!((**next_wildcard).matches(x)) ^ matches_part,
            None => matches_part,
        })
    }
}
