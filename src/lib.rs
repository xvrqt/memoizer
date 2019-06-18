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
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
