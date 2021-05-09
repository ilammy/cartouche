use crate::cacao::{NSBorderType, NSLayoutFormatOptions};
use crate::{
    cacao::{
        NSLayoutConstraint, NSLayoutConstraintOrientation, NSScrollView, NSText, NSTextContainer,
        NSTextField, NSTextView, NSView as NSViewExt,
    },
    NSDictionary, NSString,
};
use cocoa::appkit::{NSViewHeightSizable, NSViewWidthSizable};
use cocoa::{
    appkit::{
        CGFloat, NSBackingStoreType::NSBackingStoreBuffered, NSView, NSWindow, NSWindowStyleMask,
    },
    base::{id, nil, NO, YES},
    foundation::{NSAutoreleasePool, NSPoint, NSRect, NSSize},
};

pub unsafe fn new() -> id {
    let window_rect = NSRect::new(NSPoint::new(0., 0.), NSSize::new(640., 480.));

    let window_style_mask = NSWindowStyleMask::NSTitledWindowMask
        | NSWindowStyleMask::NSClosableWindowMask
        | NSWindowStyleMask::NSMiniaturizableWindowMask
        | NSWindowStyleMask::NSResizableWindowMask;
    let window = NSWindow::alloc(nil).initWithContentRect_styleMask_backing_defer_(
        window_rect,
        window_style_mask,
        NSBackingStoreBuffered,
        NO,
    );
    window.setTitle_(NSString!("Cartouche"));

    let address_bar = NSTextField::textFieldWithString_(nil, NSString!("address bar"));
    address_bar.setTranslatesAutoresizingMaskIntoConstraints_(NO);

    let lines_of_text: String = (1..=1000)
        .into_iter()
        .map(|n| format!("line {}\n", n))
        .collect();

    let text_view = NSTextView::alloc(nil).initWithFrame_(window_rect);
    text_view.setAutoresizingMask_(NSViewWidthSizable);
    text_view.setEditable_(NO);
    text_view.setHorizontallyResizable_(NO);
    text_view.setVerticallyResizable_(YES);
    text_view.setString_(NSString!(lines_of_text));

    let text_container = text_view.textContainer_();
    text_container.setWidthTracksTextView_(YES);

    let text_scroll_view = NSScrollView::alloc(nil).initWithFrame_(window_rect);
    text_scroll_view.setTranslatesAutoresizingMaskIntoConstraints_(NO);
    text_scroll_view.setBorderType_(NSBorderType::NSNoBorder);
    text_scroll_view.setHasHorizontalScroller_(NO);
    text_scroll_view.setHasVerticalScroller_(YES);
    text_scroll_view.setDocumentView_(text_view);

    let content_view = NSView::alloc(nil).initWithFrame_(window_rect);
    content_view.setTranslatesAutoresizingMaskIntoConstraints_(NO);
    content_view.addSubview_(address_bar);
    content_view.addSubview_(text_scroll_view);

    window.setContentView_(content_view);

    let views = NSDictionary! {
        NSString!("address_bar") => address_bar,
        NSString!("text_scroll_view") => text_scroll_view,
    }
    .autorelease();

    let constraints = NSLayoutConstraint::constraintsWithVisualFormat_options_metrics_views_(
        nil,
        NSString!("H:|-[address_bar]-|"),
        NSLayoutFormatOptions::empty(),
        nil,
        views,
    )
    .autorelease();
    NSLayoutConstraint::activateConstraints_(nil, constraints);

    let constraints = NSLayoutConstraint::constraintsWithVisualFormat_options_metrics_views_(
        nil,
        NSString!("H:|[text_scroll_view]|"),
        NSLayoutFormatOptions::empty(),
        nil,
        views,
    )
    .autorelease();
    NSLayoutConstraint::activateConstraints_(nil, constraints);

    let constraints = NSLayoutConstraint::constraintsWithVisualFormat_options_metrics_views_(
        nil,
        NSString!("V:|-[address_bar]-[text_scroll_view]|"),
        NSLayoutFormatOptions::empty(),
        nil,
        views,
    )
    .autorelease();
    NSLayoutConstraint::activateConstraints_(nil, constraints);

    window
}
