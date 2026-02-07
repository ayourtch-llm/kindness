use solution::*;

#[test]
fn every_minute() {
    let cron = CronExpr::parse("* * * * *").unwrap();
    let after = NaiveDateTime::new(2024, 1, 1, 12, 30);
    let next = cron.next_occurrence(&after);
    assert_eq!(next, NaiveDateTime::new(2024, 1, 1, 12, 31));
}

#[test]
fn specific_time() {
    let cron = CronExpr::parse("30 9 * * *").unwrap();
    let after = NaiveDateTime::new(2024, 6, 15, 9, 30);
    // 9:30 is not strictly after, so next is tomorrow at 9:30
    let next = cron.next_occurrence(&after);
    assert_eq!(next, NaiveDateTime::new(2024, 6, 16, 9, 30));
}

#[test]
fn step_expression() {
    let cron = CronExpr::parse("*/15 * * * *").unwrap();
    let after = NaiveDateTime::new(2024, 3, 1, 10, 14);
    let next = cron.next_occurrence(&after);
    assert_eq!(next, NaiveDateTime::new(2024, 3, 1, 10, 15));
}

#[test]
fn month_rollover() {
    let cron = CronExpr::parse("0 0 1 * *").unwrap(); // midnight on 1st of each month
    let after = NaiveDateTime::new(2024, 1, 15, 0, 0);
    let next = cron.next_occurrence(&after);
    assert_eq!(next, NaiveDateTime::new(2024, 2, 1, 0, 0));
}

#[test]
fn leap_year_feb29() {
    let cron = CronExpr::parse("0 12 29 2 *").unwrap(); // noon on Feb 29
    let after = NaiveDateTime::new(2024, 1, 1, 0, 0);
    let next = cron.next_occurrence(&after);
    assert_eq!(next, NaiveDateTime::new(2024, 2, 29, 12, 0));
}

#[test]
fn day_of_week_filter() {
    let cron = CronExpr::parse("0 9 * * 1").unwrap(); // 9am on Mondays
    let after = NaiveDateTime::new(2024, 7, 1, 0, 0); // Monday July 1 2024
    let next = cron.next_occurrence(&after);
    // July 1 2024 is a Monday, so 9am that day
    assert_eq!(next, NaiveDateTime::new(2024, 7, 1, 9, 0));
}

#[test]
fn range_and_list() {
    let cron = CronExpr::parse("0 9-11 * * *").unwrap(); // 9,10,11 o'clock
    let after = NaiveDateTime::new(2024, 1, 1, 10, 0);
    let next = cron.next_occurrence(&after);
    assert_eq!(next, NaiveDateTime::new(2024, 1, 1, 11, 0));

    let cron2 = CronExpr::parse("0,30 * * * *").unwrap(); // :00 and :30
    let after2 = NaiveDateTime::new(2024, 1, 1, 5, 0);
    let next2 = cron2.next_occurrence(&after2);
    assert_eq!(next2, NaiveDateTime::new(2024, 1, 1, 5, 30));
}
