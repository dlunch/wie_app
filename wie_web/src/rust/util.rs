use alloc::sync::Arc;
use core::{
    cell::UnsafeCell,
    future::Future,
    pin::Pin,
    sync::atomic::{AtomicBool, Ordering},
    task::{Context, Poll, Waker},
};

use wasm_bindgen_futures::spawn_local;

// Send/Sync single-slot channel driven by a Waker. Works on single-threaded
// wasm where the sender and the receiver run on the same thread but the
// outer future needs to satisfy `Send` (async_trait default bound).
struct OneshotInner<T> {
    value: UnsafeCell<Option<T>>,
    waker: UnsafeCell<Option<Waker>>,
    set: AtomicBool,
}

unsafe impl<T> Send for OneshotInner<T> {}
unsafe impl<T> Sync for OneshotInner<T> {}

struct Sender<T> {
    inner: Arc<OneshotInner<T>>,
}

pub struct Receiver<T> {
    inner: Arc<OneshotInner<T>>,
}

fn oneshot<T>() -> (Sender<T>, Receiver<T>) {
    let inner = Arc::new(OneshotInner {
        value: UnsafeCell::new(None),
        waker: UnsafeCell::new(None),
        set: AtomicBool::new(false),
    });
    (Sender { inner: inner.clone() }, Receiver { inner })
}

impl<T> Sender<T> {
    fn send(self, value: T) {
        unsafe { *self.inner.value.get() = Some(value) };
        self.inner.set.store(true, Ordering::Release);
        if let Some(waker) = unsafe { (*self.inner.waker.get()).take() } {
            waker.wake();
        }
    }
}

impl<T> Future for Receiver<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T> {
        if self.inner.set.load(Ordering::Acquire) {
            let value = unsafe { (*self.inner.value.get()).take() };
            return Poll::Ready(value.unwrap());
        }
        unsafe { *self.inner.waker.get() = Some(cx.waker().clone()) };
        if self.inner.set.load(Ordering::Acquire) {
            let value = unsafe { (*self.inner.value.get()).take() };
            return Poll::Ready(value.unwrap());
        }
        Poll::Pending
    }
}

// wrapper to run non-send js future from a send future
pub fn run_js_future<F, R>(f: F) -> impl Future<Output = R> + Send + Sync
where
    F: Future<Output = R> + 'static,
    R: 'static,
{
    let (tx, rx) = oneshot();
    spawn_local(async move { tx.send(f.await) });
    rx
}
