use chrono::{DateTime, Datelike, Duration, Local, NaiveDateTime, NaiveTime, TimeZone, Utc};
use chrono_tz::Tz;

/// Returns the start of the current week as a `DateTime<Local>`.
///
/// This function calculates the start of the week based on the current local
/// time. The week starts on Monday.
///
/// # Examples
///
/// ```
/// let start_of_week = get_start_of_the_week();
/// println!("Start of the week: {}", start_of_week);
/// ```
pub fn get_start_of_the_week() -> DateTime<Local> {
    let now = Local::now();
    let days_to_subtract = now.weekday().num_days_from_monday() as i64;
    let start_of_the_week = now - Duration::days(days_to_subtract);
    start_of_the_week
}

/// Returns a vector of strings containing the names of the days of the week in English.
///
/// This function generates a vector where each element is the name of a day of the week,
/// starting from Monday to Sunday.
///
/// # Examples
///
/// ```
/// let days = days_in_english();
/// assert_eq!(days, vec![
///     "Monday".to_string(),
///     "Tuesday".to_string(),
///     "Wednesday".to_string(),
///     "Thursday".to_string(),
///     "Friday".to_string(),
///     "Saturday".to_string(),
///     "Sunday".to_string()
/// ]);
/// ```
pub fn days_in_english() -> [&'static str; 7] {
    let days = [
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
        "Sunday",
    ];

    days
}

/// Converts a date string to a `DateTime<Utc>` based on the provided timezone.
///
/// This function accepts a date string that can be either in the format `HH:MM`,
/// `YYYY-MM-DD HH:MM`, or `MM-DD HH:MM`. It will parse the string and convert it to a
/// `DateTime<Utc>` considering the given timezone. If the string is in the `HH:MM` format,
/// it will use the current date combined with the provided time. If the string is in the
/// `MM-DD HH:MM` format, it will use the current year combined with the provided month, day, and time.
///
/// # Arguments
///
/// * `tz` - A timezone from the `chrono_tz` crate.
/// * `date` - A string slice that holds the date to be parsed.
///
/// # Returns
///
/// A `DateTime<Utc>` representing the parsed date and time in UTC.
///
/// # Examples
///
/// ```
/// use chrono_tz::Tz;
///
/// let tz: Tz = "America/New_York".parse().unwrap();
/// let date_str = String::from("2024-07-27 15:30");
/// let utc_date = get_date_from_string(tz, &date_str);
/// println!("{}", utc_date); // Outputs the parsed date and time in UTC
///
/// let time_str = String::from("15:30");
/// let utc_time = get_date_from_string(tz, &time_str);
/// println!("{}", utc_time); // Outputs the current date with the provided time in UTC
///
/// let month_day_time_str = String::from("07-27 15:30");
/// let utc_month_day_time = get_date_from_string(tz, &month_day_time_str);
/// println!("{}", utc_month_day_time); // Outputs the current year with the provided month, day, and time in UTC
/// ```
/// Parse a list-range input string into a `DateTime<Utc>` at start-of-day in `tz`.
///
/// Accepts:
/// - `today`, `tomorrow`, `yesterday`
/// - `+Nd` / `+Nw` / `-Nd` / `-Nw` (relative offsets, days/weeks)
/// - `YYYY-MM-DD` (calendar date, midnight in `tz`)
/// - RFC3339 (`2026-05-04T12:00:00Z` or `+03:00`)
/// - weekday name (`monday`, `tue`, …) — next occurrence on or after today
pub fn parse_range_input(tz: Tz, input: &str) -> Result<DateTime<Utc>, String> {
    let s = input.trim().to_lowercase();
    let now_local = Local::now().with_timezone(&tz);

    if s == "today" {
        return Ok(start_of_day_utc(tz, now_local));
    }
    if s == "tomorrow" {
        return Ok(start_of_day_utc(tz, now_local + Duration::days(1)));
    }
    if s == "yesterday" {
        return Ok(start_of_day_utc(tz, now_local - Duration::days(1)));
    }

    // Relative ±Nd / ±Nw
    if let Some(rest) = s.strip_prefix('+').or_else(|| s.strip_prefix('-')) {
        let sign: i64 = if s.starts_with('-') { -1 } else { 1 };
        if let Some(n_str) = rest.strip_suffix('d') {
            let n: i64 = n_str
                .parse()
                .map_err(|e| format!("bad days '{}': {}", n_str, e))?;
            return Ok(start_of_day_utc(tz, now_local + Duration::days(sign * n)));
        }
        if let Some(n_str) = rest.strip_suffix('w') {
            let n: i64 = n_str
                .parse()
                .map_err(|e| format!("bad weeks '{}': {}", n_str, e))?;
            return Ok(start_of_day_utc(tz, now_local + Duration::weeks(sign * n)));
        }
    }

    // Weekday name (next occurrence on or after today)
    let weekday_idx = match s.as_str() {
        "mon" | "monday" => Some(0u32),
        "tue" | "tuesday" => Some(1),
        "wed" | "wednesday" => Some(2),
        "thu" | "thursday" => Some(3),
        "fri" | "friday" => Some(4),
        "sat" | "saturday" => Some(5),
        "sun" | "sunday" => Some(6),
        _ => None,
    };
    if let Some(target_idx) = weekday_idx {
        let today_idx = now_local.weekday().num_days_from_monday();
        let delta = (target_idx + 7 - today_idx) % 7;
        return Ok(start_of_day_utc(
            tz,
            now_local + Duration::days(delta as i64),
        ));
    }

    // RFC3339
    if let Ok(dt) = DateTime::parse_from_rfc3339(input) {
        return Ok(dt.with_timezone(&Utc));
    }

    // YYYY-MM-DD
    if let Ok(d) = chrono::NaiveDate::parse_from_str(input, "%Y-%m-%d") {
        let nd = d
            .and_hms_opt(0, 0, 0)
            .ok_or_else(|| "bad time".to_string())?;
        let local = tz
            .from_local_datetime(&nd)
            .single()
            .ok_or_else(|| format!("ambiguous local datetime for {}", input))?;
        return Ok(local.with_timezone(&Utc));
    }

    Err(format!(
        "unrecognized date input '{}'. Try: today, tomorrow, +7d, +2w, monday, 2026-05-04, or RFC3339.",
        input
    ))
}

fn start_of_day_utc<TZ: TimeZone>(tz: Tz, dt: DateTime<TZ>) -> DateTime<Utc> {
    let naive = dt.naive_local().date().and_hms_opt(0, 0, 0).unwrap();
    tz.from_local_datetime(&naive)
        .single()
        .unwrap_or_else(|| tz.from_local_datetime(&naive).earliest().unwrap())
        .with_timezone(&Utc)
}

pub fn get_date_from_string(tz: Tz, date: &String) -> DateTime<Utc> {
    if let Ok(parsed_time) = NaiveTime::parse_from_str(date, "%H:%M") {
        let current_date = Utc::now().date_naive();
        let combined_naive = NaiveDateTime::new(current_date, parsed_time);
        let event_date_with_timezone = tz
            .from_local_datetime(&combined_naive)
            .unwrap()
            .naive_utc()
            .and_utc();
        return event_date_with_timezone;
    } else if let Ok(parsed_time) = NaiveDateTime::parse_from_str(date, "%Y-%m-%d %H:%M") {
        let event_date_with_timezone = tz
            .from_local_datetime(&parsed_time)
            .unwrap()
            .naive_utc()
            .and_utc();
        return event_date_with_timezone;
    } else {
        let parsed_time = NaiveDateTime::parse_from_str(
            &format!("{}-{}", Utc::now().year(), date),
            "%Y-%m-%d %H:%M",
        );
        print!("{}", &format!("{}-{}", Utc::now().year(), date));
        let combined_naive = parsed_time.unwrap();
        let event_date_with_timezone = tz
            .from_local_datetime(&combined_naive)
            .unwrap()
            .naive_utc()
            .and_utc();
        return event_date_with_timezone;
    }
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Weekday};

    use super::*;

    #[test]
    fn test_extracting_full_date_from_string() -> Result<(), String> {
        let tz: Tz = "America/New_York".parse().unwrap(); // UTC is 4 hours ahead of New York
        let date = String::from("2024-07-27 15:30");

        let actual_date = get_date_from_string(tz, &date);
        let expected_date = Utc.with_ymd_and_hms(2024, 07, 27, 19, 30, 0).unwrap();

        assert_eq!(actual_date, expected_date);
        Ok(())
    }

    #[test]
    fn test_extracting_full_date_except_year_from_string() -> Result<(), String> {
        let tz: Tz = "Europe/Zurich".parse().unwrap(); // UTC is 2 hours behind Zurich (CEST)
        let date = String::from("06-11 0:30");
        let current_year = Utc::now().year();

        let actual_date = get_date_from_string(tz, &date);
        let expected_date = Utc
            .with_ymd_and_hms(current_year, 06, 10, 22, 30, 0)
            .unwrap();

        assert_eq!(actual_date, expected_date);
        Ok(())
    }

    #[test]
    fn test_extracting_only_hours_and_minutes_from_string() -> Result<(), String> {
        let tz: Tz = "Asia/Tokyo".parse().unwrap(); // UTC is 9 hours behind Tokyo
        let date = String::from("23:12");
        let now = Utc::now();

        let actual_date = get_date_from_string(tz, &date);
        let expected_date = Utc
            .with_ymd_and_hms(now.year(), now.month(), now.day(), 14, 12, 0)
            .unwrap();

        assert_eq!(actual_date, expected_date);
        Ok(())
    }

    #[test]
    fn test_get_start_of_the_week() -> Result<(), String> {
        let start_of_the_week = get_start_of_the_week();
        // Capture `now` after the function so the function's internal
        // `Local::now()` cannot be after our anchor (which would race
        // when today is Monday and the diff equals zero).
        let now = Local::now();

        let days_difference = now.signed_duration_since(start_of_the_week).num_days();
        assert!(start_of_the_week <= now);
        assert!((0..=6).contains(&days_difference));
        assert_eq!(start_of_the_week.weekday(), Weekday::Mon);
        Ok(())
    }

    fn fixture_tz() -> Tz {
        "UTC".parse().unwrap()
    }

    #[test]
    fn parse_range_input_today() {
        let dt = parse_range_input(fixture_tz(), "today").unwrap();
        assert_eq!(dt.format("%H:%M:%S").to_string(), "00:00:00");
    }

    #[test]
    fn parse_range_input_relative_days() {
        let today = parse_range_input(fixture_tz(), "today").unwrap();
        let plus3 = parse_range_input(fixture_tz(), "+3d").unwrap();
        let diff = (plus3 - today).num_days();
        assert_eq!(diff, 3);
    }

    #[test]
    fn parse_range_input_relative_weeks() {
        let today = parse_range_input(fixture_tz(), "today").unwrap();
        let plus2w = parse_range_input(fixture_tz(), "+2w").unwrap();
        let diff = (plus2w - today).num_days();
        assert_eq!(diff, 14);
    }

    #[test]
    fn parse_range_input_yesterday() {
        let today = parse_range_input(fixture_tz(), "today").unwrap();
        let yest = parse_range_input(fixture_tz(), "yesterday").unwrap();
        let diff = (today - yest).num_days();
        assert_eq!(diff, 1);
    }

    #[test]
    fn parse_range_input_iso_date() {
        let dt = parse_range_input(fixture_tz(), "2026-05-04").unwrap();
        assert_eq!(
            dt.format("%Y-%m-%dT%H:%M:%S").to_string(),
            "2026-05-04T00:00:00"
        );
    }

    #[test]
    fn parse_range_input_rfc3339() {
        let dt = parse_range_input(fixture_tz(), "2026-05-04T12:34:56Z").unwrap();
        assert_eq!(dt.to_rfc3339(), "2026-05-04T12:34:56+00:00");
    }

    #[test]
    fn parse_range_input_weekday_returns_monday() {
        let dt = parse_range_input(fixture_tz(), "monday").unwrap();
        assert_eq!(dt.weekday(), Weekday::Mon);
    }

    #[test]
    fn parse_range_input_rejects_garbage() {
        let err = parse_range_input(fixture_tz(), "not-a-date").unwrap_err();
        assert!(err.contains("unrecognized"));
    }
}
