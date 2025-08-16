use alloc::sync::Arc;

use wasm_bindgen_futures::spawn_local;

use wie_backend::System;

// result wrapper holds result of non-send future
pub struct ResultWrapper<T> {
    result: Arc<Option<T>>,
}

impl<T> Clone for ResultWrapper<T> {
    fn clone(&self) -> Self {
        Self { result: self.result.clone() }
    }
}

impl<T> ResultWrapper<T> {
    fn new() -> Self {
        Self { result: Arc::new(None) }
    }

    fn set(&self, value: T) {
        unsafe {
            let result_ptr = Arc::as_ptr(&self.result) as *mut Option<T>;
            result_ptr.replace(Some(value));
        }
    }

    fn is_set(&self) -> bool {
        self.result.is_some()
    }

    pub fn into_inner(self) -> T {
        Arc::into_inner(self.result).unwrap().unwrap()
    }
}

unsafe impl<T> Send for ResultWrapper<T> {}
unsafe impl<T> Sync for ResultWrapper<T> {}

// wrapper to run non-send js future to run on send future
pub fn run_js_future<F, R>(system: &System, f: F) -> impl Future<Output = ResultWrapper<R>> + Sync + Send
where
    F: Future<Output = R> + 'static,
    R: 'static,
{
    let result = ResultWrapper::new();
    let result_clone = result.clone();

    spawn_local(async move { result_clone.set(f.await) });

    async move {
        while !result.is_set() {
            let now = system.platform().now();
            let due = now + 10;
            system.sleep(due).await;
        }

        result
    }
}
