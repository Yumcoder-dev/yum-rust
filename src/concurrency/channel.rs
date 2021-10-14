mod test {

    #[test]
    fn test_channel_thread() {
        let (tx, rx) = crossbeam::channel::unbounded();
        std::thread::spawn(move || {
            tx.send(42).unwrap();
        });
        // Macros can define their own syntax rules.
        select! {
            // recv(rx) is syntax defined by the macro.
            recv(rx) -> msg => println!("{:?}", msg),
        }
    }

    #[test]
    fn test_channel_ping_pong_thread() {
        #[derive(Debug)]
        enum ConnectivityCheck {
            Ping,
            Pong,
            Pang,
        }

        let n_messages = 3;
        let (requests_tx, requests_rx) = crossbeam::channel::unbounded();
        let (responses_tx, responses_rx) = crossbeam::channel::unbounded();

        std::thread::spawn(move || loop {
            match requests_rx.recv().unwrap() {
                ConnectivityCheck::Pong => eprintln!("unexpected pong response"),
                ConnectivityCheck::Ping => responses_tx.send(ConnectivityCheck::Pong).unwrap(),
                ConnectivityCheck::Pang => return,
            }
        });

        for _ in 0..n_messages {
            requests_tx.send(ConnectivityCheck::Ping).unwrap();
        }
        requests_tx.send(ConnectivityCheck::Pang).unwrap();

        for _ in 0..n_messages {
            select! {
               recv(responses_rx) -> msg => println!("{:?}", msg),
            }
        }
    }
}
