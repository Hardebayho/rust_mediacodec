#[repr(C)]
pub struct AMediaCrypto {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}
