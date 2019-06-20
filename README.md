# Memoizer
This struct allows you to memoize the results of a deterministic, pure functions to save computational resources on repeated function calls. A better version of this library exists [here](https://crates.io/crates/cached).

# Quickstart
Add the following to your Cargo.toml:
```TOML
[dependencies]
memoize = "0.2.0"
```

Add the following to your main/lib.rs:
```Rust
use memoizer::Memoizer;

fn main() {
    let mut increment = Memoizer::new(|n| {
        println!("Function Called");
        n + 1
    });

    
    println!("{}", increment.value(0));
    println!("{}", increment.value(0));
}   
```
This prints:
```rust
Fuction Called!
1
1
```

# Examples

## Trivial Expensive Function
The `difficult(n: usize)` function simulates a computationally expensive function. It taks about a 20th of a second to run and returns nothing. When called 500 times with random inputs (limited [0,9))it takes about ~2.5 seconds. When using the Memoizer struct it takes ~0.05 seconds.
```Rust
use std::thread::sleep;
use std::time::{Duration, Instant};

use rand::Rng;
use memoizer::Memoizer;

// Expensive Calculation
fn difficult(n: usize) {
    sleep(Duration::new(0,5000000));
}

fn main() {
    let mut rng = rand::thread_rng();

    let now = Instant::now();
    for i in 0..500 {
        difficult(rng.gen_range(0,10));
    }
    println!("Unmemoized Time: {:.2}", now.elapsed().as_millis() as f64 / 1000.0);

    let now = Instant::now();
    let mut memoized = Memoizer::new(difficult);
    for i in 0..500 {
        memoized.value(rng.gen_range(0,10));
    }
    println!("Memoized Time: {:.2}", now.elapsed().as_millis() as f64 / 1000.0);
}
```

This prints:
```bash
Unmemoized Time: 2.54
Memoized Time: 0.05
```

# Function Signature
The closure provided to the Memoize struct must take a single parameter and return a single value. Passing structs, references and heap allocated parameters is fine, but make sure they implement: Eq, Hash, Clone so they can be used as keys in the HashMap. Fortunately, most of these traits can be derived for complex types.

The return value must implement the Clone trait so that the value in the HashMap cannot be corrupted.

Here's an example using a struct as the input parameter and returning a number.

```Rust
use memoizer::Memoizer;

/* Dummy struct to test more complex inputs/returns */
#[derive(Debug, Clone, Hash)]
struct Dummy {
    pub id: usize,
    pub name: String,
}

/* PartialEq & Eq required for HashMap */
impl PartialEq for Dummy {
    fn eq(&self, other: &Dummy) -> bool {
        self.id == other.id && self.name == other.name
    }
}

impl Eq for Dummy {}

fn main() {
    let d = Dummy {
        id: 1,
        name: String::from("Amy"),
    };
    
    let mut calc = Memoizer::new(|d: Dummy| d.id + d.name.len());
    assert_eq!(4, calc.value(d));
}   
```

# Future Work
Currently, there is no ability to memoize recursive functions, which is unfortunate because dynamic programming is where a memoizer would really shine and they often employ recursion. Look for this in v0.3.0.