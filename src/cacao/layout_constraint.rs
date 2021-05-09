use crate::try_catch::try_;
use cocoa::{
    appkit::CGFloat,
    base::{id, nil},
    foundation::{NSString, NSUInteger},
};
use objc::{class, msg_send, sel, sel_impl};

pub trait NSLayoutConstraint: Sized {
    unsafe fn constraintWithItem_attribute_relatedBy_toItem_attribute_multiplier_constant_(
        _: Self,
        view1: id,
        attr1: NSLayoutAttribute,
        relation: NSLayoutRelation,
        view2: id,
        attr2: NSLayoutAttribute,
        multiplier: CGFloat,
        c: CGFloat,
    ) -> id {
        try_(|| {
            msg_send![class!(NSLayoutConstraint), constraintWithItem:view1
                                                       attribute:attr1
                                                       relatedBy:relation
                                                          toItem:view2
                                                       attribute:attr2
                                                      multiplier:multiplier
                                                        constant:c]
        })
        .catch(|exception| {
            println!("XXX caught exception: {:?}", exception);
            let reason: id = msg_send![exception, reason];
            if reason != nil {
                let reason = std::ffi::CStr::from_ptr(reason.UTF8String());
                let reason = reason.to_str().expect("valid UTF-8");
                println!("XXX reason: {}", reason);
            }
            nil
        })
    }

    unsafe fn constraintsWithVisualFormat_options_metrics_views_(
        _: Self,
        format: id,
        opts: NSLayoutFormatOptions,
        metrics: id,
        views: id,
    ) -> id {
        try_(|| {
            msg_send![class!(NSLayoutConstraint), constraintsWithVisualFormat:format
                                                                      options:opts
                                                                      metrics:metrics
                                                                        views:views]
        })
        .catch(|exception| {
            println!("XXX caught exception: {:?}", exception);
            let reason: id = msg_send![exception, reason];
            if reason != nil {
                let reason = std::ffi::CStr::from_ptr(reason.UTF8String());
                let reason = reason.to_str().expect("valid UTF-8");
                println!("XXX reason: {}", reason);
            }
            nil
        })
    }

    unsafe fn activateConstraints_(_: Self, constraints: id) {
        msg_send![class!(NSLayoutConstraint), activateConstraints: constraints]
    }
}

impl NSLayoutConstraint for id {}

#[repr(i64)] // NSInteger
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSLayoutAttribute {
    NSLayoutAttributeLeft = 1,
    NSLayoutAttributeRight,
    NSLayoutAttributeTop,
    NSLayoutAttributeBottom,
    NSLayoutAttributeLeading,
    NSLayoutAttributeTrailing,
    NSLayoutAttributeWidth,
    NSLayoutAttributeHeight,
    NSLayoutAttributeCenterX,
    NSLayoutAttributeCenterY,
    // NSLayoutAttributeBaseline == NSLayoutAttributeLastBaseline,
    // NSLayoutAttributeLastBaseline,
    // NSLayoutAttributeFirstBaseline,
    // TODO: remaining
    NSLayoutAttributeNotAnAttribute = 0,
}

#[repr(i64)] // NSInteger
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSLayoutRelation {
    NSLayoutRelationLessThanOrEqual = -1,
    NSLayoutRelationEqual = 0,
    NSLayoutRelationGreaterThanOrEqual = 1,
}

bitflags! {
    pub struct NSLayoutFormatOptions : NSUInteger {
        const NSLayoutFormatAlignAllLeft   = (1 << NSLayoutAttribute::NSLayoutAttributeLeft as i64);
        const NSLayoutFormatAlignAllRight  = (1 << NSLayoutAttribute::NSLayoutAttributeRight as i64);
        const NSLayoutFormatAlignAllTop    = (1 << NSLayoutAttribute::NSLayoutAttributeTop as i64);
        // TODO: remaining
        const NSLayoutFormatAlignAllCenterX = (1 << NSLayoutAttribute::NSLayoutAttributeCenterX as i64);
        // TODO: remaining
    }
}
