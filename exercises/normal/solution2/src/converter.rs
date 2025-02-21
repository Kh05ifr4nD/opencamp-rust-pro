pub fn convert_base(str_num: &str, base_dest: u32) -> String {
  let (part_num, part_base) = str_num.split_once('(').expect("Invalid input format");
  let base_src = part_base.trim_end_matches(')').parse::<u32>().expect("Invalid base format");

  let dec = part_num.chars().fold(0u64, |acc, c| {
    let digit = c.to_digit(base_src).expect("Invalid character for source base") as u64;
    acc * base_src as u64 + digit
  });

  if dec == 0 {
    return "0".to_string();
  }

  let mut rslt = Vec::new();
  let mut n = dec;
  while n > 0 {
    let rem = (n % base_dest as u64) as u32;
    let c = match rem {
      0..=9 => (b'0' + rem as u8) as char,
      _ => (b'a' + (rem - 10) as u8) as char,
    };
    rslt.push(c);
    n /= base_dest as u64;
  }

  rslt.into_iter().rev().collect()
}
