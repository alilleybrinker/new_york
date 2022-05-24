# `new_york`

Home the dollar slice!

## Example

```rust
use new_york::*;

// Choose your toppings.
#[derive(Debug, PartialEq, Eq)]
struct Pepperoni;

fn main() {
	// Get a slice for just one dollar!
	let dollar = Dollar;
	let slice: &[Pepperoni] = get_slice(dollar);
	assert_eq!(slice, &[]);
}
```

