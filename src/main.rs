#![feature(integer_casts)]
use std::{
    fmt::{Display, Write},
    ops::{AddAssign, DivAssign, Rem},
};

fn main() {
    let mut base_26 = NewBase::<char>::new();
    base_26 += 'a'..='z';
    for char in base_26.0.iter() {
        println!("{char}");
    }

    let display = base_26.to(10);
    println!("{display}");
}

/// https://cs.stackexchange.com/questions/10318/the-math-behind-converting-from-any-base-to-any-base-without-going-through-base
fn to_digits(mut n: u32, b: u32) -> Vec<u32> {
    let mut digits = vec![];

    while n > 0 {
        digits.insert(0, n % b);
        n /= b;
    }

    digits
}
fn from_digits(digits: &[u32], b: u32) -> u32 {
    let mut n = 0;

    for d in digits {
        n = b * n + d;
    }

    n
}

#[derive(Clone, Copy)]
struct DisplayInBase<'a, N: Number, T> {
    base: &'a NewBase<T>,
    value: N,
}
impl<N: Number, T: Display + Clone> Display for DisplayInBase<'_, N, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut number = self.value.clone();
        let mut digits = vec![];
        let base = N::rhs_from_usize(self.base.0.len());

        while number.not_zero() {
            let digit = number.remainder(&base);
            number /= &base;
            digits.insert(0, self.base.0[digit.to_usize()].clone());
        }

        for digit in digits {
            write!(f, "{digit}")?;
        }

        Ok(())
    }
}

struct NewBase<T = char>(Vec<T>);
impl<T> NewBase<T> {
    fn new() -> Self {
        Self(vec![])
    }

    fn to<N: Number>(&self, value: N) -> DisplayInBase<'_, N, T> {
        DisplayInBase { base: self, value }
    }
}
impl AddAssign<core::ops::RangeInclusive<char>> for NewBase<char> {
    fn add_assign(&mut self, rhs: core::ops::RangeInclusive<char>) {
        self.0.extend(rhs);
    }
}
impl AddAssign<char> for NewBase<char> {
    fn add_assign(&mut self, rhs: char) {
        self.0.push(rhs);
    }
}

trait Number: Sized + Clone + for<'a> DivAssign<&'a Self::Rhs> {
    type Rhs: Clone;
    fn rhs_from_usize(value: usize) -> Self::Rhs;

    fn not_zero(&self) -> bool;
    fn to_usize(&self) -> usize;

    fn remainder(&self, rhs: &Self::Rhs) -> Self;
}
impl Number for u32 {
    type Rhs = Self;
    fn rhs_from_usize(value: usize) -> Self::Rhs {
        value.strict_cast()
    }

    fn not_zero(&self) -> bool {
        *self != 0
    }
    fn to_usize(&self) -> usize {
        self.strict_cast()
    }

    fn remainder(&self, rhs: &Self::Rhs) -> Self {
        self.rem(rhs)
    }
}

fn base_alphabet<T: Number>(value: &str) -> T {
    let base: [char; 26] =
        core::array::from_fn(|index| char::from_u32('a' as u32 + index as u32).unwrap());
    todo!()
}

mod base {
    use std::fmt::Display;

    struct Base<const DIGITS_LENGTH: usize> {
        digits: [(&'static str, i8); DIGITS_LENGTH],
    }

    struct Integer<const DIGITS_LENGTH: usize> {
        base: Base<DIGITS_LENGTH>,
        value: i32,
    }

    impl<const DIGITS_LENGTH: usize> Display for Integer<DIGITS_LENGTH> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            //f.
            //while n > 0 {
            //digits.insert(0, n % b);
            //n /= b;
            //}

            //digits
            todo!()
        }
    }
}

struct Base {
    base: u8,
    symbols: Vec<(&'static str, i8)>,
}

mod value {
    pub const Z: (&str, i8) = ("Z", 22);
    pub const B: (&str, i8) = ("B", 13);
    pub const L: (&str, i8) = ("L", -1);
}

trait Digits {
    fn to_digits(&self) -> impl Iterator<Item = &(&str, i8)>;
}

impl<const LENGTH: usize> Digits for [(&str, i8); LENGTH] {
    fn to_digits(&self) -> impl Iterator<Item = &(&str, i8)> {
        self.iter()
    }
}

fn blah(digits: impl Digits) {
    // for &(digit, value) in digits.to_digits() {
    //     for &(digit2, value2) in digits.to_digits() {
    //         let value = value * 10 + value2;
    //         println!("{digit}{digit2} in regular base 10 is {value}")
    //     }
    // }
    inner(&digits, 1);
}

// fn base_10_to_other_base(mut number: u32) {
//     let other: Vec<usize> = vec![];

//     while number != 0 {}
// }

struct BaseNumber<'a> {
    display_in_base: &'a str,
    in_base_10: i32,
}

fn for_each_number_in_base<const CAPACITY: usize>(
    base: &[(&str, i8)],
    mut f: impl for<'a> FnMut(BaseNumber<'a>),
) {
    let mut base_indices: [usize; CAPACITY] = [0; _];
    let mut display_in_base = String::with_capacity(CAPACITY);

    loop {
        display_in_base.clear();
        for &value_index in base_indices.iter().rev() {
            display_in_base += base[value_index].0;
        }

        let mut in_base_10: i32 = 0;
        for (iteration_index, &base_index) in base_indices.iter().enumerate() {
            let value = base[base_index].1;
            in_base_10 += value as i32 * base.len().pow(iteration_index as u32) as i32;
        }

        f(BaseNumber {
            display_in_base: &display_in_base,
            in_base_10,
        });

        if base_indices
            .iter()
            .all(|&value_index| value_index == base.len() - 1)
        {
            break;
        }

        for value_index in &mut base_indices {
            *value_index += 1;
            if *value_index == base.len() {
                *value_index = 0
            } else {
                break;
            }
        }
    }
}

fn every_3_digit_in_base_2() {
    let mut value_indices: [usize; 3] = [0; _];
    let mut display = String::with_capacity(3);

    loop {
        display.clear();
        for &value_index in value_indices.iter().rev() {
            write!(display, "{value_index}").unwrap()
        }
        println!("{display}");

        if value_indices.iter().all(|&value_index| value_index == 1) {
            break;
        }

        for value_index in &mut value_indices {
            *value_index += 1;
            if *value_index == 2 {
                *value_index = 0
            } else {
                break;
            }
        }
    }
}

struct ValueVec<const CAPACITY: usize>([i8; CAPACITY], u8);

impl<const CAPACITY: usize> ValueVec<CAPACITY> {
    fn push(value: i8) {
        todo!()
    }
}

fn inner<T: Digits>(digits: &T, remaining: u8) {
    if remaining == 0 {
        println!("Got.");
        return;
    }

    for &(digit, value) in digits.to_digits() {
        inner(digits, remaining - 1);
    }
}

fn main_old() {
    // blah([
    //     ("0", 0),
    //     ("1", -1),
    //     ("2", 2),
    //     ("3", 3),
    //     ("4", 4),
    //     ("5", 5),
    //     ("6", 6),
    //     ("7", 7),
    //     ("8", 8),
    //     ("9", 9),
    // ]);
    //crazy::<5>();
    //every_3_digit_in_base_2();
    //crazy::<3>(&[("0", 0), ("a", 1)])

    let base_10 = [
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ];

    let base_2 = [("0", 0), ("1", 1)];

    let one = [
        ("0", 0),
        ("1", 1),
        value::Z,
        value::B,
        ("4", 4),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        value::L,
    ];

    let bad = [
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        //("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ];

    let mut grid = Grid::<200>(std::array::repeat("  ".to_owned()));
    for_each_number_in_base::<3>(&bad, |number| {
        if let Some(value) = grid.0.get_mut(number.in_base_10 as usize) {
            *value = number.display_in_base.to_owned();
        }
    });
    //println!("{grid}");

    // for_each_number_in_base::<2>(&base_10, |number| {
    //     println!(
    //         "{} in base 10 is {}",
    //         number.display_in_base, number.in_base_10
    //     )
    // });

    //println!("{:?}", to_digits(52, 10));
    //println!("{:?}", from_digits(&[0, 0, 1], 2));

    for i in 0..10 {
        println!("{:?}", to_digits(i, 2));
    }
}

struct Grid<const CAPACITY: usize>([String; CAPACITY]);

impl<const CAPACITY: usize> Display for Grid<CAPACITY> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.0.chunks(10) {
            for column in row {
                write!(f, "{column}, ")?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
