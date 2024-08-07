use format_num::NumberFormat;

const PATTERN: &str = ".3s";

pub fn si_format<T>(num: T) -> String
where
  T: Into<f64>,
{
  let fmt = NumberFormat::new();
  fmt.format(PATTERN, num)
}

pub fn si_format_i64(num: i64) -> String {
  si_format(num as f64)
}

#[cfg(test)]
mod tests {
  use crate::utils::format::si_format;

  #[test]
  fn format_check() {
    assert_eq!(si_format(12e6), "12.0M");
    assert_eq!(si_format(12.345), "12.3");
    assert_eq!(si_format(12345), "12.3k");
    assert_eq!(si_format(123456), "123k");
  }
}
