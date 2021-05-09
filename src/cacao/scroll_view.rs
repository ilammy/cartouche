use cocoa::{
    base::{id, BOOL},
    foundation::NSRect,
};
use objc::{class, msg_send, sel, sel_impl};

pub trait NSScrollView: Sized {
    unsafe fn alloc(_: Self) -> id {
        msg_send![class!(NSScrollView), alloc]
    }

    unsafe fn setBorderType_(self, borderType: NSBorderType);
    unsafe fn setDocumentView_(self, documentView: id);
    unsafe fn setHasHorizontalScroller_(self, hasHorizontalScroller: BOOL);
    unsafe fn setHasVerticalScroller_(self, hasVerticalScroller: BOOL);
}

impl NSScrollView for id {
    unsafe fn setBorderType_(self, borderType: NSBorderType) {
        msg_send![self, setBorderType: borderType]
    }

    unsafe fn setDocumentView_(self, documentView: id) {
        msg_send![self, setDocumentView: documentView]
    }

    unsafe fn setHasHorizontalScroller_(self, hasHorizontalScroller: BOOL) {
        msg_send![self, setHasHorizontalScroller: hasHorizontalScroller]
    }

    unsafe fn setHasVerticalScroller_(self, hasVerticalScroller: BOOL) {
        msg_send![self, setHasVerticalScroller: hasVerticalScroller]
    }
}

#[repr(i64)] // NSInteger
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSBorderType {
    NSNoBorder = 0,
    NSLineBorder = 1,
    NSBezelBorder = 2,
    NSGrooveBorder = 3,
}
