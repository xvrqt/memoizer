//! # Memoizer
//! Memoizer is a struct that can cache results from expensive injective functions, allowing you to save time and processor cycles.
#![deny(missing_docs,
        missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unsafe_code,
        unstable_features,
        unused_import_braces, unused_qualifications)]

// Imports
use std::collections::HashMap;
use std::hash::Hash;

/// The eponymous struct. Can only memoize function that takes a single argument and returns a single value, if you need more than this, you can use vectors, arrays or structs of your own to pass in more than one value.
#[derive(Debug)]
pub struct Memoizer<U, V, F>
where
    U: Eq + Hash + Clone,
    V: Clone,
    F: Fn(U) -> V,
{
    function: F,
    map: HashMap<U, V>,
}

impl<U, V, F> Memoizer<U, V, F>
where
    U: Eq + Hash + Clone,
    V: Clone,
    F: Fn(U) -> V,
{
	/// Creates a new Memoize given a function.
	///
	/// # Examples
	///
	/// ```
	///# use memoizer::Memoizer;
	/// let mut add_two = Memoizer::new(|n| {
	///		n + 2
	///	});
	/// assert_eq!(4, add_two.value(2));
	/// ```
	///
    pub fn new(function: F) -> Memoizer<U, V, F> {
        Memoizer {
            function,
            map: HashMap::new(),
        }
    }

	/// Returns the value for the memoized function. If the function has already been called before, it will use the previous value. This means Memoizer should only be used for injective functions.
	///
	/// # Examples
	///
	/// ```
	///
	///# use memoizer::Memoizer;
	/// #[derive(Debug, Clone, Hash)]
	///	 struct Dummy {
	///	     pub id: usize,
	///	     pub word: String,
	///	 }
	///
	///	 /* PartialEq & Eq required for HashMap */
	///	 impl PartialEq for Dummy {
	///	     fn eq(&self, other: &Dummy) -> bool {
	///	         self.id == other.id && self.word == other.word
	///	     }
	///	 }
	///
	///	impl Eq for Dummy {}
	///
	/// let d = Dummy {
    ///     id: 1,
    ///     word: String::from("girls"),
    /// };
    /// let mut calc = Memoizer::new(|d: &Dummy| d.id + d.word.len());
    ///
    ///  assert_eq!(6, calc.value(&d));
    ///  assert_eq!(6, calc.value(&d));
	/// ```
	///
    pub fn value(&mut self, arg: U) -> V {
        let f = &self.function;
        let key = arg.clone();
        self.map.entry(key).or_insert_with(|| { (f)(arg) }).clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /* Constructor and closure testing */
    #[test]
    fn constructor() {
        let mut add_two = Memoizer::new(|n| n + 2);
        assert_eq!(4, add_two.value(2));
    }

    /* Trivial, Copy-able memoization */
    #[test]
    fn memoization() {
        let mut add_two = Memoizer::new(|n| n + 2);
        assert_eq!(4, add_two.value(2));
        assert_eq!(4, add_two.value(2));
    }

    /* Testing that multiple values can be memoized */
    #[test]
    fn multiple() {
        let mut add_two = Memoizer::new(|n| n + 2);
        assert_eq!(4, add_two.value(2));
        assert_eq!(4, add_two.value(2));

        assert_eq!(5, add_two.value(3));
        assert_eq!(5, add_two.value(3));
    }

    /* Testing memoization with different input/return types */
    #[test]
    fn mixed_types() {
        let mut length = Memoizer::new(|s: &str| s.len());
        assert_eq!("gaygirls".len(), length.value("gaygirls"));
        assert_eq!("gaygirls".len(), length.value("gaygirls"));

        assert_eq!(3, length.value("gay"));
    }

    /* Dummy struct to test more complex inputs/returns */
    #[derive(Debug, Clone, Hash)]
    struct Dummy {
        pub field: usize,
        pub field2: String,
    }

    /* PartialEq & Eq required for HashMap */
    impl PartialEq for Dummy {
        fn eq(&self, other: &Dummy) -> bool {
            self.field == other.field && self.field2 == other.field2
        }
    }

    impl Eq for Dummy {}

    /* Testing memoization with a struct input */
    #[test]
    fn structs() {
        let d = Dummy {
            field: 1,
            field2: String::from("gay"),
        };
        let mut calc = Memoizer::new(|d: Dummy| d.field + d.field2.len());

        assert_eq!(4, calc.value(d));
    }

    /* Pass structs as inputs by reference, return structs by value. Ensure
     * the returned values cannot be used to corrupt the memoization map.
     */
    #[test]
    fn structs_by_ref() {
        let d = Dummy {
            field: 1,
            field2: String::from("gay"),
        };
        let mut calc = Memoizer::new(|d: &Dummy| {
            let field = d.field + d.field2.len();
            let field2 = d.field2.clone();
            let new_dummy = Dummy { field, field2 };
            new_dummy
        });

        /* Create a new struct from reference, see if it's what is expected
         * from the calc closure.
        	*/
        let mut new = calc.value(&d);
        assert_eq!(
            Dummy {
                field: 4,
                field2: String::from("gay")
            },
            new
        );

        // Mutate the return struct to make sure it is not changing the map's value
        new.field = 0;
        assert_eq!(
            Dummy {
                field: 4,
                field2: String::from("gay")
            },
            calc.value(&d)
        );
    }

    /* Test passing in heap allocated types as inputs to the function */
    #[test]
    fn heap_allocated() {
        let v = vec![1, 2, 3];
        let v2 = vec![1, 2, 3];
        let mut calc = Memoizer::new(|v: Vec<u32>| v.len());

        assert_eq!(3, calc.value(v));
        assert_eq!(3, calc.value(v2));
    }
    /* Use heap allocated types in the input and return values. Ensure they can
     * not be used to corrupt the memoization map.
     */
    #[test]
    fn heap_returned() {
        let v = vec![1, 2, 3];
        let v2 = vec![1, 2, 3];
        let mut calc = Memoizer::new(|v: Vec<u32>| {
            let mut r = vec![3, 2, 1];
            r.extend(v);
            r
        });

        /* Create a new vector and see that it is what is expected from the
         * calc closure.
        	*/
        let mut calculated_v = calc.value(v);
        let assert_v = vec![3, 2, 1, 1, 2, 3];
        for (i, _) in calculated_v.iter().enumerate() {
            assert_eq!(calculated_v[i], assert_v[i]);
        }

        // Mutate the return vector to make sure it is not changing the map's value
        calculated_v[0] = 23;
        let calculated_v = calc.value(v2);
        for (i, _) in calculated_v.iter().enumerate() {
            assert_eq!(calculated_v[i], assert_v[i]);
        }
    }
}
