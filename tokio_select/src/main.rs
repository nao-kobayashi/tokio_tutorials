use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let wait01 = std::time::Duration::from_millis(1000);
    let wait02 = std::time::Duration::from_millis(700);

    let (tx1, mut rx1) = mpsc::channel(32);
    let (tx2, mut rx2) = mpsc::channel(32);

    tokio::spawn(async move {
        //tokio::time::sleep(wait01).await;
        for i in 0..10 {
            let _ = tx1.send(format!("one {}", i)).await;
            tokio::time::sleep(wait01).await;
        }
    });

    tokio::spawn(async move {
        tokio::time::sleep(wait02).await;
        for i in 0..10 {
            let _ = tx2.send(format!("two {}", i)).await;
            tokio::time::sleep(wait02).await;
        }
    });

    loop {
        let msg = tokio::select! {
            Some(val) = rx1.recv() => val,
            Some(val) = rx2.recv() => val,
            else => { break }
        };

        println!("Got message => {:?}", msg);
    }

/*    loop {
        tokio::select! {
            Some(val) = rx1.recv() => {
                println!("rx1 completed with {:?}", val);
            }
            Some(val) = rx2.recv() => {
                println!("rx2 completed with {:?}", val);
            }
            else => { break }
        }
    }*/
}


/*use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
    let (tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();

    tokio::spawn(async {
        let _ = tx1.send("one");
    });

    tokio::spawn(async {
        let _ = tx2.send("two");
    });

    tokio::select! {
        val = rx1 => {
            println!("rx1 completed with {:?}", val);
        }
        val = rx2 => {
            println!("rx2 completed with {:?}", val);
        }
    }
}*/