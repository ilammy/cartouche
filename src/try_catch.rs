use cocoa::base::{id, nil};
use std::panic::{catch_unwind, UnwindSafe};

pub fn try_<F: FnOnce() -> R + UnwindSafe, R>(f: F) -> Try<F, R> {
    Try(f)
}

pub struct Try<F: FnOnce() -> R + UnwindSafe, R>(F);

impl<F: FnOnce() -> R + UnwindSafe, R> Try<F, R> {
    pub fn catch<H: FnOnce(id) -> R + UnwindSafe>(self, f: H) -> R {
        match catch_unwind(self.0) {
            Ok(v) => v,
            Err(panic) => {
                match panic.downcast::<String>() {
                    Ok(string) => {
                        // Uncaught exception <NSException: 0x7fab3154e0a0>
                        if string.starts_with("Uncaught exception") {
                            let pointer = string
                                .trim_start_matches("Uncaught exception <")
                                .trim_end_matches(">")
                                .split(": ")
                                .nth(1);
                            if let Some(pointer) = pointer {
                                if pointer.starts_with("0x") {
                                    let pointer = pointer.trim_start_matches("0x");
                                    // TODO: intptr, or something?
                                    let pointer = u64::from_str_radix(pointer, 16);
                                    if let Ok(pointer) = pointer {
                                        f(pointer as id)
                                    } else {
                                        println!("5");
                                        f(nil)
                                    }
                                } else {
                                    println!("4");
                                    f(nil)
                                }
                            } else {
                                println!("3");
                                f(nil)
                            }
                        } else {
                            println!("2");
                            f(nil)
                        }
                    }
                    Err(_panic) => {
                        println!("1");
                        f(nil)
                    }
                }
            }
        }
    }
}
