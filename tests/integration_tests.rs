use miopen_sys::{
    miopenHandle_t,
    miopenStatus_t,
    miopenCreate,
    miopenDestroy,
};

#[test]
fn test_miopen_create_with_stream() {
    let mut miopen_handle = std::ptr::null_mut();
    let status = unsafe {
        miopenCreate(&mut miopen_handle as *mut miopenHandle_t)
    };
    assert_eq!(status, miopenStatus_t::miopenStatusSuccess);
    let status = unsafe {
        miopenDestroy(miopen_handle)
    };
    assert_eq!(status, miopenStatus_t::miopenStatusSuccess);
}
