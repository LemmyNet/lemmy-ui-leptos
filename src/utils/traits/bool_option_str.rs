pub trait BoolOptionStr {
    fn then_str(self) -> Option<&'static str>;
}

impl BoolOptionStr for bool {
    fn then_str(self) -> Option<&'static str> {
        self.then_some("true")
    }
}