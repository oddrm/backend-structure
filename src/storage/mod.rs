pub mod metadata;
pub mod sequence;
pub mod storage_instance;

/*
TODO
notify crates poll watcher rescans file system everytime,
uses walkdir, hashes whole file, also uses update time for comparison.
Probably needs modified Pollwatcher implementation. https://github.com/notify-rs/notify/blob/b912ce5400010eab383180c69df798e991e9b922/notify/src/poll.rs
*/
