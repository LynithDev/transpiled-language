use std::{fmt::Display, iter::Peekable};

use crate::error::SourceFile;

pub trait ComponentErrors<E>
where E: Display {

    fn fetch_errors(&self) -> &Vec<E>;

    fn has_errors(&self) -> bool {
        !self.fetch_errors().is_empty()
    }

    fn print_errors(&self) {
        for error in self.fetch_errors() {
            println!("{}", error)
        }
    }

    fn source(&self) -> &SourceFile;
}

pub trait ComponentIter<'a, C, T, I> where 
    C: PartialEq<T> + PartialEq,
    T: PartialEq<C> + PartialEq + Clone + 'a,
    I: Iterator<Item = T> + 'a {

    fn get_iter(&mut self) -> &mut Peekable<I>;
    fn cursor_next(&mut self, item: &T);

    /// Skip the list until an item of the same type in `term` is found
    fn skip_until(&mut self, term: &[T]) {
        while let Some(item) = self.peek() {
            if term.contains(item) {
                break;
            }

            self.next();
        }
    }

    /// Iterates to the next item
    fn next(&mut self) -> Option<T> {
        if let Some(item) = self.get_iter().next() {
            self.cursor_next(&item);
            Some(item.to_owned())
        } else {
            None
        }
    }

    /// Iterates to the next item if the next item is equal to the item argument
    fn next_if_eq(&mut self, item: &C) -> Option<T> {
        if self.peek_is(item) {
            self.next()
        } else {
            None
        }
    }

    /// Iterates to the next item if the next item is equal to the item argument
    fn next_if_eq_mul(&mut self, items: &[C]) -> Option<T> {
        if self.peek().is_some_and(|t| items.iter().any(|c| c == t)) {
            self.next()
        } else {
            None
        }
    }

    /// Expects a item to be there
    fn expect(&mut self, expected: &C) -> std::result::Result<T, Option<T>> {
        let Some(item) = self.peek() else {
            return Err(None);
        };

        if expected == item {
            let cloned = item.clone();
            self.next();
            Ok(cloned)
        } else {
            Err(Some(item.clone()))
        }
    }

    /// Expects one of an item to be there
    fn expect_any(&mut self, expected: &[C]) -> std::result::Result<T, Option<T>> {
        let Some(item) = self.peek() else {
            return Err(None);
        };

        if expected.iter().any(|t| t == item) {
            let cloned = item.clone();
            self.next();
            Ok(cloned)
        } else {
            Err(Some(item.clone()))
        }
    }

    /// Checks if the next item is equal to the item argument
    fn peek_is(&mut self, item: &C) -> bool {
        if let Some(peek) = self.peek() {
            peek == item
        } else {
            false
        }
    }

    /// Returns the next item if exists without iterating
    fn peek<'b>(&'b mut self) -> Option<&'b T>
    where I: 'b {
        self.get_iter().peek()
    }
}