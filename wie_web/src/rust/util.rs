use alloc::sync::Arc;
use core::{
    future::{Future, poll_fn},
    task::Poll,
};

use wasm_bindgen_futures::spawn_local;

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

// yields to the executor once; wakes itself so it is re-polled on the
// next microtask. Send because it holds no state that references JS.
async fn yield_once() {
    let mut polled = false;
    poll_fn(|cx| {
        if polled {
            Poll::Ready(())
        } else {
            polled = true;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    })
    .await
}

// wrapper to run non-send js future from a send future
pub fn run_js_future<F, R>(f: F) -> impl Future<Output = ResultWrapper<R>> + Sync + Send
where
    F: Future<Output = R> + 'static,
    R: 'static,
{
    let result = ResultWrapper::new();
    let result_clone = result.clone();

    spawn_local(async move { result_clone.set(f.await) });

    async move {
        while !result.is_set() {
            yield_once().await;
        }

        result
    }
}
