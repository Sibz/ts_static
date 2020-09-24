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