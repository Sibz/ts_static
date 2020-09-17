# ts_static
Wrapper macro for lazy static and a struct to aid in accessing a static in a thread safe way.

Can be used in global scope in module scope
Required lazy_static imported to scope used

## Usage
`ts_static!(STATIC_NAME, Type)`

## Example
`ts_static!(MY_STATIC_NAME, i32)`

Static is set to a `ThreadSafeStruct<T>` to use the value
use the with function, to set the value use the `set` function

You can access the mutex field directly `value` but the helpers should
be suffice for most needs

### Set value
 `MY_STATIC_NAME.set(Some(1337));`
### Use value
```rust 
MY_STATIC_NAME.with(|value| {
    *value += 1 
}).expect(".with failed");
```
### Clear value
`MY_STATIC_NAME.set(None);`
