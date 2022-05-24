//! New York City, home of the dollar slice.

/// A dollar to buy a slice with.
pub struct Dollar;

/// Get a slice for just one dollar!
pub fn get_slice<Toppings>(_dollar: Dollar) -> &'static [Toppings] {
    &[]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    struct Pepperoni;

    #[test]
    fn it_works() {
        let dollar = Dollar;
        let slice: &[Pepperoni] = get_slice(dollar);
        assert_eq!(slice, &[]);
    }
}
