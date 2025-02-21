pub fn retire_time(time: &str, tp: &str) -> String {
  let birth: Vec<f32> = time.split('-').map(|x| x.parse().unwrap()).collect();
  let is_male = tp.contains("男");

  if is_male {
    hndl_male(&birth)
  } else {
    hndl_female(tp, &birth)
  }
}

fn hndl_male(birth: &[f32]) -> String {
  let (base_year, base_age, max_delay) =
    if birth[0] < 1965.0 { (1965.0, 60.0, 0.0) } else { (1965.0, 60.0, 36.0) };
  calc_rtrmt(birth, base_year, base_age, max_delay)
}

fn hndl_female(tp: &str, birth: &[f32]) -> String {
  let (base_year, base_age, max_delay) = if tp.contains("50") {
    (1975.0, 50.0, 60.0)
  } else if birth[0] < 1970.0 {
    (1970.0, 55.0, 0.0)
  } else {
    (1970.0, 55.0, 36.0)
  };
  calc_rtrmt(birth, base_year, base_age, max_delay)
}

fn calc_rtrmt(birth: &[f32], base_year: f32, age_base: f32, max_delay: f32) -> String {
  if birth[0] < base_year {
    return format!("{}-{:0>2},{},0", birth[0] + age_base, birth[1], age_base);
  }

  let month_since_base = (birth[0] - base_year) * 12.0 + birth[1];
  let diff = (month_since_base / 4.0).ceil();

  if diff >= max_delay {
    let fin_age = age_base + (max_delay / 12.0);
    return format!("{}-{:0>2},{},{}", birth[0] + fin_age, birth[1], fin_age, max_delay);
  }

  let month_ttl = diff + birth[1];
  let y = (month_ttl / 12.0).floor();
  let m = (month_ttl % 12.0).floor();

  let age_rtrmt = age_base + diff / 12.0;
  format!("{}-{:0>2},{:.2},{}", birth[0] + age_base + y, m, age_rtrmt, diff)
}
