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
    };
}

fn do_now() {
    let now = OffsetDateTime::now_utc();
    let gregorian = GregorianYear::new(
        now.year() as usize,
        W(now.month()).into(),
        now.day() as usize,
    );
    let ethiopian: EthiopianYear = gregorian.into();
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
    let now = OffsetDateTime::now_utc();
    let gregorian = GregorianYear::new(
        now.year() as usize,
        W(now.month()).into(),
        now.day() as usize,
    );
    let ethiopian: EthiopianYear = gregorian.into();

    let first_day = EthiopianYear::new(ethiopian.year(), ethiopian.month(), 1);

    let month = ethiopian.amharic_month();
    let year = ethiopian.formatted_year();
    let month_title = format!("{} {}", month, year);

    let month = format!("\n{:^20}\n", month_title.green().bold());
    println!("{}", month);

    println!("Su Mo Tu We Th Fr Sa");
    let mut line: String;
    let mut day = 1;
    let mut weekday = first_day.weekday();
    while day <= 30 {
        line = String::new();
        while weekday < 7 && day <= 30 {
            if day == 1 {
                line.push_str(" ".repeat(weekday * 3).as_str());
            }

            if day == ethiopian.day() {
                line.push_str(&format!("{:2} ", day.to_string().bold().black().on_white()));
            } else {
                line.push_str(&format!("{:2} ", day));
            }

            day += 1;
            weekday += 1;
        }
        weekday = 0;
        println!("{}", line);
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
