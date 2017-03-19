// subint — Operations on a "partial" integer
// Copyright (C) 2017  Ben Wiederhake
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.


pub mod raw;

#[derive(Clone, Copy)]
pub struct Subint {
    count: u32,
}

pub fn of(count: u32) -> Subint {
    debug_assert!(count <= 32, format!("Out of range: {}", count));
    Subint {
        count: count,
    }
}


/// Iterator object, yields single permutations and has a 'next' method.
/// Not intended to be stored explicitly, but can be used either way.
#[derive(Clone, Copy)]
pub struct PermIter {
    subint_count: u32,
    ones: u32,
    next: u32,
}

impl Iterator for PermIter {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.subint_count < self.ones {
            /* Invalid setup.  For example, someone wanted all
             * permutations of 8 bits within the last 5 bits.
             * There is no such permutation, so return 'None' right away. */
            None
        } else {
            let ret: u32 = self.next;
            self.next = raw::advance(self.subint_count, self.ones, self.next);
            if self.next <= ret {
                /* Wrapped around.  For the iterator, we don't want wrap-around,
                 * so just pretend next round that there was invalid input. */
                self.ones = u32::max_value();
            }
            Some(ret)
        }
    }
}

impl Subint {
    pub fn permute(&self, ones: u32) -> PermIter {
        PermIter{
            subint_count: self.count,
            ones: ones,
            next: raw::mk_ones(ones),
        }
    }
}

#[test]
fn test_perm_iter_corner() {
    assert_eq!(None, of(3).permute(4).next());
    assert_eq!(None, of(0).permute(1).next());
    assert_eq!(Some(0xFFFF_FFFF), of(32).permute(32).next());
}

#[test]
fn test_perm_iter_corner2() {
    let mut i = of(0).permute(0);
    assert_eq!(Some(0), i.next());
    assert_eq!(None, i.next());
}

#[test]
fn test_perm_iter_simple() {
    let mut i = of(5).permute(1);
    assert_eq!(Some(0b00001), i.next());
    assert_eq!(Some(0b00010), i.next());
    assert_eq!(Some(0b00100), i.next());
    assert_eq!(Some(0b01000), i.next());
    assert_eq!(Some(0b10000), i.next());
    assert_eq!(None, i.next());
    assert_eq!(None, i.next());
    assert_eq!(None, i.next());
    assert_eq!(None, i.next());
}

#[test]
fn test_perm_iter_full() {
    let mut i = of(4).permute(4);
    assert_eq!(Some(0b1111), i.next());
    assert_eq!(None, i.next());
}

#[test]
fn test_perm_collect() {
    let p = of(3).permute(1).collect::<Vec<_>>();
    assert_eq!(p, vec![0b001, 0b010, 0b100]);
}

#[test]
fn test_perm_for_consuming() {
    let mut perms = Vec::<u32>::with_capacity(3);
    for p in of(3).permute(1) {
        perms.push(p);
    }
    assert_eq!(perms, vec![0b001, 0b010, 0b100]);
}

impl Subint {
    pub fn invert(&self, v: u32) -> u32 {
        v ^ raw::mk_ones(self.count)
    }
}

#[test]
fn test_inv() {
    assert_eq!(0, of(0).invert(0));
    assert_eq!(0xCAFEBABEu32, of(0).invert(0xCAFEBABEu32));
    assert_eq!(u32::max_value(), of(32).invert(0));
    assert_eq!(0x12345678, of(32).invert(0xEDCBA987));
    assert_eq!(0x1234A987, of(16).invert(0x12345678));
}

#[test]
#[should_panic(expected = "Out of range: 33")]
fn test_of_panic() {
    of(33);
}
