use cocoa::{
    base::{id, BOOL},
    foundation::NSRect,
};
use objc::{class, msg_send, sel, sel_impl};

pub trait NSTextView: Sized {
    unsafe fn alloc(_: Self) -> id {
        msg_send![class!(NSTextView), alloc]
    }

    unsafe fn setEditable_(self, editable: BOOL);
    unsafe fn textContainer_(self) -> id;
}

impl NSTextView for id {
    unsafe fn setEditable_(self, editable: BOOL) {
        msg_send![self, setEditable: editable]
    }

    unsafe fn textContainer_(self) -> id {
        msg_send![self, textContainer]
    }
}
