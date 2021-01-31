use std::{
	borrow::Borrow,
	cell::RefCell,
	collections::HashMap,
	hash::Hash,
	mem::drop,
};

pub type Memoize<T> = RefCell<Option<T>>;
pub type MemoizeMap<K, V> = RefCell<HashMap<K, V>>;

#[inline(always)]
pub fn memo<T: Clone>(memo_field: &Memoize<T>, init: impl FnOnce() -> T) -> T {
	let field = memo_field.borrow();
	if let Some(v) = field.as_ref() {
		v.clone()
	} else {
		drop(field);
		let out = init();
		*memo_field.borrow_mut() = Some(out.clone());
		out
	}
}

#[inline(always)]
pub fn memo_map<K, Q, T: Clone>(memo_map: &MemoizeMap<K, T>, key: &Q, init: impl FnOnce() -> T) -> T
	where
		K: Eq + Hash + Borrow<Q>,
		Q: Eq + Hash + ToOwned<Owned=K> + ?Sized
{
	let map = memo_map.borrow();
	if let Some(out) = map.get(key) {
		out.clone()
	} else {
		drop(map);
		let out = init();
		memo_map.borrow_mut().insert(key.to_owned(), out.clone());
		out
	}
}
