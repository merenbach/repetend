// Copyright 2021 Andrew Merenbach
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate num;

use num::{CheckedMul, Integer};
use std::iter;

#[cfg(test)]
mod tests {
    use super::divide;

    #[test]
    fn test_divide() {
        assert_eq!(
            divide(1, 7, 20, 10),
            [0, 1, 4, 2, 8, 5, 7, 1, 4, 2, 8, 5, 7, 1, 4, 2, 8, 5, 7, 1, 4]
        );

        assert_eq!(
            divide(10, 7, 20, 10),
            [1, 4, 2, 8, 5, 7, 1, 4, 2, 8, 5, 7, 1, 4, 2, 8, 5, 7, 1, 4, 2]
        );

        assert_eq!(
            divide(100, 7, 20, 10),
            [14, 2, 8, 5, 7, 1, 4, 2, 8, 5, 7, 1, 4, 2, 8, 5, 7, 1, 4, 2, 8]
        );

        assert_eq!(
            divide(100, 7, 20, 10),
            [14, 2, 8, 5, 7, 1, 4, 2, 8, 5, 7, 1, 4, 2, 8, 5, 7, 1, 4, 2, 8]
        );

        assert_eq!(
            divide(142, 38, 40, 10),
            [
                3, 7, 3, 6, 8, 4, 2, 1, 0, 5, 2, 6, 3, 1, 5, 7, 8, 9, 4, 7, 3, 6, 8, 4, 2, 1, 0, 5,
                2, 6, 3, 1, 5, 7, 8, 9, 4, 7, 3, 6, 8
            ]
        );
    }
}

// Divide `a / b` out to `c` digits.
// Divide returns the whole number component and the decimal expansion.
// Divide does not apply rounding to the end of the decimal expansion.
// Divide currently will not properly handle a base other than 10, at least for the first element.
pub fn divide<T>(a: T, b: T, c: usize, radix: T) -> Vec<T>
where
    T: Integer + CheckedMul,
{
    iter::successors(Some(a), |n| radix.checked_mul(&n.mod_floor(&b)))
        .take(c + 1)
        .map(|x| x.div_floor(&b))
        .collect()
}
