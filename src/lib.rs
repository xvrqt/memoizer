use std::hash::Hash;
use std::collections::HashMap;

pub struct Memoizer<U,V,F>
	where U: Eq + Hash + Clone,
		  V: Clone,
		  F: Fn(U) -> V
{
	function: F,
	map: HashMap<U,V>
}

impl<U,V,F> Memoizer<U,V,F>
	where U: Eq + Hash + Clone,
		  V: Clone,
	  	  F: Fn(U) -> V
{
	pub fn new(function: F) -> Memoizer<U,V,F> {
		Memoizer {
			function,
			map: HashMap::new()
		}
	}

	pub fn value(&mut self, arg: U) -> V {
		let key = arg.clone();
		self.map.entry(key).or_insert((self.function)(arg)).clone()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

    #[test]
    fn constructor() {
    	let mut add_two = Memoizer::new(|n| {
    		n + 2
    	});
        assert_eq!(4, add_two.value(2));
    }

    #[test]
    fn memoization() {
    	let mut add_two = Memoizer::new(|n| {
    		n + 2
    	});
        assert_eq!(4, add_two.value(2));
        assert_eq!(4, add_two.value(2));
    }

    #[test]
    fn multiple() {
    	let mut add_two = Memoizer::new(|n| {
    		n + 2
    	});
        assert_eq!(4, add_two.value(2));
        assert_eq!(5, add_two.value(3));
    }

    #[test]
    fn mixed_types() {
    	let mut length = Memoizer::new(|s: &str| {
    		s.len()
    	});
        assert_eq!("gaygirls".len(), length.value("gaygirls"));
        assert_eq!("gaygirls".len(), length.value("gaygirls"));

        assert_eq!(3, length.value("gay"));
    }

    #[derive(Debug, Clone, Hash)]
    struct Dummy {
    	pub field: usize,
    	pub field2: String
    }

    impl PartialEq for Dummy {
    	fn eq(&self, other: &Dummy) -> bool {
    		self.field == other.field && self.field2 == other.field2
    	}
    }

    impl Eq for Dummy {}

    #[test]
    fn structs() {
    	let d = Dummy { field: 1, field2: String::from("gay") };
    	let mut calc = Memoizer::new(|d: Dummy| {
    		d.field + d.field2.len()
    	});

        assert_eq!(4, calc.value(d));
    }

    #[test]
    fn structs_by_ref() {
    	let d = Dummy { field: 1, field2: String::from("gay") };
    	let mut calc = Memoizer::new(|d: &Dummy| {
    		let field = d.field + d.field2.len();
    		let field2 = d.field2.clone();
    		let new_dummy = Dummy { field, field2 };
    		new_dummy
    	});

    	let mut new = calc.value(&d);
        assert_eq!(Dummy { field: 4, field2: String::from("gay") }, new);
        new.field = 0;
        assert_eq!(Dummy { field: 4, field2: String::from("gay") }, calc.value(&d));
    }

    #[test]
    fn heap_allocated() {
    	let v = vec![1, 2, 3];
    	let v2 = vec![1, 2, 3];
    	let mut calc = Memoizer::new(|v: Vec<u32>| {
    		v.len()
    	});

    	assert_eq!(3, calc.value(v));
    	assert_eq!(3, calc.value(v2));
    }

    #[test]
    fn heap_returned() {
    	let v = vec![1, 2, 3];
    	let v2 = vec![1, 2, 3];
    	let mut calc = Memoizer::new(|v: Vec<u32>| {
    		let mut r = vec![3,2,1];
    		r.extend(v);
    		r
    	});

    	let mut calculated_v = calc.value(v);
    	let assert_v = vec![3,2,1,1,2,3];
    	for (i, e ) in calculated_v.iter().enumerate() {
    		assert_eq!(calculated_v[i], assert_v[i]);
    	}
    	calculated_v[0] = 23;

    	let calculated_v = calc.value(v2);
    	for (i, e ) in calculated_v.iter().enumerate() {
    		assert_eq!(calculated_v[i], assert_v[i]);
    	}
    }
}
