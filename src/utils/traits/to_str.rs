pub trait ToStr {
  fn to_str(self) -> &'static str;
}

impl ToStr for bool {
  fn to_str(self) -> &'static str {
    if self {
      "true"
    } else {
      "false"
    }
  }
}
