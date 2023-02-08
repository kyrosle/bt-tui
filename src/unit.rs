use std::fmt;
use Unit::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Unit {
    Bytes(u64),
    KiB(f64),
    MiB(f64),
    GiB(f64),
}

impl Unit {
    pub fn new(bytes: u64) -> Self {
        let mut r = bytes as f64;
        let mut depth = 0;
        while r > 1024.0 && depth < 3 {
            r /= 1024.0;
            depth += 1;
        }

        match depth {
            0 => Self::Bytes(bytes),
            1 => Self::KiB(r),
            2 => Self::MiB(r),
            3 => Self::GiB(r),
            _ => unreachable!(),
        }
    }
}

impl From<u64> for Unit {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Bytes(b) => write!(f, "{b} B"),
            KiB(b) => write!(f, "{b:.2} KiB"),
            MiB(b) => write!(f, "{b:.2} MiB"),
            GiB(b) => write!(f, "{b:.2} GiB"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_convert_not_convert_bytes() {
        let unit = Unit::new(1011);
        match unit {
            Bytes(u) => {
                assert_eq!(u, 1011);
            }
            _ => {
                panic!("unit should remain in bytes");
            }
        }
    }

    #[test]
    fn should_convert_bytes_to_kib() {
        let unit = Unit::new(5_460);
        assert_eq!("5.33 KiB", unit.to_string());
    }

    #[test]
    fn should_convert_bytes_to_mib() {
        let unit = Unit::new(32_123_962);
        assert_eq!("30.64 MiB", unit.to_string());
    }

    #[test]
    fn should_convert_bytes_to_gib() {
        let unit = Unit::new(2_229_863_925);
        assert_eq!("2.08 GiB", unit.to_string());

        let unit = Unit::new(12262129863925);
        assert_eq!("11420.00 GiB", unit.to_string());
    }
}
