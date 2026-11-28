use std::{error::Error, fmt};

/// Operation that prevented an Oxide application from starting or continuing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppStage {
    /// Invalid initial window configuration.
    Configuration,
    /// Another Oxide application is already running.
    AlreadyRunning,
    /// Creating or running the platform event loop.
    EventLoop,
    /// Creating the operating system window.
    Window,
    /// Creating the graphics device, renderer, or presentation surface.
    GraphicsInitialization,
    /// Beginning a graphics frame.
    BeginFrame,
    /// Submitting or presenting a graphics frame.
    EndFrame,
    /// Resizing graphics resources.
    Resize,
}

/// Recoverable application failure, retaining its operation and original cause.
#[derive(Debug)]
pub struct AppError {
    stage: AppStage,
    source: Box<dyn Error + Send + Sync>,
}

impl AppError {
    pub(crate) fn new(stage: AppStage, source: impl Error + Send + Sync + 'static) -> Self {
        Self { stage, source: Box::new(source) }
    }

    /// Returns the operation that failed. The underlying error is available through Error::source.
    pub fn stage(&self) -> AppStage { self.stage }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let operation = match self.stage {
            AppStage::Configuration => "invalid application configuration",
            AppStage::AlreadyRunning => "an Oxide application is already running",
            AppStage::EventLoop => "Oxide event loop failed",
            AppStage::Window => "failed to create Oxide window",
            AppStage::GraphicsInitialization => "failed to initialize Oxide graphics",
            AppStage::BeginFrame => "failed to begin Oxide graphics frame",
            AppStage::EndFrame => "failed to finish Oxide graphics frame",
            AppStage::Resize => "failed to resize Oxide graphics",
        };
        write!(f, "{operation}: {}", self.source)
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> { Some(self.source.as_ref()) }
}
