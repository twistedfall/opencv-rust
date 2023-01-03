use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::mem::drop;

pub type MemoizeMap<K, V> = RefCell<HashMap<K, V>>;

pub trait MemoizeMapExt<K, V> {
	fn memo_get<Q>(&self, key: &Q, init: impl FnOnce() -> V) -> V
	where
		K: Eq + Hash + Borrow<Q>,
		Q: Eq + Hash + ToOwned<Owned = K> + ?Sized,
		V: Copy;
}

impl<K, V> MemoizeMapExt<K, V> for MemoizeMap<K, V> {
	fn memo_get<Q>(&self, key: &Q, init: impl FnOnce() -> V) -> V
	where
		K: Eq + Hash + Borrow<Q>,
		Q: Eq + Hash + ToOwned<Owned = K> + ?Sized,
		V: Copy,
	{
		let map = self.borrow();
		if let Some(out) = map.get(key) {
			*out
		} else {
			drop(map);
			let out = init();
			self.borrow_mut().insert(key.to_owned(), out);
			out
		}
	}
}
