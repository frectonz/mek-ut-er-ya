use clap::{command, Parser};
use colored::Colorize;
use ethiopic_calendar::{EthiopianYear, GregorianYear};
use time::{Month, OffsetDateTime};

/// Simple program for handling Ethiopian dates.
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// The action to perform.
    #[command(subcommand)]
    action: Action,
}

#[derive(clap::Subcommand, Debug)]
enum Action {
    /// Get today's date in the Ethiopian calendar.
    Now,
    /// Get the Ethiopian date for a given Gregorian date. (alias: g2e)
    #[command(alias = "g2e")]
    GregorianToEthiopian {
        year: usize,
        month: usize,
        day: usize,
    },
    /// Get the Gregorian date for a given Ethiopian date. (alias: e2g)
    #[command(alias = "e2g")]
    EthiopianToGregorian {
        year: usize,
        month: usize,
        day: usize,
    },
    /// Display the current month in the Ethiopian calendar. (alias: cal)
    #[command(alias = "cal")]
    Calendar,
    /// Display the current year in the Ethiopian calendar.
    Year,
}

fn main() {
    let args = Args::parse();

    match args.action {
        Action::Now => do_now(),
        Action::GregorianToEthiopian { year, month, day } => {
            do_gregorian_to_ethiopian(year, month, day)
        }
        Action::EthiopianToGregorian { year, month, day } => {
            do_ethiopian_to_gregorian(year, month, day)
        }
        Action::Calendar => do_calendar(),
        Action::Year => do_year(),
    };
}

fn get_now() -> EthiopianYear {
    let now = OffsetDateTime::now_utc();
    let gregorian = GregorianYear::new(
        now.year() as usize,
        W(now.month()).into(),
        now.day() as usize,
    );
    gregorian.into()
}

fn format_month(month: usize, year: usize, highlight_day: Option<usize>) -> Vec<String> {
    let first_day = EthiopianYear::new(year, month, 1);
    let month = first_day.amharic_month();
    let year = first_day.formatted_year();
    let month_title = format!("{} {}", month, year);

    let mut list = Vec::with_capacity(7);

    list.push(format!("{:^20}", month_title.green().bold()));
    let header = "Su Mo Tu We Th Fr Sa".to_string();
    list.push(header.green().bold().to_string());

    let mut line: String;
    let mut day = 1;
    let mut weekday = first_day.weekday();
    while day <= first_day.days_in_month() {
        line = String::new();
        while weekday < 7 && day <= first_day.days_in_month() {
            if day == 1 {
                line.push_str(" ".repeat(weekday * 3).as_str());
            }

            if day == highlight_day.unwrap_or(31) {
                line.push_str(&format!("{:2} ", day.to_string().white().bold().on_black()));
            } else {
                line.push_str(&format!("{:2} ", day));
            }

            day += 1;
            weekday += 1;
        }
        weekday = 0;
        let padding = header.len().checked_sub(line.len()).unwrap_or(0);
        line.push_str(&" ".repeat(padding));
        list.push(line);
    }

    list
}

fn do_now() {
    let ethiopian = get_now();
    let month = ethiopian.amharic_month();
    let day = ethiopian.amharic_weekday();

    let output = format!(
        "{} ፣ {} {} ቀን {} ዓ/ም",
        day,
        month,
        ethiopian.day(),
        ethiopian.year()
    );
    println!("{}", output.green());
}

fn do_gregorian_to_ethiopian(y: usize, m: usize, d: usize) {
    if m > 12 {
        println!("{}", "Invalid month.".red());
        return;
    }

    if d > 31 {
        println!("{}", "Invalid day.".red());
        return;
    }

    let gregorian = GregorianYear::new(y, m, d);
    let ethiopian: EthiopianYear = gregorian.into();

    let weekday = ethiopian.amharic_weekday();
    let month = ethiopian.amharic_month();
    let day = ethiopian.day();
    let year = ethiopian.formatted_year();

    let output = format!("{weekday} ፣ {month} {day} ቀን {year} ዓ/ም");
    println!("{}", output.green().bold());
}

fn do_ethiopian_to_gregorian(y: usize, m: usize, d: usize) {
    if m > 13 {
        println!("{}", "Invalid month.".red());
        return;
    }

    if d > 30 {
        println!("{}", "Invalid day.".red());
        return;
    }

    let ethiopian = EthiopianYear::new(y, m, d);
    let gregorian: GregorianYear = ethiopian.into();

    let weekday = gregorian.english_weekday();
    let month = gregorian.english_month();
    let day = gregorian.day();
    let year = gregorian.formatted_year();

    let output = format!("{weekday}, {month} {day}, {year}");
    println!("{}", output.green());
}

fn do_calendar() {
    let ethiopian: EthiopianYear = get_now();
    let lines = format_month(ethiopian.month(), ethiopian.year(), Some(ethiopian.day()));
    lines.into_iter().for_each(|l| println!("{}", l.green()));
}

fn do_year() {
    let ethiopian = get_now();
    let mut group = Vec::with_capacity(3);
    let mut max_number_of_lines = 0;
    for month in 1..=13 {
        let highlight_day = if month == ethiopian.month() {
            Some(ethiopian.day())
        } else {
            None
        };
        let lines = format_month(month, ethiopian.year(), highlight_day);
        max_number_of_lines = lines.len().max(max_number_of_lines);
        group.push(lines);
        if month % 3 == 0 || month == 13 {
            let mut lines = Vec::with_capacity(7);
            for i in 0..=max_number_of_lines - 1 {
                let mut line = String::new();
                for e in &group {
                    let padding = " ".repeat(20);
                    let e = e.get(i).map(|a| a.as_str()).unwrap_or(&padding);
                    line.push_str(e);
                    line.push_str("\t");
                }
                lines.push(line);
            }
            lines.into_iter().for_each(|l| println!("{}", l.green()));
            group.clear();
            println!();
            max_number_of_lines = 0;
        }
    }
}

struct W<T>(T);
impl From<W<Month>> for usize {
    fn from(month: W<Month>) -> Self {
        match month.0 {
            Month::January => 1,
            Month::February => 2,
            Month::March => 3,
            Month::April => 4,
            Month::May => 5,
            Month::June => 6,
            Month::July => 7,
            Month::August => 8,
            Month::September => 9,
            Month::October => 10,
            Month::November => 11,
            Month::December => 12,
        }
    }
}
