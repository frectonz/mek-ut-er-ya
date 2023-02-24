use hifitime::Epoch;

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
        let epoch = Epoch::from_jde_utc(jdn);
        let (year, month, day, _, _, _, _) = epoch.to_gregorian_utc();
        Self {
            year: year as usize,
            month: month as usize,
            day: day as usize,
        }
    }

    pub fn to_jdn(&self) -> f64 {
        Epoch::from_gregorian_utc_at_noon(self.year as i32, self.month as u8, self.day as u8)
            .to_jde_utc_days()
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
            _ => unreachable!(),
        }
    }

    pub fn formatted_year(&self) -> String {
        format!("{:04}", self.year)
    }
}
