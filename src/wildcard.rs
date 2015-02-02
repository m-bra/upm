
pub struct WildCard<T> {
    val_range: ValueRange<T>,
    next_comb: Option<(LogicConj, Box<WildCard<T>>)>,
}

/// a locical combination
pub enum LogicComb {
    And, Or, Xor
}

/// Define a range of values
///
/// For example:
/// Eq(t): range of values is all values equal to t
/// Lt(t): range of values less than t
pub enum ValueRange<T> {
    Any,
    Eq(T),
    Lt(T), Le(T),
    Gt(T), Ge(T),
}

impl<T> WildCard<T> {
    pub fn new(ValueRange<T> a_val_range) -> WildCard {
        WildCard {
            val_range: a_val_range,
            next_comb: None,
        }
    }

    pub fn combine(self, LogicConj a_conj, ValueRange<T> a_range) -> WildCard {
        WildCard {
            val_range: a_range,
            next_comb: Some((a_conj, box self)),
        }
    }
}

/// Checks if value matches wildcard. Value has to implement `Ord`
pub fn matches_wildcard<T: Ord>(x: T, wildcard: Wildcard<T>) -> bool {
    let matches_part = match wildcard.val_range {
        Any => true,
        Eq(y) => x == y,
        Lt(y) => x < y,
        Le(y) => x <= y,
        Gt(y) => x > y,
        Ge(y) => x >= y,
    }

    match wildcard.next_comb {
        Some((And, box next_wildcard)) => matches_part && matches_wildcard(x, next_wildcard),
        Some((Or,  box next_wildcard)) => matches_part || matches_wildcard(x, next_wildcard),
        Some((Xor, box next_wildcard)) => matches_part ^^ matches_wildcard(x, next_wildcard),
        None => matches_part
    }
}

/// Checks if value matches a wildcard only with ordering comparisions. Value has to implement `Ord`
pub fn matches_wildcard_eq<T: Eq>(x: T, wildcard: Wildcard<T>) -> Result<bool, String> {
    let matches_part = match wildcard.val_range {
        Any => true,
        Eq(y) => x == y,
        _ => return Err("Found ordering comparison while matching for equality only.".to_string())
    }

    match wildcard.next_comb {
        Some((And, box next_wildcard)) => matches_part && matches_wildcard(x, next_wildcard),
        Some((Or,  box next_wildcard)) => matches_part || matches_wildcard(x, next_wildcard),
        Some((Xor, box next_wildcard)) => matches_part ^^ matches_wildcard(x, next_wildcard),
        None => matches_part
    }
}
