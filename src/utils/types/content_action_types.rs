#[derive(Clone, Copy)]
pub enum ContentId {
  Post(i32),
  #[allow(dead_code)]
  Comment(i32),
}

impl ContentId {
  pub fn get_id(&self) -> i32 {
    match self {
      Self::Post(id) => *id,
      Self::Comment(id) => *id,
    }
  }
}

impl Default for ContentId {
  fn default() -> Self {
    Self::Post(0)
  }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Hidden(pub bool);
