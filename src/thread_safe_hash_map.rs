use crate::ThreadSafeStruct;
use std::collections::HashMap;
use std::hash::Hash;

pub trait TsHashMap<T,U> {
    fn remove(&self, key: &T) -> Result<U, String>;
    fn insert(&self, key: T, value: U) -> Result<Option<U>, String>;
}

impl<T,U> TsHashMap<T,U> for ThreadSafeStruct<HashMap<T,U>> where T: Eq + Hash, U: Eq + Hash {
    fn remove(&self, key: &T)-> Result<U, String> {
        let mut result: Result<U, String> = Err(format!("This is a bug"));
        match self.with(|value|{
            result = match value.remove(key) {
                Some(v) => Ok(v),
                None => Err(format!("Error, key did not exist in hashmap"))
            }
        }) { Err(e) => return Err(e.to_string()),
            _=>()};
        result
    }
    fn insert(&self, key:T, value:U) -> Result<Option<U>, String> {
        let mut result: Result<Option<U>, String> = Err(format!("This is a bug"));
        let x = &mut result;
        match self.with(|hm|{
            *x = Ok(hm.insert(key, value));
        }) { Err(e) => return Err(e.to_string()),
            _=>()};
        result
    }
}

mod tests {
    use std::collections::HashMap;
    use lazy_static::*;
    #[macro_use]
    use crate::*;
    use crate::thread_safe_hash_map::TsHashMap;

    ts_static!(TEST_ADD, HashMap<i32,i32>);

    #[test]
    fn should_add_and_get_value() {
        TEST_ADD.set(Some(HashMap::new()));

        TEST_ADD.insert(1, 2);

        let value = TEST_ADD.remove(&1).expect("Couldn't remove value");

        assert_eq!(2, value);
    }

    ts_static!(TEST_EXIST_ERROR, HashMap<i32,i32>);

    #[test]
    fn when_key_does_not_exist_should_get_an_error() {
        TEST_EXIST_ERROR.set(Some(HashMap::new()));

        let result = TEST_EXIST_ERROR.remove(&1);

        assert_eq!(true, result.is_err());
    }
}