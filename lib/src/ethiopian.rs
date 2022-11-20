#[derive(Debug, PartialEq, Eq)]
pub struct EthiopianYear {
    year: usize,
    month: usize,
    day: usize,
}

const JDN_EPOCH_OFFSET_AMETE_MIHRET: f64 = 1723856.;

impl EthiopianYear {
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
        let r = mod_op(jdn - JDN_EPOCH_OFFSET_AMETE_MIHRET, 1461.);
        let n = mod_op(r, 365.) + 365. * quotient(r, 1460.);

        let year = (4. * quotient(jdn - JDN_EPOCH_OFFSET_AMETE_MIHRET, 1461.) + quotient(r, 365.)
            - quotient(r, 1460.))
        .floor() as usize;
        let month = quotient(n, 30.) as usize + 1;
        let day = mod_op(n, 30.) as usize + 1;

        Self { year, month, day }
    }

    pub fn to_jdn(&self) -> f64 {
        (JDN_EPOCH_OFFSET_AMETE_MIHRET + 365.)
            + (365 * (self.year - 1)) as f64
            + quotient(self.year as f64, 4.)
            + (30 * self.month) as f64
            + (self.day as f64 - 31.)
    }

    pub fn amharic_month(&self) -> &'static str {
        match self.month {
            1 => "መስከረም",
            2 => "ጥቅምት",
            3 => "ኅዳር",
            4 => "ታኅሣሥ",
            5 => "ጥር",
            6 => "የካቲት",
            7 => "መጋቢት",
            8 => "ሚያዝያ",
            9 => "ግንቦት",
            10 => "ሰኔ",
            11 => "ሐምሌ",
            12 => "ነሐሴ",
            13 => "ጳጉሜ",
            _ => unreachable!(),
        }
    }

    pub fn weekday(&self) -> usize {
        (self.to_jdn() as usize + 1) % 7
    }

    pub fn amharic_weekday(&self) -> &'static str {
        match self.weekday() {
            0 => "እሁድ",
            1 => "ሰኞ",
            2 => "ማክሰኞ",
            3 => "ረቡዕ",
            4 => "ሐሙስ",
            5 => "ዓርብ",
            6 => "ቅዳሜ",
            _ => unreachable!(),
        }
    }

    pub fn formatted_year(&self) -> String {
        format!("{:04}", self.year)
    }
}

fn quotient(a: f64, b: f64) -> f64 {
    (a / b).floor()
}

fn mod_op(a: f64, b: f64) -> f64 {
    a - b * quotient(a, b)
}

#[cfg(test)]
mod tests {
    use super::EthiopianYear;

    #[test]
    fn test_from_jdn() {
        let year = EthiopianYear::from_jdn(2401443.5);
        assert_eq!(
            year,
            EthiopianYear {
                year: 1855,
                month: 2,
                day: 20
            }
        );
    }

    #[test]
    fn test_to_jdn() {
        let year = EthiopianYear {
            year: 1855,
            month: 2,
            day: 20,
        };
        assert_eq!(year.to_jdn(), 2401443.);
    }

    #[test]
    fn tests() {
        // Tests taken from https://www.geez.org/Calendars/EthiopicCalendarTest.java
        let tests = [
            (1724221., 1, 1, 1),
            (1724586., 2, 1, 1),
            (1724951., 3, 1, 1),
            (1724585., 1, 13, 5),
            (1724950., 2, 13, 5),
            (1725315., 3, 13, 5),
            (1725316., 3, 13, 6),
            (2299159., 1575, 2, 6),
            (2299160., 1575, 2, 7),
            (2299161., 1575, 2, 8),
            (2299162., 1575, 2, 9),
            (2401443., 1855, 2, 20),
            (2402423., 1857, 10, 29),
            (2402631., 1858, 5, 22),
            (2402709., 1858, 8, 10),
            (2402972., 1859, 4, 28),
            (2403345., 1860, 5, 5),
            (2415021., 1892, 4, 23),
            (2453372., 1997, 4, 23),
            (2454720., 2000, 13, 5),
            (2415385., 1893, 4, 22),
            (2448988., 1985, 4, 22),
            (2450449., 1989, 4, 22),
            (2451910., 1993, 4, 22),
            (2453371., 1997, 4, 22),
            (2817152., 2993, 4, 14),
            (3182395., 3993, 4, 7),
            (3912880., 5993, 3, 22),
        ];

        for test in tests.iter() {
            let year = EthiopianYear::from_jdn(test.0);
            assert_eq!(
                year,
                EthiopianYear {
                    year: test.1,
                    month: test.2,
                    day: test.3,
                }
            );
        }
    }
}
