// enum Ordering {
//     Less,
//     Equal,
//     Greater
// }

// use std::cmp::Ordering;

// fn compare(n:i32, m:i32) {
//     if n < m {
//         Ordering::Less
//     } else if n > m {
//         Ordering::Greater
//     } else {
//         Ordering::Equal
//     }
// }

// with all constructors

use std::cmp::Ordering::{self, *}; // `*` to import all children

fn compare(n: i32, m:i32) -> Ordering{
    if n < m {
        Less
    } else if n > m {
        Greater
    } else {
        Equal
    }
}

enum Pet {
    Orca,
    Giraffe,
    //...
}

use self::Pet as PPet; // using enum in same module

enum HttpStatus {
    Ok = 200,
    NotModified = 304,
    NotFound = 404,
    // ...
}

fn http_status_from_u32(n: u32) -> Option<HttpStatus> {
    match n {
        200 => Some(HttpStatus::Ok),
        304 => Some(HttpStatus::NotModified),
        404 => Some(HttpStatus::NotFound),
        // ...
        _ => None
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
    Months,
    Years,
}

impl TimeUnit {
    /// Return the plural noun of this time unit
    pub fn plural(self) -> &'static str {
        match self {
            TimeUnit::Seconds => "seconds",
            TimeUnit::Minutes => "minutes",
            TimeUnit::Hours => "hours",
            TimeUnit::Days => "days",
            TimeUnit::Months => "months",
            TimeUnit::Years => "years",
        }
    }

    /// Return the singular noun for this time unit
    pub fn singular(self) -> &'static str {
        self.plural().trim_end_matches('s')
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_enums_vlues() {
        use std::mem::size_of;

        assert_eq!(size_of::<Ordering>(), 1); // fit in a u8 i.e. single byte
        assert_eq!(size_of::<HttpStatus>(), 2); // doesn't fit in a u8

        assert_eq!(HttpStatus::Ok as i32, 200);
    }
}