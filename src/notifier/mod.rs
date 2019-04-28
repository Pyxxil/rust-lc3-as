pub mod diagnostic;
pub use self::diagnostic::{Colour, NoColour};
pub use self::diagnostic::{
    Diagnostic, DiagnosticType, HighlightDiagnostic, NoteDiagnostic, PointerDiagnostic, Type,
};
use std::sync::Mutex;

trait Notify {
    fn notify(&self, diagnostic: &Diagnostic);
}

pub enum StdoutNotifier {
    NoColour,
    Colour,
    Quiet,
}

// TODO: At the moment, notifications will only work for StdoutNotifier. This
//       should be changed to be more similar to how the C++ version does it
//       i.e. with Callbacks.

/**
 * The `StdoutNotifier` will simply push the diagnostic to stdout, with
 * optional colouring.
 */
impl Notify for StdoutNotifier {
    fn notify(&self, diagnostic: &Diagnostic) {
        match *self {
            StdoutNotifier::NoColour => match *diagnostic {
                Diagnostic::Note(ref d) => println!("{}", d.no_colour()),
                Diagnostic::Highlight(ref d) => println!("{}", d.no_colour()),
                Diagnostic::Pointer(ref d) => println!("{}", d.no_colour()),
            },
            StdoutNotifier::Colour => match *diagnostic {
                Diagnostic::Note(ref d) => println!("{}", d.colour()),
                Diagnostic::Highlight(ref d) => println!("{}", d.colour()),
                Diagnostic::Pointer(ref d) => println!("{}", d.colour()),
            },
            StdoutNotifier::Quiet => {}
        }
    }
}

#[derive(Default)]
pub struct NotificationController {
    notifiers: Vec<StdoutNotifier>,
    diagnostics: Vec<Diagnostic>,
}

#[inline]
pub fn add_notifier(notifier: StdoutNotifier) {
    let mut guard = NOTIFICATION_CONTROLLER.lock().unwrap();
    guard.register(notifier);
}

#[inline]
pub fn add_diagnostic(diagnostic: Diagnostic) {
    let mut guard = NOTIFICATION_CONTROLLER.lock().unwrap();
    guard.push(diagnostic);
}

#[inline]
pub fn error_count() -> u64 {
    let guard = NOTIFICATION_CONTROLLER.lock().unwrap();
    guard
        .diagnostics()
        .iter()
        .filter(|diag| diag.diagnostic_type() == &DiagnosticType::Error)
        .count() as u64
}

#[inline]
pub fn clear() {
    let mut guard = NOTIFICATION_CONTROLLER.lock().unwrap();
    guard.diagnostics.clear();
}

impl NotificationController {
    fn push(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
        self.notify();
    }

    #[inline]
    fn diagnostics(&self) -> &Vec<Diagnostic> {
        &self.diagnostics
    }

    #[inline]
    fn register(&mut self, notification: StdoutNotifier) {
        self.notifiers.push(notification);
    }

    fn notify(&self) {
        if let Some(diagnostic) = self.diagnostics.last() {
            self.notifiers
                .iter()
                .for_each(|notifier| notifier.notify(diagnostic))
        }
    }
}

lazy_static! {
    pub static ref NOTIFICATION_CONTROLLER: Mutex<NotificationController> =
        Mutex::new(NotificationController::default());
}
