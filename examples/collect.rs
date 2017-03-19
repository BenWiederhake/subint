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

extern crate subint;

fn main() {
    let actual = subint::of(4).permute(2).collect::<Vec<u32>>();
    let expected = vec![0b0011, 0b0101, 0b0110, 0b1001, 0b1010, 0b1100];
    assert_eq!(actual, expected);
}
