#[derive(Debug,Clone)]
pub struct DrawError {
    kind : DrawErrorKind,
}

#[derive(Debug,Clone)]
pub enum DrawErrorKind {
    WidgetOutOfTerminalBounds,
    ContentOutOfWidgetBounds,
    OverlappingWidgets,
}

impl std::fmt::Display for DrawError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Encountered an error when drawing ui of type : [{:?}]", self.kind)
    }
}
impl std::error::Error for DrawError {}