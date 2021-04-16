#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub unsafe fn convert(buf: &[u8], w: u32, h: u32, dst: &mut [u8]) {
    assert_eq!(yuv_sys::I420ToRGBA(
        buf.as_ptr(),
        w as i32,
        buf.as_ptr().offset((w * h) as isize),
        (w / 2) as i32,
        buf.as_ptr().offset(((w * h * 3) / 2) as isize),
        (w / 2) as i32,
        dst.as_mut_ptr(),
        (w * 4) as i32,
        w as i32, h as i32,
    ), 0);
}