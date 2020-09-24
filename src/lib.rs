#![feature(test)]
#![allow(dead_code)]
mod thread_safe_static;
mod thread_safe_hash_map;

pub use thread_safe_static::*;

/// # Helper to create a thread safe static value
///
/// Can be used in global scope in module scope
/// Required lazy_static imported to scope used
///
/// # Usage
/// `ts_static!(STATIC_NAME, Type)`
///
/// # Example
/// `ts_static!(MY_STATIC_NAME, i32)`
///
/// # Notes
/// Static is set to a `ThreadSafeStruct<T>` to use the value
/// use the with function, to set the value use the `set` function
///
/// You can access the mutex field directly `value` but the helpers should
/// be suffice for most needs
///
///  ### Set value
///  `MY_STATIC_NAME.set(Some(1337));`
///
///  ### Use value
/// `MY_STATIC_NAME.with(|value| { *value += 1 }).expect(".with failed");`
///
///  ### Clear value
/// `MY_STATIC_NAME.set(None);`
#[macro_export]
macro_rules! ts_static {
    ($name:ident, $type_ty: ty) =>
    {
        lazy_static! {
            static ref $name: ThreadSafeStruct<$type_ty> = ThreadSafeStruct {
                value: std::sync::Mutex::new(None),
            };
        }
    }
}



/*use lazy_static::lazy_static;
fn default_value() -> i32 { 123 }

ts_static_with_default!(MY_STATIC2, i32, default_value());

fn tt() {
    MY_STATIC2.with(|x| {
        assert_eq!(*x, 123);
    });
}*/

/// # Same as ts_static only can be assigned a value from a function
/// Calue must be identifier of a function
///
///
/// # Example
///
/// ```
/// use lazy_static::lazy_static;
/// use ts_static::*;
///
/// fn my_default() -> i32 { 123 }
///
/// ts_static_with_default!(MY_STATIC, i32, my_default);
/// MY_STATIC.with(|x| {
///         assert_eq!(123, *x);
///     });
/// ```
#[macro_export]
macro_rules! ts_static_with_default {
    ($name:ident, $type_ty: ty,$default_function:ident) =>
    {
        lazy_static! {
            static ref $name: ThreadSafeStruct<$type_ty> = ThreadSafeStruct {
                value: std::sync::Mutex::new(Some($default_function())),
            };
        }
    }
}

use lazy_static::lazy_static;
fn do_something() -> i32 {
    123
}
ts_static_with_default!(MY_STATIC, i32, do_something);
#[test]
fn test_ts_static_with_default() {
    MY_STATIC.with(|x| {
        assert_eq!(123, *x)
    }).expect("Error");
}