use criterion::{criterion_group, criterion_main, Criterion};
use rand::prelude::*;
use rand_pcg::Pcg32;
use simd_pcg::Avx2Pcg;

const N: usize = 10000000;

pub fn avx_one(c: &mut Criterion) {
    c.bench_function("avx one", |b| {
        b.iter(|| {
            let mut arr = vec![0u32; N];

            let mut rng1 = Avx2Pcg::from_entropy();

            for i in (0..N).step_by(4) {
                let r1_arr = rng1.next_u32x4();
                arr[i..(i + 4)].copy_from_slice(&r1_arr);
            }
        })
    });
}

pub fn avx_two(c: &mut Criterion) {
    c.bench_function("avx two", |b| {
        b.iter(|| {
            let mut arr = vec![0u32; N];

            let mut rng1 = Avx2Pcg::from_entropy();
            let mut rng2 = Avx2Pcg::from_entropy();

            for i in (0..N).step_by(8) {
                let r1_arr = rng1.next_u32x4();
                let r2_arr = rng2.next_u32x4();
                arr[i..(i + 4)].copy_from_slice(&r1_arr);
                arr[(i + 4)..(i + 8)].copy_from_slice(&r2_arr);
            }
        })
    });
}

pub fn avx_four(c: &mut Criterion) {
    c.bench_function("avx four", |b| {
        b.iter(|| {
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
        })
    });
}

pub fn baseline_one(c: &mut Criterion) {
    c.bench_function("baseline one", |b| {
        b.iter(|| {
            let mut arr = vec![0u32; N];
            let mut rng1 = Pcg32::from_entropy();
            for i in (0..N).step_by(1) {
                arr[i] = rng1.gen();
            }
        })
    });
}

pub fn baseline_two(c: &mut Criterion) {
    c.bench_function("baseline two", |b| {
        b.iter(|| {
            let mut arr = vec![0u32; N];
            let mut rng1 = Pcg32::from_entropy();
            let mut rng2 = Pcg32::from_entropy();
            for i in (0..N).step_by(2) {
                arr[i] = rng1.gen();
                arr[i + 1] = rng2.gen();
            }
        })
    });
}

pub fn baseline_four(c: &mut Criterion) {
    c.bench_function("baseline four", |b| {
        b.iter(|| {
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
        })
    });
}

/*

fn main() {
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
}
*/

criterion_group!(
    benches,
    avx_one,
    avx_two,
    avx_four,
    baseline_one,
    baseline_two,
    baseline_four
);
criterion_main!(benches);
