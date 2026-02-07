#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NaiveDateTime {
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
}

impl NaiveDateTime {
    pub fn new(year: i32, month: u32, day: u32, hour: u32, minute: u32) -> Self {
        Self { year, month, day, hour, minute }
    }

    /// Return day of week: 0=Sunday, 1=Monday, ..., 6=Saturday
    /// Using Tomohiko Sakamoto's algorithm.
    pub fn day_of_week(&self) -> u32 {
        let t = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
        let mut y = self.year;
        if self.month < 3 {
            y -= 1;
        }
        let dow = (y + y / 4 - y / 100 + y / 400 + t[(self.month - 1) as usize] + self.day as i32) % 7;
        ((dow % 7 + 7) % 7) as u32
    }
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => if is_leap_year(year) { 29 } else { 28 },
        _ => panic!("invalid month"),
    }
}

#[derive(Debug, Clone)]
struct CronField {
    values: Vec<u32>,
}

impl CronField {
    fn matches(&self, val: u32) -> bool {
        self.values.contains(&val)
    }
}

fn parse_field(s: &str, min: u32, max: u32) -> Result<CronField, String> {
    let mut values = Vec::new();
    for part in s.split(',') {
        if let Some((range_part, step_str)) = part.split_once('/') {
            let step: u32 = step_str.parse().map_err(|_| format!("invalid step: {}", step_str))?;
            if step == 0 {
                return Err("step cannot be zero".to_string());
            }
            let (start, end) = if range_part == "*" {
                (min, max)
            } else if let Some((a, b)) = range_part.split_once('-') {
                (a.parse().map_err(|_| format!("invalid: {}", a))?,
                 b.parse().map_err(|_| format!("invalid: {}", b))?)
            } else {
                let start: u32 = range_part.parse().map_err(|_| format!("invalid: {}", range_part))?;
                (start, max)
            };
            let mut v = start;
            while v <= end {
                values.push(v);
                v += step;
            }
        } else if part == "*" {
            for v in min..=max {
                values.push(v);
            }
        } else if let Some((a, b)) = part.split_once('-') {
            let start: u32 = a.parse().map_err(|_| format!("invalid: {}", a))?;
            let end: u32 = b.parse().map_err(|_| format!("invalid: {}", b))?;
            for v in start..=end {
                values.push(v);
            }
        } else {
            let v: u32 = part.parse().map_err(|_| format!("invalid: {}", part))?;
            values.push(v);
        }
    }
    values.sort();
    values.dedup();
    Ok(CronField { values })
}

#[derive(Debug, Clone)]
pub struct CronExpr {
    minutes: CronField,
    hours: CronField,
    days_of_month: CronField,
    months: CronField,
    days_of_week: CronField,
}

impl CronExpr {
    pub fn parse(expr: &str) -> Result<CronExpr, String> {
        let parts: Vec<&str> = expr.split_whitespace().collect();
        if parts.len() != 5 {
            return Err("expected 5 fields".to_string());
        }
        let minutes = parse_field(parts[0], 0, 59)?;
        let hours = parse_field(parts[1], 0, 23)?;
        let days_of_month = parse_field(parts[2], 1, 31)?;
        let months = parse_field(parts[3], 1, 12)?;
        let mut days_of_week = parse_field(parts[4], 0, 7)?;
        // Normalize: treat 7 as 0 (both mean Sunday)
        for v in &mut days_of_week.values {
            if *v == 7 {
                *v = 0;
            }
        }
        days_of_week.values.sort();
        days_of_week.values.dedup();
        Ok(CronExpr { minutes, hours, days_of_month, months, days_of_week })
    }

    pub fn next_occurrence(&self, after: &NaiveDateTime) -> NaiveDateTime {
        // Start from the next minute after `after`
        let mut year = after.year;
        let mut month = after.month;
        let mut day = after.day;
        let mut hour = after.hour;
        let mut minute = after.minute + 1;

        // Handle overflow
        if minute > 59 {
            minute = 0;
            hour += 1;
        }
        if hour > 23 {
            hour = 0;
            day += 1;
        }

        let end_year = after.year + 4;

        while year <= end_year {
            if !self.months.matches(month) {
                month += 1;
                if month > 12 {
                    month = 1;
                    year += 1;
                }
                day = 1;
                hour = 0;
                minute = 0;
                continue;
            }

            let max_day = days_in_month(year, month);
            if day > max_day {
                month += 1;
                if month > 12 {
                    month = 1;
                    year += 1;
                }
                day = 1;
                hour = 0;
                minute = 0;
                continue;
            }

            if !self.days_of_month.matches(day) {
                day += 1;
                hour = 0;
                minute = 0;
                continue;
            }

            let dt = NaiveDateTime::new(year, month, day, 0, 0);
            let dow = dt.day_of_week();
            if !self.days_of_week.matches(dow) {
                day += 1;
                hour = 0;
                minute = 0;
                continue;
            }

            if !self.hours.matches(hour) {
                hour += 1;
                if hour > 23 {
                    day += 1;
                    hour = 0;
                }
                minute = 0;
                continue;
            }

            if !self.minutes.matches(minute) {
                minute += 1;
                if minute > 59 {
                    minute = 0;
                    hour += 1;
                    if hour > 23 {
                        hour = 0;
                        day += 1;
                    }
                }
                continue;
            }

            return NaiveDateTime::new(year, month, day, hour, minute);
        }

        panic!("no matching occurrence found within 4 years");
    }
}
