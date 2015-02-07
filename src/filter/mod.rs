
pub mod wildcard;
pub mod version;

pub trait Filter<T> {
    fn matches(&self, x: &T) -> Result<bool, &str>;
}
