#[derive(Clone, Copy, PartialEq, Default)]
pub enum ContentActionType {
  #[default]
  Post,
  #[allow(dead_code)]
  Comment,
}

#[derive(Clone, Copy, PartialEq)]
pub struct Comments(pub i64);

#[derive(Clone, Copy, PartialEq)]
pub struct Hidden(pub bool);
