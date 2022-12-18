use alloc::vec::Vec;
use core::ptr::slice_from_raw_parts_mut;
use core::str::FromStr;
use uefi::CStr16;
use crate::allocator::{free, malloc};

pub struct CString16 {
    vec: Vec<u16>,
}

impl CString16 {
    pub fn from_u16_vec(vec: Vec<u16>) -> Self {
        Self {
            vec
        }
    }

    pub fn to_cstr16(&self) -> &CStr16 {
        unsafe {
            CStr16::from_u16_with_nul_unchecked(self.vec.as_slice())
        }
    }
}

impl FromStr for CString16 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vec = str_to_u16_vec(s);
        Ok(Self {
            vec
        })
    }
}

fn str_to_u16_vec(str: &str) -> Vec<u16> {
    unsafe {
        let size = str.encode_utf16().count();
        let buf = malloc::<u16>(size * 2 + 2);
        let buf_ref = slice_from_raw_parts_mut(buf, size + 1).as_mut().unwrap();
        let vec = CStr16::from_str_with_buf(str, buf_ref)
            .unwrap().to_u16_slice_with_nul().to_vec();
        free(buf, size * 2 + 2);
        vec
    }
}
