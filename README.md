# ts_static
Wrapper macro for `lazy_static!` and a struct to aid in accessing a static in a thread safe way.

Can be used in global scope or in module scope.

*Requires lazy_static imported to the scope where ts_static is used*

## Usage
```rust
ts_static!(STATIC_NAME, Type);
```

## Example
```rust
ts_static!(MY_STATIC_NAME, i32);
```

Static is set to a `ThreadSafeStruct<T>`.

To set the value use the `set` member function.

To work with the value use the `with` member function.

You can access the mutex field directly `value` but the helpers should
be suffice for most needs

### Set value
 ```rust
 MY_STATIC_NAME.set(Some(1337));
```
### Use value
```rust 
MY_STATIC_NAME.with(|value| {
    *value += 1 
}).expect(".with failed");
```
### Clear value
```rust
MY_STATIC_NAME.set(None);
```
