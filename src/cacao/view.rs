use cocoa::base::{id, BOOL};
use objc::{msg_send, sel, sel_impl};

pub trait NSView: Sized {
    unsafe fn setTranslatesAutoresizingMaskIntoConstraints_(
        self,
        translatesAutoresizingMaskIntoConstraints: BOOL,
    );

    unsafe fn addConstraint_(self, constraint: id);

    unsafe fn setContentHuggingPriority_forOrientation_(
        self,
        priority: NSLayoutPriority,
        orientation: NSLayoutConstraintOrientation,
    );

    unsafe fn setContentCompressionResistancePriority_forOrientation_(
        self,
        priority: NSLayoutPriority,
        orientation: NSLayoutConstraintOrientation,
    );
}

type NSLayoutPriority = f32;
// TODO: constants

#[repr(i64)] // NSInteger
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSLayoutConstraintOrientation {
    NSLayoutConstraintOrientationHorizontal = 0,
    NSLayoutConstraintOrientationVertical = 1,
}

impl NSView for id {
    unsafe fn setTranslatesAutoresizingMaskIntoConstraints_(
        self,
        translatesAutoresizingMaskIntoConstraints: BOOL,
    ) {
        msg_send![
            self,
            setTranslatesAutoresizingMaskIntoConstraints: translatesAutoresizingMaskIntoConstraints
        ]
    }

    unsafe fn addConstraint_(self, constraint: id) {
        msg_send![self, addConstraint: constraint]
    }

    unsafe fn setContentHuggingPriority_forOrientation_(
        self,
        priority: NSLayoutPriority,
        orientation: NSLayoutConstraintOrientation,
    ) {
        msg_send![self, setContentHuggingPriority:priority
                                   forOrientation:orientation]
    }

    unsafe fn setContentCompressionResistancePriority_forOrientation_(
        self,
        priority: NSLayoutPriority,
        orientation: NSLayoutConstraintOrientation,
    ) {
        msg_send![self, setContentCompressionResistancePriority:priority
                                                 forOrientation:orientation]
    }
}
