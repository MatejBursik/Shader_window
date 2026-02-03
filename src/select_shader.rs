#[derive(PartialEq)]
pub enum SelectShader {
    None,
    Test,
    Pixel,
    Ascii,
    EdgeDetect
}

impl SelectShader {
    pub fn next(self) -> Self {
        match self {
            SelectShader::None => SelectShader::Test,
            SelectShader::Test => SelectShader::Pixel,
            SelectShader::Pixel => SelectShader::Ascii,
            SelectShader::Ascii => SelectShader::EdgeDetect,
            SelectShader::EdgeDetect => SelectShader::None
        }
    }
}
