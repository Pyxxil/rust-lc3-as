use std::collections::HashMap;
use std::sync::Mutex;

pub use self::diagnostic::{Colour, NoColour};
pub use self::diagnostic::{DiagType, Diagnostic, Highlight, Note, Pointer, Type};

pub mod diagnostic;

trait Notify {
    fn notify(&mut self, diagnostic: &Diagnostic);
}

pub enum Stdout {
    NoColour,
    Colour,
    Quiet,
}

pub enum Notifier {
    Standard(Stdout),
    Stringify(Vec<String>),
}

/// `Standard` will simply push the diagnostic to stdout, with
/// optional colouring.
///
/// `Stringifiy` will simply collect each into a vector for later
impl Notify for Notifier {
    fn notify(&mut self, diagnostic: &Diagnostic) {
        match *self {
            Self::Standard(ref stdout) => match stdout {
                Stdout::NoColour => println!("{}", diagnostic.uncoloured()),
                Stdout::Colour => println!("{}", diagnostic.coloured()),
                Stdout::Quiet => {}
            },
            Self::Stringify(ref mut strings) => strings.push(diagnostic.uncoloured()),
        }
    }
}

impl Notifier {
    #[must_use]
    pub fn inner(&self) -> Vec<String> {
        match self {
            Self::Stringify(i) => i.clone(),
            _ => Vec::new(),
        }
    }

    pub fn reset(&mut self) {
        if let Self::Stringify(i) = self {
            i.clear();
        }
    }
}

#[derive(Default)]
pub struct NotificationController {
    notifiers: HashMap<String, Notifier>,
    diagnostics: Vec<Diagnostic>,
}

#[inline]
pub fn register(name: String, notifier: Notifier) {
    let mut guard = NOTIFICATION_CONTROLLER.lock().unwrap();
    guard.register(name, notifier);
}

#[inline]
pub fn add_diagnostic(diagnostic: Diagnostic) {
    let mut guard = NOTIFICATION_CONTROLLER.lock().unwrap();
    guard.push(diagnostic);
}

#[inline]
#[must_use]
pub fn error_count() -> u64 {
    let guard = NOTIFICATION_CONTROLLER.lock().unwrap();
    guard
        .diagnostics()
        .iter()
        .filter(|diag| diag.diagnostic_type() == &DiagType::Error)
        .count() as u64
}

#[inline]
pub fn clear(notifier: Option<&str>) {
    let mut guard = NOTIFICATION_CONTROLLER.lock().unwrap();
    guard.diagnostics.clear();

    if let Some(n) = notifier {
        if let Some(no) = guard.notifiers.get_mut(n) {
            no.reset();
        }
    }
}

#[inline]
pub fn notifications() -> Vec<String> {
    let guard = NOTIFICATION_CONTROLLER.lock().unwrap();
    guard
        .notifiers
        .values()
        .find_map(|notifier| match notifier {
            Notifier::Stringify(_) => Some(notifier.inner()),
            _ => None,
        })
        .unwrap_or_else(Vec::new)
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
    fn register(&mut self, name: String, notification: Notifier) {
        self.notifiers.insert(name, notification);
    }

    fn notify(&mut self) {
        if let Some(diagnostic) = self.diagnostics.last() {
            self.notifiers
                .values_mut()
                .for_each(|notifier| notifier.notify(diagnostic))
        }
    }
}

lazy_static! {
    pub static ref NOTIFICATION_CONTROLLER: Mutex<NotificationController> =
        Mutex::new(NotificationController::default());
}
