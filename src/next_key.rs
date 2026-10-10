// SPDX-FileCopyrightText: Copyright (c) 2023-2026 Yegor Bugayenko
// SPDX-License-Identifier: MIT

use crate::Map;

impl<V> Map<V> {
    /// Get the next key available for insertion.
    ///
    /// # Panics
    ///
    /// If no more keys left.
    #[inline]
    #[must_use]
    pub fn next_key(&self) -> usize {
        if self.first_free.is_def() {
            self.first_free.get()
        } else {
            panic!("No more keys available left");
        }
    }
}

#[cfg(test)]
#[path = "next_key_tests.rs"]
mod tests;
