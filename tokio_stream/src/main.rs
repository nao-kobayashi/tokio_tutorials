use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};
use tokio_stream::{Stream, StreamExt};

struct Delay {
    when: Instant,
}

impl Future for Delay {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<&'static str> {
        if Instant::now() >= self.when {
            //println!("Hello World");
            Poll::Ready("done")
        } else {
            //println!("yet...");
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

struct Interval {
    rem: usize,
    delay: Delay,
}

impl Stream for Interval {
    type Item = ();

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<()>> {
        if self.rem == 0 {
            return Poll::Ready(None);
        }

        match Pin::new(&mut self.delay).poll(cx) {
            Poll::Ready(_) => {
                self.delay = Delay { when: Instant::now() + Duration::from_millis(300)};
                self.rem -= 1;
                Poll::Ready(Some(()))
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

#[tokio::main]
async fn main() {
    let mut stream = Interval {
        rem: 10,
        delay: Delay {
            when: Instant::now() + Duration::from_millis(100),
        },
    };

    while let Some(_) = stream.next().await {
        println!("val {:?}", stream.rem);
    }
}

/*
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    let mut stream = tokio_stream::iter(&[1, 2 ,3]);

    while let Some(v) = stream.next().await {
        println!("got {:?}", v);
    }
}
*/
