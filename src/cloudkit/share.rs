//! This module includes wrappers for `CKShare` and `CKShareMetaData`.

use objc::rc::Id;
use objc::runtime::NSObject;

use crate::foundation::id;

/// A wrapper for `CKShareMetaData`, which describes details about a `CKShare`. You might use this
/// to, say, handle accepting an invite for a share.
#[derive(Clone, Debug)]
pub struct CKShareMetaData {
    pub inner: Id<NSObject>
}

impl CKShareMetaData {
    /// Internal method for wrapping a system-provided `CKShareMetaData` object.
    pub(crate) fn with_inner(object: id) -> Self {
        CKShareMetaData {
            inner: unsafe { Id::retain(object).unwrap() }
        }
    }
}
