pub fn time_info(time: &str) -> String {
  let (year, month, day) = parse_date(time);
  let date = Date::new(year, month, day);

  let nweek = nweek_since_year(&date);
  let weekday = weekday(&date);
  let nday_since_year = date.nday_since_year();
  let nday_rem_year = date.nday_rem_year();
  let nday_to_cny = nday_to_cny_2025(&date);
  let nday_to_a_share = nday_to_a_share(&date);

  format!("{nweek},{weekday},{nday_since_year},{nday_rem_year},{nday_to_cny},{nday_to_a_share}")
}

const fn is_leap_year(year: i32) -> bool {
  (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

const fn nday_month(year: i32, month: i32) -> i32 {
  const MONTH_DAYS: &[i32] = &[31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
  let month = month as usize;

  match month {
    2 if is_leap_year(year) => 29,
    m @ 1..=12 => MONTH_DAYS[m - 1],
    _ => panic!("无效月份"),
  }
}

struct Date {
  year: i32,
  month: i32,
  day: i32,
}

impl Date {
  const fn new(year: i32, month: i32, day: i32) -> Self {
    Self { year, month, day }
  }

  fn nday_since_year(&self) -> i32 {
    (1..self.month).map(|m| nday_month(self.year, m)).sum::<i32>() + self.day
  }

  fn nday_rem_year(&self) -> i32 {
    let total_days = if is_leap_year(self.year) { 366 } else { 365 };
    total_days - self.nday_since_year()
  }
}

const fn weekday(date: &Date) -> i32 {
  let (mut y, mut m) = (date.year, date.month);
  if m < 3 {
    m += 12;
    y -= 1;
  }

  let c = y / 100;
  let y = y % 100;

  let w = (y + y / 4 + c / 4 - 2 * c + 26 * (m + 1) / 10 + date.day - 1) % 7;
  match w {
    w if w < 0 => w + 7,
    0 => 7,
    _ => w,
  }
}

fn nweek_since_year(date: &Date) -> i32 {
  let (first_monday_year, first_monday_month, first_monday_day) = first_monday(date.year);
  let day_diff = diff_nday(
    date.year,
    date.month,
    date.day,
    first_monday_year,
    first_monday_month,
    first_monday_day,
  );
  let week_diff = day_diff / 7;

  if date.month == 12 && (29..=31).contains(&date.day) {
    let (_, next_monday_month, next_monday_day) = first_monday(date.year + 1);
    if next_monday_month == 12 && next_monday_day <= date.day {
      return 1;
    }
  }

  if date.month == 1
    && (1..=4).contains(&date.day)
    && first_monday_month == 1
    && first_monday_day > date.day
  {
    let (last_monday_year, last_monday_month, last_monday_day) = first_monday(date.year - 1);
    diff_nday(
      date.year,
      date.month,
      date.day,
      last_monday_year,
      last_monday_month,
      last_monday_day,
    );
  }

  1 + week_diff
}

fn nday_since_epoch(year: i32, month: i32, day: i32) -> i32 {
  let mut acc = 0;
  for y in 1..year {
    acc += if is_leap_year(y) { 366 } else { 365 };
  }

  for m in 1..month {
    acc += nday_month(year, m);
  }
  acc + day
}

fn diff_nday(y1: i32, m1: i32, d1: i32, y2: i32, m2: i32, d2: i32) -> i32 {
  (nday_since_epoch(y2, m2, d2) - nday_since_epoch(y1, m1, d1)).abs()
}

fn first_monday(year: i32) -> (i32, i32, i32) {
  let mut first_thursday_day = 1;
  for i in 0..6 {
    if weekday(&Date::new(year, 1, 1 + i)) == 4 {
      first_thursday_day = 1 + i;
      break;
    }
  }

  if first_thursday_day <= 3 {
    (year - 1, 12, 31 - (3 - first_thursday_day))
  } else {
    (year, 1, first_thursday_day - 3)
  }
}

fn nday_to_cny_2025(date: &Date) -> i32 {
  let nday_cur = date.nday_since_year();
  let nday_cny = Date::new(date.year, 1, 29).nday_since_year();

  if nday_cur < nday_cny {
    nday_cny - nday_cur
  } else {
    let year_next = date.year + 1;
    let nday_cny_next = Date::new(year_next, 2, 17).nday_since_year();
    let nday_year = if is_leap_year(date.year) { 366 } else { 365 };
    nday_year - nday_cur + nday_cny_next
  }
}

fn nday_to_a_share(date: &Date) -> i32 {
  let cny = Date::new(date.year, 1, 2).nday_since_year();
  let spring = Date::new(date.year, 2, 5).nday_since_year();
  let qingming = Date::new(date.year, 4, 7).nday_since_year();
  let labor = Date::new(date.year, 5, 6).nday_since_year();
  let dbf = Date::new(date.year, 6, 3).nday_since_year();
  let autumn = Date::new(date.year, 10, 9).nday_since_year();
  let cny_next = Date::new(date.year + 1, 1, 1).nday_since_year();

  let nday = date.nday_since_year();
  let weekday = weekday(date);

  match (date.month, date.day) {
    (1, 1) => cny - nday - 1,
    (1, 28..=31) | (2, 1..=4) => spring - nday - 1,
    (4, 4..=6) => qingming - nday - 1,
    (5, 1..=5) => labor - nday - 1,
    (5, 31) | (6, 1..=2) => dbf - nday - 1,
    (10, 1..=8) => autumn - nday - 1,
    (12, 31) => {
      let days = if is_leap_year(date.year) { 366 } else { 365 };
      cny_next - nday + days
    }
    _ => match weekday {
      5 => 2,
      6 => 1,
      _ => 0,
    },
  }
}

fn parse_date(time: &str) -> (i32, i32, i32) {
  let parts: Vec<&str> = time.split('-').collect();
  assert_eq!(parts.len(), 3, "无效日期格式，应为YYYY-MM-DD");

  let year = i32::from(parts[0].parse::<u16>().expect("无效年份"));
  let month = i32::from(parts[1].parse::<u8>().expect("无效月份"));
  let day = i32::from(parts[2].parse::<u8>().expect("无效日期"));

  (year, month, day)
}
