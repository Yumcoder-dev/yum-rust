mod tests {

    #[test]
    fn test_mutex() {
        let m = std::sync::Mutex::new(5);
        {
            let mut num = m.lock().unwrap();
            *num = 6;
        }
        println!("m = {:?}", m.lock().unwrap());
    }

    #[test]
    fn test_mutex_count() {
        let counter = std::sync::Arc::new(std::sync::Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter = std::sync::Arc::clone(&counter);
            let handle = std::thread::spawn(move || {
                let mut num = counter.lock().unwrap();

                *num += 1;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("Result: {}", *counter.lock().unwrap());
    }

    #[test]
    fn test_shared_memory_thread() {}
}
