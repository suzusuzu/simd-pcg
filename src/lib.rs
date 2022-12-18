#![feature(portable_simd)]
#![allow(dead_code)]
use rand::{Error, RngCore};

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;
use std::simd::{u32x4, u64x2};

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub struct Avx2Pcg {
    state: __m256i,
    inc: __m256i,
    mul_l: __m256i,
    mul_h: __m256i,
}

const MULTIPLIER: i64 = 6364136223846793005;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "avx2")]
#[inline]
unsafe fn _mm256_mullo_epi64(x: __m256i, ml: __m256i, mh: __m256i) -> __m256i {
    let xl = _mm256_and_si256(x, _mm256_set1_epi64x(0x00000000ffffffff));
    let xh = _mm256_srli_epi64(x, 32);
    let hl = _mm256_slli_epi64(_mm256_mul_epu32(xh, ml), 32);
    let lh = _mm256_slli_epi64(_mm256_mul_epu32(xl, mh), 32);
    let ll = _mm256_mul_epu32(xl, ml);
    let ret = _mm256_add_epi64(ll, _mm256_add_epi64(hl, lh));
    return ret;
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "avx2")]
#[inline]
unsafe fn _mm256_rorv_epi32(x: __m256i, r: __m256i) -> __m256i {
    let ret = _mm256_or_si256(
        _mm256_sllv_epi32(x, _mm256_sub_epi32(_mm256_set1_epi32(32), r)),
        _mm256_srlv_epi32(x, r),
    );
    return ret;
}

impl Avx2Pcg {
    #[inline]
    fn next(&mut self) -> __m128i {
        unsafe { self.next_() }
    }

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    #[target_feature(enable = "avx2")]
    #[inline]
    unsafe fn next_(&mut self) -> __m128i {
        let old_state = self.state;

        self.state = _mm256_add_epi64(
            _mm256_mullo_epi64(self.state, self.mul_l, self.mul_h),
            self.inc,
        );

        let xorshifted = _mm256_srli_epi64(
            _mm256_xor_si256(_mm256_srli_epi64(old_state, 18), old_state),
            27,
        );
        let rot = _mm256_srli_epi64(old_state, 59);

        let ret = _mm256_castsi256_si128(_mm256_permutevar8x32_epi32(
            _mm256_rorv_epi32(xorshifted, rot),
            _mm256_set_epi32(7, 7, 7, 7, 6, 4, 2, 0),
        ));

        return ret;
    }

    #[inline]
    pub fn next_u32x4(&mut self) -> [u32; 4] {
        let m128: u32x4 = self.next().into();
        return *m128.as_array();
    }

    #[inline]
    pub fn from_state_inc(state: [i64; 4], inc: [i64; 4]) -> Avx2Pcg {
        unsafe {
            Avx2Pcg {
                state: _mm256_set_epi64x(state[0], state[1], state[2], state[3]),
                inc: _mm256_set_epi64x(inc[0] | 1, inc[1] | 1, inc[2] | 1, inc[3] | 1),
                mul_l: _mm256_set1_epi64x(MULTIPLIER & 0x00000000ffffffff),
                mul_h: _mm256_set1_epi64x(MULTIPLIER >> 32),
            }
        }
    }

    #[inline]
    pub fn from_entropy() -> Avx2Pcg {
        let mut state_buf: [u8; 32] = [0u8; 32];
        let mut inc_buf: [u8; 32] = [0u8; 32];
        let _ = getrandom::getrandom(&mut state_buf);
        let _ = getrandom::getrandom(&mut inc_buf);
        let state_buf64: [i64; 4] = unsafe { std::mem::transmute(state_buf) };
        let inc_buf64: [i64; 4] = unsafe { std::mem::transmute(inc_buf) };
        Avx2Pcg::from_state_inc(state_buf64, inc_buf64)
    }
}

impl RngCore for Avx2Pcg {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        let m128: u32x4 = self.next().into();
        return m128.as_array()[0];
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        let m128: u64x2 = self.next().into();
        return m128.as_array()[0];
    }

    #[inline]
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        let mut left = dest;

        while left.len() >= 16 {
            let (l, r) = { left }.split_at_mut(16);
            left = r;
            let chunk: [u8; 16] = unsafe { std::mem::transmute(self.next_u32x4()) };
            l.copy_from_slice(&chunk);
        }

        let n = left.len();

        let chunk: [u8; 16] = unsafe { std::mem::transmute(self.next_u32x4()) };
        left.copy_from_slice(&chunk[..n]);
    }

    #[inline]
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        Ok(self.fill_bytes(dest))
    }
}
