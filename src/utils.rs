pub fn get_timestamp_n_hours_from_now(n: u64) -> u64 {
    let now = std::time::SystemTime::now();
    let since_epoch = now
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time goes back");
    (since_epoch + std::time::Duration::from_secs(n * 60 * 60)).as_secs()
}
