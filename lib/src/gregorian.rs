#[derive(Debug, PartialEq, Eq)]
pub struct GregorianYear {
    year: usize,
    month: usize,
    day: usize,
}

impl GregorianYear {
    pub fn new(year: usize, month: usize, day: usize) -> Self {
        Self { year, month, day }
    }

    pub fn year(&self) -> usize {
        self.year
    }

    pub fn month(&self) -> usize {
        self.month
    }

    pub fn day(&self) -> usize {
        self.day
    }

    pub fn from_jdn(jdn: f64) -> Self {
        let z = jdn.floor();
        let w = ((z - 1867216.25).floor() / 36524.25).floor();
        let x = (w / 4.0).floor();
        let a = (z + 1.0 + w - x).floor();
        let b = (a + 1524.0).floor();
        let c = ((b - 122.1) / 365.25).floor();
        let d = (365.25 * c).floor();
        let e = ((b - d) / 30.6001).floor();
        let f = (30.6001 * e).floor();

        let day = (b - d - f).floor() as usize;
        let month = if e < 14.0 {
            (e - 1.0).floor() as usize
        } else {
            (e - 13.0).floor() as usize
        };
        let year = if month > 2 {
            (c - 4716.0).floor() as usize
        } else {
            (c - 4715.0).floor() as usize
        };

        Self { year, month, day }
    }

    pub fn to_jdn(&self) -> f64 {
        let a = (self.year / 100) as isize;
        let b = a / 4;
        let c = 2 - a + b;
        let e = (365.25 * (self.year + 4716) as f64).floor();
        let f = (30.6001 * (self.month + 1) as f64).floor();
        let jd = c as f64 + self.day as f64 + e + f - 1524.5;
        jd + 0.5
    }

    pub fn weekday(&self) -> usize {
        (self.to_jdn() as usize + 1) % 7
    }

    pub fn english_weekday(&self) -> &'static str {
        match self.weekday() {
            0 => "Sunday",
            1 => "Monday",
            2 => "Tuesday",
            3 => "Wednesday",
            4 => "Thursday",
            5 => "Friday",
            6 => "Saturday",
            _ => unreachable!(),
        }
    }

    pub fn english_month(&self) -> &'static str {
        match self.month {
            1 => "January",
            2 => "February",
            3 => "March",
            4 => "April",
            5 => "May",
            6 => "June",
            7 => "July",
            8 => "August",
            9 => "September",
            10 => "October",
            11 => "November",
            12 => "December",
            _ => unimplemented!(),
        }
    }

    pub fn formatted_year(&self) -> String {
        format!("{:04}", self.year)
    }
}

#[cfg(test)]
mod tests {
    use super::GregorianYear;

    #[test]
    fn test_from_jdn() {
        let year = GregorianYear::from_jdn(2299161.);
        assert_eq!(
            year,
            GregorianYear {
                year: 1582,
                month: 10,
                day: 15
            }
        );
    }

    #[test]
    fn test_to_jdn() {
        let year = GregorianYear {
            year: 1582,
            month: 10,
            day: 15,
        };
        assert_eq!(year.to_jdn(), 2299161.);
    }

    #[test]
    fn tests() {
        // Tests taken from https://www.geez.org/Calendars/EthiopicCalendarTest.java
        let tests = [
            (1724221., 8, 8, 27),
            (1724586., 9, 8, 27),
            (1724951., 10, 8, 27),
            (1724585., 9, 8, 26),
            (1724950., 10, 8, 26),
            (1725315., 11, 8, 26),
            (1725316., 11, 8, 27),
            (2299159., 1582, 10, 13),
            (2299160., 1582, 10, 14),
            (2299161., 1582, 10, 15),
            (2299162., 1582, 10, 16),
            (2401443., 1862, 10, 29),
            (2402423., 1865, 7, 5),
            (2402631., 1866, 1, 29),
            (2402709., 1866, 4, 17),
            (2402972., 1867, 1, 5),
            (2403345., 1868, 1, 13),
            (2415021., 1900, 1, 1),
            (2453372., 2005, 1, 1),
            (2454720., 2008, 9, 10),
            (2415385., 1900, 12, 31),
            (2448988., 1992, 12, 31),
            (2450449., 1996, 12, 31),
            (2451910., 2000, 12, 31),
            (2453371., 2004, 12, 31),
            (2817152., 3000, 12, 31),
            (3182395., 4000, 12, 31),
            (3912880., 6000, 12, 31),
        ];

        for test in tests {
            let year = GregorianYear::from_jdn(test.0);
            assert_eq!(
                year,
                GregorianYear {
                    year: test.1,
                    month: test.2,
                    day: test.3
                }
            );
        }
    }
}
