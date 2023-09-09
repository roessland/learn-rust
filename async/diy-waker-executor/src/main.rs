use std::future::Future;
use std::pin::pin;
use std::task;
use std::task::RawWakerVTable;
use std::time::Duration;

fn clone_fn(ptr: *const ()) -> task::RawWaker {
    println!("clone_fn");
    task::RawWaker::new(ptr, &VTABLE)
}

const VTABLE: RawWakerVTable = RawWakerVTable::new(
    clone_fn,
    |_ptr| {
        println!("waek");
    },
    |_ptr| {},
    |_ptr| {},
);

fn main() {
    let hello_fut = hello();
    let mut pin_fut = pin!(hello_fut);

    let raw_waker = task::RawWaker::new(std::ptr::null(), &VTABLE);
    let waker = unsafe { task::Waker::from_raw(raw_waker) };
    let mut context = task::Context::from_waker(&waker);

    let mut poll_result = std::task::Poll::Pending;
    while poll_result == std::task::Poll::Pending {
        poll_result = pin_fut.as_mut().poll(&mut context);
        println!("poll_result: {:?}", poll_result);
    }
}

pub trait Runnable {
    fn run(&self);
}

struct PhfredFutoor {}

impl PhfredFutoor {
    fn new(runnable: Box<dyn Runnable>) -> Self {
        PhfredFutoor {}
    }
}

impl Future for PhfredFutoor {
    type Output = ();

    fn poll(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut task::Context<'_>,
    ) -> task::Poll<Self::Output> {
        std::task::Poll::Pending
    }
}

struct MyFootu {
    polled: bool,
}

impl MyFootu {
    fn new() -> Self {
        MyFootu { polled: false }
    }
}

impl Future for MyFootu {
    type Output = ();

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        _cx: &mut task::Context<'_>,
    ) -> task::Poll<Self::Output> {
        if !self.polled {
            println!("ha haaa!");
            self.polled = true;
            std::task::Poll::Pending
        } else {
            std::task::Poll::Ready(())
        }
    }
}

async fn hello() {
    println!("Hello, world!");
    MyFootu::new().await;
    bob().await;
    urecked();
    bob().await;
    bob().await;
}

async fn bob() {
    println!("Bob, world!");
}

fn urecked() {
    let body: String = ureq::get("http://example.com")
        .set("Example-Header", "header value")
        .call()
        .expect("edoisj")
        .into_string()
        .expect("pleas");
    println!("{}", body);
}
