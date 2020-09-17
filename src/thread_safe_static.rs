use std::sync::Mutex;
use std::error::Error;
use core::fmt;

pub struct ThreadSafeStruct<T> {
    pub value: Mutex<Option<T>>
}

impl<T> ThreadSafeStruct<T> {
    /// Borrows the value from the mutex, mutably in order to work with/on
    pub fn with(&self, mut action: impl FnMut(&mut T)) -> Result<(), ThreadSafeStructError> {
        match self.value.lock() {
            Err(_) => return Err(ThreadSafeStructError { kind: ThreadSafeStructErrorKind::OtherThreadPanickedWithLock }),
            Ok(mut mutex_guard) => {
                match mutex_guard.as_mut() {
                    Some(value) => action(value),
                    None => return Err(ThreadSafeStructError { kind: ThreadSafeStructErrorKind::ValueNotSet })
                }
            }
        }
        Ok(())
    }

    /// Sets the mutex to a value or None
    ///
    /// This provides no guarantee that other threads don't change it straight away
    /// so if you call `with()` after value may have changed
    ///
    /// I.e. two threads, one calls set, the other calls with
    ///    `set` thread wins, and the `with` thread waits for the lock
    ///    as soon as thread `set` finishes, thread `with` will do its thing
    pub fn set(&self, value: Option<T>) {
        let mut mg = self.value.lock().unwrap();
        *mg = value;
    }
}

#[cfg(test)]
mod tests {
    use rayon::prelude::*;
    use lazy_static::*;
    use super::ThreadSafeStruct;

    extern crate test;

    use test::Bencher;
    use std::hint::black_box;
    use crate::ThreadSafeStructErrorKind;


    lazy_static! {
    static ref ERR_TEST: ThreadSafeStruct<i32> = ThreadSafeStruct {
        value: std::sync::Mutex::new(None),
    };}

    #[test]
    fn when_value_is_none_should_return_error() {

        let result = ERR_TEST.with(|_| {});

        assert_eq!(result.is_err(), true);
        assert_eq!(result.unwrap_err().kind, ThreadSafeStructErrorKind::ValueNotSet);
    }


    lazy_static! {
    static ref SET_TEST: ThreadSafeStruct<i32> = ThreadSafeStruct {
        value: std::sync::Mutex::new(None),
    };}

    #[test]
    fn when_setting_value_should_set_value() {

        SET_TEST.set(Some(222));

        let mut fetched_value: i32 = 0;

        SET_TEST.with(|x| {
            fetched_value = *x;
        }).expect("With Faulted!");

        assert_eq!(222, fetched_value);
    }


    lazy_static! {
    static ref ALTER_TEST: ThreadSafeStruct<i32> = ThreadSafeStruct {
        value: std::sync::Mutex::new(Some(222)),
    };}

    #[test]
    fn when_performing_with_should_be_able_to_alter_value() {

        ALTER_TEST.with(|x| {
            *x = *x * 2;
        }).expect("With Faulted!");

        let mut fetched_value: i32 = 0;
        ALTER_TEST.with(|x| {
            fetched_value = *x;
        }).expect("With Faulted!");

        assert_eq!(444, fetched_value);
    }


    lazy_static! {
    static ref THREAD_TEST: ThreadSafeStruct<u32> = ThreadSafeStruct {
        value: std::sync::Mutex::new(Some(0)),
    };}

    const TEST_SIZE: u32 = 100000;

    #[test]
    fn when_threading_should_return_no_errors() {
        let mut vals = <Vec<u8>>::with_capacity(TEST_SIZE as usize);
        unsafe { vals.set_len(TEST_SIZE as usize) }

        // !!!WARNING!!! I DID NOT SET THE VALUES IN `vals`
        // DO NOT TRY READ THEM IN THE THREAD
        vals.par_iter().for_each(|_| {
            THREAD_TEST.with(|i| { *i += 1; }).expect("With Faulted!");
        });
        let mut fetched_value: u32 = 0;
        assert_eq!(THREAD_TEST.with(|x| {
            fetched_value = *x;
        }).is_err(), false);
        assert_eq!(TEST_SIZE, fetched_value);
    }

    lazy_static! {
    static ref THREAD_BENCH_TEST: ThreadSafeStruct<u32> = ThreadSafeStruct {
        value: std::sync::Mutex::new(Some(0)),
    };}

    #[bench]
    fn bench_threaded_model(b: &mut Bencher) {
        b.iter(|| {
            THREAD_BENCH_TEST.set(Some(0));
            THREAD_BENCH_TEST.with(|i| {
                *i += 1;
                if *i % 2 == 1
                {
                    *i += 1;
                }
            }).expect("With Faulted!");
            //std::thread::sleep(Duration::new(0, 1));
        });
        let mut fetched_value: u32 = 0;
        assert_eq!(THREAD_TEST.with(|x| {
            fetched_value = *x;
        }).is_err(), false);
        assert_eq!(0, fetched_value * 0);
    }

    lazy_static! {
    static ref ITER_BENCH_TEST: ThreadSafeStruct<u32> = ThreadSafeStruct {
        value: std::sync::Mutex::new(Some(0)),
    };}

    #[bench]
    fn bench_normal_iterator(b: &mut Bencher) {
        let mut i = 1;
        b.iter(|| {
            i = 0;
            i += 1;
            if i % 2 == 1
            {
                i += 1;
            }
            //std::thread::sleep(Duration::new(0, 1));
        });
        assert_eq!(black_box(2), black_box(i));
    }
}


/// Error stuff
#[derive(Debug)]
pub struct ThreadSafeStructError {
    kind: ThreadSafeStructErrorKind
}

impl fmt::Display for ThreadSafeStructError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            ThreadSafeStructErrorKind::OtherThreadPanickedWithLock => write!(f, "Error getting lock, thread with lock panicked"),
            ThreadSafeStructErrorKind::ValueNotSet => write!(f, "Value was not set")
        }
    }
}

// TODO Implement source for debugging
impl Error for ThreadSafeStructError {}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum ThreadSafeStructErrorKind {
    OtherThreadPanickedWithLock,
    ValueNotSet,
}