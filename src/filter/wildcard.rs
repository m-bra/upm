
use filter::Filter;

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

impl<T> WildCard<T> {
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
    /*{
        let wildcard = WildCard::<f32>
            ::new(Condition::Any)
            .combine(LogicComb::And, Condition::Any)
            .combine(LogicComb::Or, Condition::Any);
        Filter::matches(&wildcard, 1);
        assert!(wildcard.matches(1).ok().unwrap());
        assert!(wildcard.matches(50.07).ok().unwrap());
        assert!(wildcard.matches(-23).ok().unwrap());
    }*/
}
