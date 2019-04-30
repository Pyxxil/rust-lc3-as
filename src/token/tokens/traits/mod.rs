use token::Token;

use std::vec::Drain;

pub trait Assemble {
    fn assemble(&mut self);

    fn assembled(self, program_counter: &mut i16) -> Vec<(u16, String)>;
}

pub trait Requirements {
    /* The amount of tokens that the token requires for operands.
     */
    fn require_range(&self) -> (u64, u64);

    /* Whether or not the token's requirements have been satisfied.
     */
    fn is_satisfied(&self) -> bool;

    /* Consume a range of tokens corresponding to Requirements::require_amount (at most).
     *
     * @param: from The vector containing the tokens we can consume from.
     * @param at The index to begin consuming from
     *
     * @return The number of consumed tokens (TODO: This might not be required, as Vec::remove does update the length).
     */
    fn consume(&mut self, tokens: Vec<Token>) -> Vec<Token>;
}

// Copyright (c) 2017 Alex Sayers

// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:

// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

pub trait DrainWhileable<T> {
    /// Take the elements of a vector, left-to-right, stopping at the first non-matching element.
    ///
    /// The returned `DrainWhile` iterates over the longest prefex in which all elements satisfy
    /// `pred`; when the iterator is dropped, the prefix is removed from `vec`.
    ///
    /// ```
    /// # use drain_while::*;
    /// let mut orig: Vec<usize> = vec![0,1,2,3,4,5];
    /// let none: Vec<usize> = orig.drain_while(|_| false).collect();
    /// let some: Vec<usize> = orig.drain_while(|x| *x < 3).collect();
    /// let rest: Vec<usize> = orig.drain_while(|_| true).collect();
    ///
    /// assert_eq!(none, vec![]);
    /// assert_eq!(some, vec![0,1,2]);
    /// assert_eq!(rest, vec![3,4,5]);
    /// assert_eq!(orig, vec![]);
    /// ```
    ///
    /// The behaviour of `drain_while()` differs from `drain().take_while()` in the final state of
    /// the original vector, as illustrated here:
    ///
    /// ```
    /// # use drain_while::*;
    /// let mut v1 = vec![1,2,3,4,5];
    /// let mut v2 = vec![1,2,3,4,5];
    /// v1.drain(..).take_while(|x| *x < 3);
    /// v2.drain_while(|x| *x < 3);
    /// assert_eq!(v1, vec![]);
    /// assert_eq!(v2, vec![3,4,5]);
    /// ```
    ///
    /// The same caveats which apply to `drain` also apply to `drain_while`:
    ///
    /// 1. The element range is removed even if the iterator is only partially consumed or not
    ///    consumed at all.
    /// 2. It is unspecified how many elements are removed from the vector, if the `DrainWhile`
    ///    value is leaked.
    ///
    /// The current implementation is fairly naive, but I believe there's scope for speeding it up
    /// substantially.
    fn drain_while<P>(&mut self, pred: P) -> DrainWhile<T>
    where
        P: Fn(&T) -> bool;
}

/// A draining iterator for `Vec<T>`.
///
/// See [`Vec::drain_while`](trait.DrainWhileable.html#tymethod.drain_while) for more.
//
// Just a newtype to allow changing the implementation later.
pub struct DrainWhile<'a, T: 'a>(Drain<'a, T>);
impl<'a, T> Iterator for DrainWhile<'a, T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.0.next()
    }
}

impl<T> DrainWhileable<T> for Vec<T> {
    // TODO: Surely this can be implemented more efficiently, but it may not be worth the effort...
    fn drain_while<P>(&mut self, mut pred: P) -> DrainWhile<T>
    where
        P: FnMut(&T) -> bool,
    {
        // This is purely a performance optimisation for the 0-matching case.
        match self.iter().position(|x| !pred(x)) {
            None =>
            /* they all matched pred */
            {
                DrainWhile(self.drain(..))
            }
            Some(i) =>
            /* they matched until i */
            {
                DrainWhile(self.drain(..i))
            }
        }
    }
}
