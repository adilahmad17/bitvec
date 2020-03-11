//! Iteration processes for `BitBox`.

use crate::{
	boxed::BitBox,
	order::BitOrder,
	slice::BitSlice,
	store::BitStore,
};

use core::iter::FusedIterator;

impl<O, T> IntoIterator for BitBox<O, T>
where
	O: 'static + BitOrder,
	T: 'static + BitStore,
{
	type IntoIter = IntoIter<O, T>;
	type Item = bool;

	fn into_iter(self) -> Self::IntoIter {
		let slice: &'static BitSlice<O, T> =
			unsafe { &*(self.as_bitslice() as *const _) };
		IntoIter {
			bitbox: self,
			iter: slice.iter(),
		}
	}
}

impl<'a, O, T> IntoIterator for &'a BitBox<O, T>
where
	O: 'a + BitOrder,
	T: 'a + BitStore,
{
	type IntoIter = <&'a BitSlice<O, T> as IntoIterator>::IntoIter;
	type Item = <Self::IntoIter as Iterator>::Item;

	fn into_iter(self) -> Self::IntoIter {
		self.as_bitslice().into_iter()
	}
}

impl<'a, O, T> IntoIterator for &'a mut BitBox<O, T>
where
	O: 'a + BitOrder,
	T: 'a + BitStore,
{
	type IntoIter = <&'a mut BitSlice<O, T> as IntoIterator>::IntoIter;
	type Item = <Self::IntoIter as Iterator>::Item;

	fn into_iter(self) -> Self::IntoIter {
		self.as_mut_bitslice().into_iter()
	}
}

/// State keeper for consuming iteration over a `BitBox`.
#[repr(C)]
pub struct IntoIter<O, T>
where
	O: 'static + BitOrder,
	T: 'static + BitStore,
{
	/// Owning pointer to the full slab
	bitbox: BitBox<O, T>,
	/// Interior iterator.
	iter: <&'static BitSlice<O, T> as IntoIterator>::IntoIter,
}

impl<O, T> Iterator for IntoIter<O, T>
where
	O: BitOrder,
	T: BitStore,
{
	type Item = bool;

	fn next(&mut self) -> Option<Self::Item> {
		self.iter.next().copied()
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		self.iter.size_hint()
	}

	fn count(self) -> usize {
		self.len()
	}

	fn nth(&mut self, n: usize) -> Option<Self::Item> {
		self.iter.nth(n).copied()
	}

	fn last(mut self) -> Option<Self::Item> {
		self.next_back()
	}
}

impl<O, T> DoubleEndedIterator for IntoIter<O, T>
where
	O: BitOrder,
	T: BitStore,
{
	fn next_back(&mut self) -> Option<Self::Item> {
		self.iter.next_back().copied()
	}
}

impl<O, T> ExactSizeIterator for IntoIter<O, T>
where
	O: BitOrder,
	T: BitStore,
{
}

impl<O, T> FusedIterator for IntoIter<O, T>
where
	O: BitOrder,
	T: BitStore,
{
}
