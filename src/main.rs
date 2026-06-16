use std::fmt::{Display, Write};

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

fn main() {
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
    println!("{grid}");

    // for_each_number_in_base::<2>(&base_10, |number| {
    //     println!(
    //         "{} in base 10 is {}",
    //         number.display_in_base, number.in_base_10
    //     )
    // });
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
