use std::time::Duration;
use std::thread::sleep;

pub fn sleep_millis(millis: u64) {
    sleep(Duration::from_millis(millis));
}

pub fn xor_encrypt_decrypt(key: &[u8], data: &mut [u8]) {
    for (data_elem, key_elem) in data.iter_mut().zip(key.iter().cycle()) {
        *data_elem ^= key_elem;
    }
}
