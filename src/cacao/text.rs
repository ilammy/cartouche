use cocoa::{
    base::{id, BOOL},
    foundation::NSRect,
};
use objc::{class, msg_send, sel, sel_impl};

pub trait NSText: Sized {
    unsafe fn setString_(self, string: id);
    unsafe fn setHorizontallyResizable_(self, horizontallyResizable: BOOL);
    unsafe fn setVerticallyResizable_(self, verticallyResizable: BOOL);
}

impl NSText for id {
    unsafe fn setString_(self, string: id) {
        msg_send![self, setString: string]
    }

    unsafe fn setHorizontallyResizable_(self, horizontallyResizable: BOOL) {
        msg_send![self, setHorizontallyResizable: horizontallyResizable]
    }

    unsafe fn setVerticallyResizable_(self, verticallyResizable: BOOL) {
        msg_send![self, setVerticallyResizable: verticallyResizable]
    }
}
