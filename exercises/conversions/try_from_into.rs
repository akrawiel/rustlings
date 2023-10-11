// try_from_into.rs
//
// TryFrom is a simple and safe type conversion that may fail in a controlled
// way under some circumstances. Basically, this is the same as From. The main
// difference is that this should return a Result type instead of the target
// type itself. You can read more about it at
// https://doc.rust-lang.org/std/convert/trait.TryFrom.html
//
// Execute `rustlings hint try_from_into` or use the `hint` watch subcommand for
// a hint.

use std::convert::{TryFrom, TryInto};

#[derive(Debug, PartialEq)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// We will use this error type for these `TryFrom` conversions.
#[derive(Debug, PartialEq)]
enum IntoColorError {
    // Incorrect length of slice
    BadLen,
    // Integer conversion error
    IntConversion,
}

// Your task is to complete this implementation and return an Ok result of inner
// type Color. You need to create an implementation for a tuple of three
// integers, an array of three integers, and a slice of integers.
//
// Note that the implementation for tuple and array will be checked at compile
// time, but the slice implementation needs to check the slice length! Also note
// that correct RGB color values must be integers in the 0..=255 range.

impl Color {
    fn i16_to_u8(number: i16) -> Result<u8, IntoColorError> {
        if (0..=255).contains(&number) {
            Ok(number as u8)
        } else {
            Err(IntoColorError::IntConversion)
        }
    }

    fn i16_to_u8_ref(number: &i16) -> Result<u8, IntoColorError> {
        if (0..=255).contains(number) {
            Ok(*number as u8)
        } else {
            Err(IntoColorError::IntConversion)
        }
    }
}

// Tuple implementation
impl TryFrom<(i16, i16, i16)> for Color {
    type Error = IntoColorError;
    fn try_from(tuple: (i16, i16, i16)) -> Result<Self, Self::Error> {
        let numbers = Into::<[i16; 3]>::into(tuple).map(Self::i16_to_u8);

        if numbers.iter().any(|color| color.is_err()) {
            return Err(IntoColorError::IntConversion);
        }

        let [red, green, blue] = numbers.map(|color| color.unwrap());

        Ok(Color { red, green, blue })
    }
}

// Array implementation
impl TryFrom<[i16; 3]> for Color {
    type Error = IntoColorError;
    fn try_from(arr: [i16; 3]) -> Result<Self, Self::Error> {
        let numbers = arr.map(Self::i16_to_u8);

        if numbers.iter().any(|color| color.is_err()) {
            return Err(IntoColorError::IntConversion);
        }

        let [red, green, blue] = numbers.map(|color| color.unwrap());

        Ok(Color { red, green, blue })
    }
}

// Slice implementation
impl TryFrom<&[i16]> for Color {
    type Error = IntoColorError;
    fn try_from(slice: &[i16]) -> Result<Self, Self::Error> {
        let numbers = slice.iter();

        if numbers.len() != 3 {
            return Err(IntoColorError::BadLen);
        }

        let mut numbers = slice.iter().map(|color| Self::i16_to_u8_ref(color));

        if numbers.any(|color| color.is_err()) {
            return Err(IntoColorError::IntConversion);
        }

        let mut numbers = slice.iter().map(|color| Self::i16_to_u8_ref(color).unwrap());

        let red = numbers.next().unwrap();
        let green = numbers.next().unwrap();
        let blue = numbers.next().unwrap();

        Ok(Color { red, green, blue })
    }
}

fn main() {
    // Use the `try_from` function
    let c1 = Color::try_from((183, 65, 14));
    println!("{:?}", c1);

    // Since TryFrom is implemented for Color, we should be able to use TryInto
    let c2: Result<Color, _> = [183, 65, 14].try_into();
    println!("{:?}", c2);

    let v = vec![183, 65, 14];
    // With slice we should use `try_from` function
    let c3 = Color::try_from(&v[..]);
    println!("{:?}", c3);
    // or take slice within round brackets and use TryInto
    let c4: Result<Color, _> = (&v[..]).try_into();
    println!("{:?}", c4);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tuple_out_of_range_positive() {
        assert_eq!(
            Color::try_from((256, 1000, 10000)),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_tuple_out_of_range_negative() {
        assert_eq!(
            Color::try_from((-1, -10, -256)),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_tuple_sum() {
        assert_eq!(
            Color::try_from((-1, 255, 255)),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_tuple_correct() {
        let c: Result<Color, _> = (183, 65, 14).try_into();
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14
            }
        );
    }
    #[test]
    fn test_array_out_of_range_positive() {
        let c: Result<Color, _> = [1000, 10000, 256].try_into();
        assert_eq!(c, Err(IntoColorError::IntConversion));
    }
    #[test]
    fn test_array_out_of_range_negative() {
        let c: Result<Color, _> = [-10, -256, -1].try_into();
        assert_eq!(c, Err(IntoColorError::IntConversion));
    }
    #[test]
    fn test_array_sum() {
        let c: Result<Color, _> = [-1, 255, 255].try_into();
        assert_eq!(c, Err(IntoColorError::IntConversion));
    }
    #[test]
    fn test_array_correct() {
        let c: Result<Color, _> = [183, 65, 14].try_into();
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14
            }
        );
    }
    #[test]
    fn test_slice_out_of_range_positive() {
        let arr = [10000, 256, 1000];
        assert_eq!(
            Color::try_from(&arr[..]),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_slice_out_of_range_negative() {
        let arr = [-256, -1, -10];
        assert_eq!(
            Color::try_from(&arr[..]),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_slice_sum() {
        let arr = [-1, 255, 255];
        assert_eq!(
            Color::try_from(&arr[..]),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_slice_correct() {
        let v = vec![183, 65, 14];
        let c: Result<Color, _> = Color::try_from(&v[..]);
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14
            }
        );
    }
    #[test]
    fn test_slice_excess_length() {
        let v = vec![0, 0, 0, 0];
        assert_eq!(Color::try_from(&v[..]), Err(IntoColorError::BadLen));
    }
    #[test]
    fn test_slice_insufficient_length() {
        let v = vec![0, 0];
        assert_eq!(Color::try_from(&v[..]), Err(IntoColorError::BadLen));
    }
}
