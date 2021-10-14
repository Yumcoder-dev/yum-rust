mod test {
    #[test]
    fn test_spawning_thread() {
        let start = std::time::Instant::now();
        let handler = std::thread::spawn(|| {
            let pause = std::time::Duration::from_millis(300);
            std::thread::sleep(pause.clone());
        });

        handler.join().unwrap();

        let finish = std::time::Instant::now();

        println!("{:02?}", finish.duration_since(start));
    }
}
