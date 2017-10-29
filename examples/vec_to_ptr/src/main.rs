extern crate libc;

use libc::uint8_t;

fn main() {
    let v: Vec<u8> = vec![1, 2, 3, 4];

    // Option 1: Shrinking our vector
    let mut v1 = v.clone();
    let v1_len = v1.len();
    v1.shrink_to_fit();
    assert_eq!(v1.len(), v1.capacity());
    let v1_ptr: *const uint8_t = v1.as_ptr();
    std::mem::forget(v1);

    // Option 2: Use `into_boxed_slice`
    let v2 = v.clone();
    let v2_len = v2.len();
    let v2_box: Box<[u8]> = v2.into_boxed_slice();
    let v2_ptr: *const [uint8_t] = Box::into_raw(v2_box);
}
