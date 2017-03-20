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

#[cfg(test)]
use std;


pub fn mk_ones(amount: u32) -> u32 {
    debug_assert!(amount <= 32, "Out of range: {}", amount);

    // (1u32 << amount) - 1
    /* For 'amount == 0', this would be no problem, but 32 is non-trivial:
     * We want to end up with 32 one bits in this case (duh),
     * but '1u32.wrapping_shl', or any other 'shl', actually first computes
     * 'amount % 32'.  So we transform everything to 64 bit first.
     * Note that this means that it probably won't run as well
     * on x86 or other 32 bit archs.
     */
    1u64.wrapping_shl(amount).wrapping_sub(1) as u32
}

#[test]
fn test_mk_ones() {
    assert_eq!(0b0, mk_ones(0));
    assert_eq!(0b1, mk_ones(1));
    assert_eq!(0b11, mk_ones(2));
    assert_eq!(0xFF, mk_ones(8));
    assert_eq!(0x7FFF_FFFF, mk_ones(31));
    assert_eq!(0xFFFF_FFFF, mk_ones(32));
}

#[test]
#[should_panic(expected = "Out of range: 33")]
fn test_mk_ones_panic() {
    mk_ones(33);
}

/// Computes the lexicographically next permutation of the bitmask `last_pos`
/// *within* the last `part` bits.  Heavily inspired by
/// [~seander's bithacks](https://graphics.stanford.edu/~seander/bithacks.html#NextBitPermutation).
/// Closely related to a
/// [more general algorithm](https://github.com/BenWiederhake/masked_permute/blob/master/src/raw.rs#L78)
///
/// All math operations are intended to run very speedily on both 32-bit
/// and 64-bit hardware.  This is considered the "hot path".
///
/// Note that this implements wrap-around.  So inputting the lexicographically
/// *last* permutation will yield the lexicographically *first* permutation.
pub fn advance(part: u32, ones: u32, last_perm: u32) -> u32 {
    debug_assert_eq!(last_perm.count_ones(), ones);
    debug_assert!(ones <= part);

    // Set it up
    let t = last_perm | last_perm.wrapping_sub(1);
    /*
     * The 'wrapping_sub' is some hackery that needs justification.
     * Here's a line of reasoning:
     * - The underflow (-1u32) only occurs when last_perm is 0.
     * - Whenever last_perm is 0, then mask is 0
     * - Whenever mask is zero, then due to the "| !mask"
     *   we can stomach any undefined *value*.
     * - 'wrapping_sub' compiles down to a single 'leal -1(%rdi)' instruction,
     *   which we need anyway
     * The point is: literally any other way of substracting would be valid,
     * as long as an underflow does not halt execution.
     */

    // Exploit the carry-chain to set the bit that will be *set* in the
    // next permutation, and clean up.
    let next_upper = t.wrapping_add(1) & mk_ones(part);
    // This is essentially the "(((~t & -~t) - 1) >> (__builtin_ctz(v) + 1))"-part,
    // but I expect that 'count_ones()' is faster.  Can't under-/overflow.
    let need_ones = ones - next_upper.count_ones();
    // let next_lower = (1 << need_ones) - 1;
    let next_lower = mk_ones(need_ones);
    // And we're done.
    next_upper | next_lower
}

#[test]
fn test_advance_2_1() {
    assert_eq!(advance(2, 1, 0b01), 0b10);
    assert_eq!(advance(2, 1, 0b10), 0b01);
}

#[test]
fn test_advance_3_1() {
    assert_eq!(advance(3, 1, 0b001), 0b010);
    assert_eq!(advance(3, 1, 0b010), 0b100);
    assert_eq!(advance(3, 1, 0b100), 0b001);
}

#[test]
fn test_advance_2_0() {
    assert_eq!(advance(2, 0, 0b00), 0b00);
}

#[test]
fn test_advance_1_0() {
    assert_eq!(advance(1, 0, 0b00), 0b00);
}

#[test]
fn test_advance_0_0() {
    assert_eq!(advance(0, 0, 0b00), 0b00);
}

#[test]
fn test_advance_corner() {
    let u32max = std::u32::MAX;

    assert_eq!(advance(32, 31, 0xFFFF_FFF0 | 0b1101), 0xFFFF_FFF0 | 0b1110);
    assert_eq!(advance(32, 31, 0xFFFF_FFFE), 0x7FFF_FFFF);
    assert_eq!(advance(32, 32, u32max), u32max);
}
