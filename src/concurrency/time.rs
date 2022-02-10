mod test {
    use std::time::Duration;
    use tokio::time::{sleep, Instant};

    #[tokio::test]
    async fn my_test() {
        let now = Instant::now();
        sleep(Duration::from_millis(100)).await;
        //sleep(Duration::new(1, 0)).await;
        let new_now = Instant::now();

        // println!("100 ms have elapsed");
        // println!("{:?}", new_now.checked_duration_since(now));
        assert_eq!(new_now.checked_duration_since(now).unwrap() > Duration::from_millis(100), true)
    }
}
