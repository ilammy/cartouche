use cocoa::{
    base::{id, BOOL},
    foundation::{NSRect, NSSize},
};
use objc::{class, msg_send, sel, sel_impl};

pub trait NSTextContainer: Sized {
    unsafe fn setSize_(self, size: NSSize);
    unsafe fn setWidthTracksTextView_(self, widthTracksTextView: BOOL);
}

impl NSTextContainer for id {
    unsafe fn setSize_(self, size: NSSize) {
        msg_send![self, setSize: size]
    }

    unsafe fn setWidthTracksTextView_(self, widthTracksTextView: BOOL) {
        msg_send![self, setWidthTracksTextView: widthTracksTextView]
    }
}
