#[derive(PartialEq)]
pub enum SelectMode {
    ScreenCapture,
    Image
}

impl SelectMode {
    pub fn next(self) -> Self {
        match self {
            SelectMode::Image => SelectMode::ScreenCapture,
            SelectMode::ScreenCapture => SelectMode::Image
        }
    }
}
