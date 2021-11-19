use std::cell::UnsafeCell;
use std::fmt;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicBool, Ordering};

use log::error;

use crate::errors::WebError;
use crate::lock::request_tick::{request_tick_after_timeout, ClosureHandle};

/// Like a Mutex but not as smart
pub struct Stupex<T: ?Sized> {
    locked: AtomicBool,
    max_tries: u32,
    data: UnsafeCell<T>,
}

impl<T> Stupex<T> {
    pub fn new(data: T) -> Self {
        Self::with_max_tries(data, 100)
    }

    pub(crate) fn with_max_tries(data: T, max_tries: u32) -> Self {
        Self {
            locked: AtomicBool::new(false),
            max_tries,
            data: UnsafeCell::new(data),
        }
    }

    #[allow(unused)]
    pub fn try_lock(&self) -> Result<StupexGuard<'_, T>, WebError> {
        if self.can_acquire_lock() {
            Ok(StupexGuard::new(self))
        } else {
            Err(WebError::custom_error("Already locked"))
        }
    }

    pub async fn lock(&self) -> Result<StupexGuard<'_, T>, WebError> {
        if self.can_acquire_lock() {
            Ok(StupexGuard::new(self))
        } else {
            self.schedule_retry().await
        }
    }

    pub fn can_acquire_lock(&self) -> bool {
        !self.locked.load(Ordering::Relaxed)
    }

    async fn schedule_retry(&self) -> Result<StupexGuard<'_, T>, WebError> {
        let mut i: u32 = 0;
        #[allow(unused_assignments)]
        let mut closure_handle = None;
        while i < self.max_tries {
            let (ch, result) = request_tick_after_timeout(100).await;
            closure_handle = Some(ch);
            if let Err(e) = result {
                error!("{}", WebError::from(e));
            }
            if self.can_acquire_lock() {
                return Ok(StupexGuard::with_closure_handle(self, closure_handle));
            }
            i += 1;
        }

        Err(WebError::custom_error(format!(
            "Could not acquire lock after {} tries",
            self.max_tries
        )))
    }
}

impl<T: ?Sized> DerefMut for StupexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.lock.data.get() }
    }
}

impl<T: ?Sized + fmt::Display> fmt::Display for StupexGuard<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (**self).fmt(f)
    }
}

pub struct StupexGuard<'a, T: ?Sized> {
    /// Keep a reference to the "parent" lock instance
    ///
    /// [`StupexGuard`'s] drop implementation will release the lock inside the "parent" lock
    /// instance
    lock: &'a Stupex<T>,

    /// Keep a reference to the callback object so it won't get dropped
    _closure_handle: Option<ClosureHandle>,
}

impl<'a, T: ?Sized> StupexGuard<'a, T> {
    fn new(lock: &'a Stupex<T>) -> Self {
        lock.locked.store(true, Ordering::Relaxed);

        Self {
            lock,
            _closure_handle: None,
        }
    }

    fn with_closure_handle(lock: &'a Stupex<T>, closure_handle: Option<ClosureHandle>) -> Self {
        lock.locked.store(true, Ordering::Relaxed);

        Self {
            lock,
            _closure_handle: closure_handle,
        }
    }
}

impl<T: ?Sized> Deref for StupexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.lock.data.get() }
    }
}

impl<T: ?Sized + fmt::Debug> fmt::Debug for StupexGuard<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl<T: ?Sized> Drop for StupexGuard<'_, T> {
    #[inline]
    fn drop(&mut self) {
        self.lock.locked.store(false, Ordering::Relaxed)
    }
}

#[cfg(test)]
mod test {
    use wasm_bindgen_test::wasm_bindgen_test_configure;
    use wasm_bindgen_test::*;

    use super::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[derive(PartialEq, Debug)]
    struct TestValue {
        pub age: i32,
        pub name: String,
    }

    #[wasm_bindgen_test]
    async fn try_lock() {
        let sl = Stupex::new(2i8);
        {
            let result_lock1 = sl.try_lock();
            assert!(result_lock1.is_ok());

            let result_lock2 = sl.try_lock();
            assert!(result_lock2.is_err())
        }
        {
            let result_lock3 = sl.try_lock();
            assert!(result_lock3.is_ok());
        }
    }

    #[wasm_bindgen_test]
    async fn try_lock_access() {
        let name = "Grace".to_string();
        let age = 21;

        let sl = Stupex::new(TestValue {
            age,
            name: name.clone(),
        });

        let result_lock1 = sl.try_lock();
        assert!(result_lock1.is_ok(), "{:?}", result_lock1.unwrap_err());
        assert_eq!(*result_lock1.unwrap(), TestValue { age, name });
    }

    #[wasm_bindgen_test]
    async fn lock_access() {
        let name = "Grace".to_string();
        let age = 21;

        let sl = Stupex::with_max_tries(
            TestValue {
                age,
                name: name.clone(),
            },
            2,
        );

        let mut lock_guard_1 = Some(sl.lock().await);
        let (_, result) = request_tick_after_timeout(10).await;
        result.unwrap();
        lock_guard_1.take();

        let result_lock2 = sl.lock().await;
        assert!(result_lock2.is_ok());
        assert_eq!(*result_lock2.unwrap(), TestValue { age, name });
    }
}
