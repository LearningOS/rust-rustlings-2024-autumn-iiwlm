// tests6.rs
//
// In this example we take a shallow dive into the Rust standard library's
// unsafe functions. Fix all the question marks and todos to make the test
// pass.
//
// Execute `rustlings hint tests6` or use the `hint` watch subcommand for a
// hint.

struct Foo {
    a: u128,
    b: Option<String>,
}

/// # Safety
///
/// The `ptr` must contain an owned box of `Foo`.
/// The caller must ensure that the pointer is valid and points to a `Box<Foo>` that was previously obtained via `Box::into_raw`.
unsafe fn raw_pointer_to_box(ptr: *mut Foo) -> Box<Foo> {
    // SAFETY: The `ptr` contains an owned box of `Foo` by contract. We
    // simply reconstruct the box from that pointer.
    let ret: Box<Foo> = unsafe { Box::from_raw(ptr) };
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_success() {
        let data = Box::new(Foo { a: 1, b: Some(String::from("hello")) });

        let ptr_1 = (&*data) as *const Foo as usize;
        // SAFETY: We pass an owned box of `Foo`.
        let ret = unsafe { raw_pointer_to_box(Box::into_raw(data)) };

        let ptr_2 = (&*ret) as *const Foo as usize;

        assert_eq!(ptr_1, ptr_2);
        assert_eq!(ret.b, Some(String::from("hello")));
    }
}