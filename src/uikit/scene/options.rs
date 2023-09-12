use objc::rc::Id;
use objc::runtime::NSObject;
use objc::{class, msg_send, sel};

use crate::foundation::{id, NSString};

/// A wrapper for UISceneConfiguration.
///
/// Due to the way we have to implement this, you likely never need to touch this.
#[derive(Debug)]
pub struct SceneConnectionOptions(Id<NSMutableObject>);

impl SceneConnectionOptions {
    pub fn with(opts: id) -> Self {
        SceneConnectionOptions(unsafe { Id::retain(opts).unwrap() })
    }
}
