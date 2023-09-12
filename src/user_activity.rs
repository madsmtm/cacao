//! A module wrapping `NSUserActivity`.
//!
//! This is primarily used in handling app handoff between devices.

use objc::rc::Id;
use objc::runtime::NSObject;

use crate::foundation::id;

/// Represents an `NSUserActivity`, which acts as a lightweight method to capture
/// the state of your app.
#[derive(Debug)]
pub struct UserActivity(pub Id<NSObject>);

impl UserActivity {
    /// An internal method for wrapping a system-provided activity.
    pub(crate) fn with_inner(object: id) -> Self {
        UserActivity(unsafe { Id::retain(object).unwrap() })
    }
}
