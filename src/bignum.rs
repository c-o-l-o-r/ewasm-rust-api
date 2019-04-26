/// The bignum system library.
extern "C" {
    pub fn bignum_mul256(a: *const u32, b: *const u32, ret: *mut u32);
    pub fn bignum_umulmod256(a: *const u32, b: *const u32, modulo: *const u32, ret: *mut u32);
}
