#![feature(portable_simd)]
#![allow(dead_code)]
use rand::prelude::*;
use rand::Rng;
use rand_pcg::Pcg32;
use simd_pcg::Avx2Pcg;
use std::time::{Duration, Instant};

fn bench<F: Fn()>(f: F) -> Duration {
    let start = Instant::now();
    f();
    start.elapsed()
}

fn main() {
    const N: usize = 500000000;

    let avx_f1 = || {
        let mut arr = vec![0u32; N];

        let mut rng1 = Avx2Pcg::from_entropy();

        for i in (0..N).step_by(4) {
            let r1_arr = rng1.next_u32x4();
            arr[i..(i + 4)].copy_from_slice(&r1_arr);
        }
    };
    println!("avx2 one: {:?}", bench(avx_f1));

    let avx_f2 = || {
        let mut arr = vec![0u32; N];

        let mut rng1 = Avx2Pcg::from_entropy();
        let mut rng2 = Avx2Pcg::from_entropy();

        for i in (0..N).step_by(8) {
            let r1_arr = rng1.next_u32x4();
            let r2_arr = rng2.next_u32x4();
            arr[i..(i + 4)].copy_from_slice(&r1_arr);
            arr[(i + 4)..(i + 8)].copy_from_slice(&r2_arr);
        }
    };
    println!("avx2 two: {:?}", bench(avx_f2));

    let avx_f4 = || {
        let mut arr = vec![0u32; N];

        let mut rng1 = Avx2Pcg::from_entropy();
        let mut rng2 = Avx2Pcg::from_entropy();
        let mut rng3 = Avx2Pcg::from_entropy();
        let mut rng4 = Avx2Pcg::from_entropy();

        for i in (0..N).step_by(16) {
            let r1_arr = rng1.next_u32x4();
            let r2_arr = rng2.next_u32x4();
            let r3_arr = rng3.next_u32x4();
            let r4_arr = rng4.next_u32x4();
            arr[i..(i + 4)].copy_from_slice(&r1_arr);
            arr[(i + 4)..(i + 8)].copy_from_slice(&r2_arr);
            arr[(i + 8)..(i + 12)].copy_from_slice(&r3_arr);
            arr[(i + 12)..(i + 16)].copy_from_slice(&r4_arr);
        }
    };
    println!("avx2 four: {:?}", bench(avx_f4));

    let avx_f8 = || {
        let mut arr = vec![0u32; N];

        let mut rng1 = Avx2Pcg::from_entropy();
        let mut rng2 = Avx2Pcg::from_entropy();
        let mut rng3 = Avx2Pcg::from_entropy();
        let mut rng4 = Avx2Pcg::from_entropy();
        let mut rng5 = Avx2Pcg::from_entropy();
        let mut rng6 = Avx2Pcg::from_entropy();
        let mut rng7 = Avx2Pcg::from_entropy();
        let mut rng8 = Avx2Pcg::from_entropy();

        for i in (0..N).step_by(32) {
            let r1_arr = rng1.next_u32x4();
            let r2_arr = rng2.next_u32x4();
            let r3_arr = rng3.next_u32x4();
            let r4_arr = rng4.next_u32x4();
            let r5_arr = rng5.next_u32x4();
            let r6_arr = rng6.next_u32x4();
            let r7_arr = rng7.next_u32x4();
            let r8_arr = rng8.next_u32x4();
            arr[i..(i + 4)].copy_from_slice(&r1_arr);
            arr[(i + 4)..(i + 8)].copy_from_slice(&r2_arr);
            arr[(i + 8)..(i + 12)].copy_from_slice(&r3_arr);
            arr[(i + 12)..(i + 16)].copy_from_slice(&r4_arr);
            arr[(i + 16)..(i + 20)].copy_from_slice(&r5_arr);
            arr[(i + 20)..(i + 24)].copy_from_slice(&r6_arr);
            arr[(i + 24)..(i + 28)].copy_from_slice(&r7_arr);
            arr[(i + 28)..(i + 32)].copy_from_slice(&r8_arr);
        }
    };
    println!("avx2 eight: {:?}", bench(avx_f8));

    let baseline_f1 = || {
        let mut arr = vec![0u32; N];
        let mut rng1 = Pcg32::from_entropy();
        for i in (0..N).step_by(1) {
            arr[i] = rng1.gen();
        }
    };
    println!("baseline one: {:?}", bench(baseline_f1));

    let baseline_f2 = || {
        let mut arr = vec![0u32; N];
        let mut rng1 = Pcg32::from_entropy();
        let mut rng2 = Pcg32::from_entropy();
        for i in (0..N).step_by(2) {
            arr[i] = rng1.gen();
            arr[i + 1] = rng2.gen();
        }
    };
    println!("baseline two: {:?}", bench(baseline_f2));

    let baseline_f4 = || {
        let mut arr = vec![0u32; N];
        let mut rng1 = Pcg32::from_entropy();
        let mut rng2 = Pcg32::from_entropy();
        let mut rng3 = Pcg32::from_entropy();
        let mut rng4 = Pcg32::from_entropy();
        for i in (0..N).step_by(4) {
            arr[i] = rng1.gen();
            arr[i + 1] = rng2.gen();
            arr[i + 2] = rng3.gen();
            arr[i + 3] = rng4.gen();
        }
    };
    println!("baseline four: {:?}", bench(baseline_f4));

    let baseline_f8 = || {
        let mut arr = vec![0u32; N];
        let mut rng1 = Pcg32::from_entropy();
        let mut rng2 = Pcg32::from_entropy();
        let mut rng3 = Pcg32::from_entropy();
        let mut rng4 = Pcg32::from_entropy();
        let mut rng5 = Pcg32::from_entropy();
        let mut rng6 = Pcg32::from_entropy();
        let mut rng7 = Pcg32::from_entropy();
        let mut rng8 = Pcg32::from_entropy();
        for i in (0..N).step_by(8) {
            arr[i] = rng1.gen();
            arr[i + 1] = rng2.gen();
            arr[i + 2] = rng3.gen();
            arr[i + 3] = rng4.gen();
            arr[i + 4] = rng5.gen();
            arr[i + 5] = rng6.gen();
            arr[i + 6] = rng7.gen();
            arr[i + 7] = rng8.gen();
        }
    };
    println!("baseline eight: {:?}", bench(baseline_f8));
}
