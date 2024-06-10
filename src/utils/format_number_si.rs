const SUFFIXES: [&str; 3] = ["k", "M", "B"];

pub fn format_number_si(number: i64) -> String {
  if number < 1000 {
    number.to_string()
  } else {
    let mut number_as_float = number as f32;
    for suffix in SUFFIXES {
      number_as_float /= 1000f32;

      if number_as_float < 1000f32 {
        return format!(
          "{:.*}{suffix}",
          if (number_as_float - number_as_float.floor()) < 0.1 || number_as_float >= 100f32 {
            0
          } else {
            1
          },
          number_as_float
        );
      }
    }

    panic!("Number {number} is larger than 1 trillion! Where are you getting numbers this big?");
  }
}

#[cfg(test)]
mod test {
  use super::format_number_si;

  #[test]
  fn formats_1_digit_number() {
    let number = 7;
    let formatted = format_number_si(number);

    assert_eq!(formatted.as_str(), "7");
  }

  #[test]
  fn formats_2_digit_number() {
    let number = 42;
    let formatted = format_number_si(number);

    assert_eq!(formatted.as_str(), "42");
  }

  #[test]
  fn formats_3_digit_number() {
    let number = 717;
    let formatted = format_number_si(number);

    assert_eq!(formatted.as_str(), "717");
  }

  #[test]
  fn formats_4_digit_number_without_decimal() {
    let number = 1_001;
    let formatted = format_number_si(number);

    assert_eq!(formatted.as_str(), "1k");
  }

  #[test]
  fn formats_4_digit_number_with_decimal() {
    let number = 1_624;
    let formatted = format_number_si(number);

    assert_eq!(formatted.as_str(), "1.6k");
  }

  #[test]
  fn formats_5_digit_number_without_decimal() {
    let number = 19_007;
    let formatted = format_number_si(number);

    assert_eq!(formatted.as_str(), "19k");
  }

  #[test]
  fn formats_5_digit_number_with_decimal() {
    let number = 73_444;
    let formatted = format_number_si(number);

    assert_eq!(formatted.as_str(), "73.4k");
  }

  #[test]
  fn formats_6_digit_number_without_decimal() {
    let number = 469_070;
    let formatted = format_number_si(number);

    assert_eq!(formatted.as_str(), "469k");
  }

  #[test]
  fn formats_6_digit_number_with_decimal() {
    let number = 945_661;
    let formatted = format_number_si(number);

    assert_eq!(formatted.as_str(), "946k");
  }

  #[test]
  fn formats_7_digit_number_without_decimal() {
    let number = 3_001_500;
    let formatted = format_number_si(number);

    assert_eq!(formatted.as_str(), "3M");
  }

  #[test]
  fn formats_7_digit_number_with_decimal() {
    let number = 7_926_400;
    let formatted = format_number_si(number);

    assert_eq!(formatted.as_str(), "7.9M");
  }

  #[test]
  fn formats_8_digit_number_without_decimal() {
    let number = 75_032_115;
    let formatted = format_number_si(number);

    assert_eq!(formatted.as_str(), "75M");
  }

  #[test]
  fn formats_8_digit_number_with_decimal() {
    let number = 23_333_452;
    let formatted = format_number_si(number);

    assert_eq!(formatted.as_str(), "23.3M");
  }

  #[test]
  fn formats_9_digit_number_without_decimal() {
    let number = 555_067_885;
    let formatted = format_number_si(number);

    assert_eq!(formatted.as_str(), "555M");
  }

  #[test]
  fn formats_9_digit_number_with_decimal() {
    let number = 352_344_120;
    let formatted = format_number_si(number);

    assert_eq!(formatted.as_str(), "352M");
  }

  #[test]
  fn formats_10_digit_number_without_decimal() {
    let number = 2_004_254_578;
    let formatted = format_number_si(number);

    assert_eq!(formatted.as_str(), "2B");
  }

  #[test]
  fn formats_10_digit_number_with_decimal() {
    let number = 7_667_973_223;
    let formatted = format_number_si(number);

    assert_eq!(formatted.as_str(), "7.7B");
  }

  #[test]
  fn formats_11_digit_number_without_decimal() {
    let number = 87_050_671_768;
    let formatted = format_number_si(number);

    assert_eq!(formatted.as_str(), "87B");
  }

  #[test]
  fn formats_11_digit_number_with_decimal() {
    let number = 44_444_333_222;
    let formatted = format_number_si(number);

    assert_eq!(formatted.as_str(), "44.4B");
  }

  #[test]
  fn formats_12_digit_number_without_decimal() {
    let number = 899_055_111_032;
    let formatted = format_number_si(number);

    assert_eq!(formatted.as_str(), "899B");
  }

  #[test]
  fn formats_12_digit_number_with_decimal() {
    let number = 723_999_324_999;
    let formatted = format_number_si(number);

    assert_eq!(formatted.as_str(), "724B");
  }

  #[test]
  #[should_panic]
  fn format_13_digit_number_should_panic() {
    let number = 1_222_333_444_555;
    let _ = format_number_si(number);
  }
}
