use cocoa::{base::id, foundation::NSRect};
use objc::{class, msg_send, sel, sel_impl};

pub trait NSTextField: Sized {
    unsafe fn textFieldWithString_(_: Self, stringValue: id) -> id {
        msg_send![class!(NSTextField), textFieldWithString: stringValue]
    }
}

impl NSTextField for id {}
