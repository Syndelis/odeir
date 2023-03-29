use std::{panic::UnwindSafe, ffi::c_int};

pub fn catch_panic(f: impl FnOnce() + UnwindSafe) -> c_int {
    result_to_int(std::panic::catch_unwind(f))
}

pub(crate) fn result_to_int<T, E>(result: Result<T, E>) -> c_int {
    match result {
        Ok(_) => 1,
        Err(_) => 0,
    }
}

pub(crate) fn vec_to_ptr<T, U>(vec: Vec<T>) -> (*mut U, usize)
where
    T: Into<U>,
{
    let data = vec
        .into_iter()
        .map(|el: T| el.into())
        .collect::<Vec<U>>()
        .into_boxed_slice();
    let data = Box::leak(data);
    (data.as_mut_ptr(), data.len())
}
