/// Create enums for `str` literals. Useful for parsing puzzle inputs.
///
/// ```
/// # #[macro_use]
/// # extern crate aoc2020;
///
/// str_enum! {
///     Colour {
///         (Red, "Red"),
///         (Green, "Green"),
///         (Blue, "Blue"),
///     }
/// }
///
/// # fn main() {
/// assert_eq!("Red".parse::<Colour>(), Ok(Colour::Red));
/// assert_eq!("what".parse::<Colour>(), Err("Was not Colour"));
/// # }
/// ```
#[macro_export]
macro_rules! str_enum {
    ($name:ident { $(($item:ident, $repr:expr),)* }) => {
        #[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
        pub enum $name {
        $(
            $item,
        )*
        }

        impl std::fmt::Display for $name {
           fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", match self {
                $(
                  &$name::$item => $repr,
                )*
                })
            }
        }

        impl std::str::FromStr for $name {
            type Err = &'static str;
            fn from_str(s: &str) -> Result<$name, Self::Err> {
                match s {
                $(
                  $repr => Ok($name::$item),
                )*
                 _ => Err(concat!("Was not ", stringify!($name))),
                }
            }
        }
    }
}

/// Create enums for `char`s. Useful for parsing puzzle inputs.
///
/// ```
/// # #[macro_use]
/// # extern crate aoc2020;
///
/// char_enum! {
///     Tile {
///         (Wall, '#'),
///         (Path, '.'),
///         (Lava, 'L'),
///     }
/// }
///
/// # fn main() {
/// assert_eq!(Tile::new('#'), Ok(Tile::Wall));
/// assert_eq!(Tile::new(','), Err("Was not Tile"));
/// # }
/// ```
#[macro_export]
macro_rules! char_enum {
    ($name:ident { $(($item:ident, $repr:expr),)* }) => {
        #[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Hash, Copy, Clone)]
        pub enum $name {
        $(
            $item,
        )*
        }

        impl $name {
            pub fn new(c: char) -> Result<$name, &'static str>  {
                match c {
                $(
                  $repr => Ok($name::$item),
                )*
                 _ => Err(concat!("Was not ", stringify!($name))),
                }
            }
        }

        impl std::fmt::Display for $name {
           fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", match self {
                $(
                  &$name::$item => $repr,
                )*
                })
            }
        }
    }
}

/// Conditionally compiled `std::eprintln` macro. Included iff `debug_assertions` (off by
/// default in release mode) is present.
#[macro_export]
macro_rules! dbg_eprintln {
    ($($rest:tt)*) => {
        #[cfg(debug_assertions)]
        std::eprintln!($($rest)*)
    }
}
