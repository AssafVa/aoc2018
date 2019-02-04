pub struct IterableInput<'a> {
    path: &'a str
}

impl<'a> IterableInput<'a> {
    pub fn new(path: &'a str) -> IterableInput<'a> {
        IterableInput { path }
    }

    pub fn path(&self) -> &'a str{
        self.path
    }
}
