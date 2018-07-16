#![feature(extern_types)]
#![feature ( const_slice_as_ptr , extern_types , libc )]
#![feature(extern_prelude)]

#[cfg(feature="sys")]
extern crate speexdsp_sys;
extern crate libc;

pub mod resampler;
#[cfg(not(feature="sys"))]
mod speex_resample;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
