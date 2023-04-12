use cside::BoxedSlice;

pub mod cside;
pub mod rustside;

#[no_mangle]
pub extern "C" fn _tmp() -> BoxedSlice<()> {
    todo!()
}
