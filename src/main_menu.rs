use crate::NSString;
use cocoa::{
    appkit::{NSMenu, NSMenuItem},
    base::{id, nil, selector},
    foundation::{NSAutoreleasePool, NSProcessInfo},
};

pub unsafe fn new() -> id {
    let menubar = NSMenu::new(nil);
    let app_menu_item = NSMenuItem::new(nil).autorelease();
    menubar.addItem_(app_menu_item);

    let app_menu = NSMenu::new(nil).autorelease();
    let quit_prefix = NSString!("Quit ");
    let quit_title =
        quit_prefix.stringByAppendingString_(NSProcessInfo::processInfo(nil).processName());
    let quit_item = NSMenuItem::alloc(nil)
        .initWithTitle_action_keyEquivalent_(quit_title, selector("terminate:"), NSString!("q"))
        .autorelease();
    app_menu.addItem_(quit_item);
    app_menu_item.setSubmenu_(app_menu);

    menubar
}
