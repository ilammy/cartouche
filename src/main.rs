#[macro_use]
extern crate bitflags;

mod cacao;
mod macros;
mod main_menu;
mod main_window;
mod try_catch;

use cocoa::{
    appkit::{
        NSApp, NSApplication, NSApplicationActivateIgnoringOtherApps,
        NSApplicationActivationPolicyRegular, NSRunningApplication, NSWindow,
    },
    base::nil,
    foundation::{NSAutoreleasePool, NSString},
};
use objc::rc::autoreleasepool;

fn main() {
    autoreleasepool(|| unsafe {
        let app = NSApp();
        app.setActivationPolicy_(NSApplicationActivationPolicyRegular);

        let menubar = main_menu::new();
        app.setMainMenu_(menubar);

        let window = main_window::new().autorelease();
        window.center();
        window.makeKeyAndOrderFront_(nil);

        focus_app();

        app.run();
    });
}

unsafe fn focus_app() {
    let current_app = NSRunningApplication::currentApplication(nil);
    current_app.activateWithOptions_(NSApplicationActivateIgnoringOtherApps);
}
