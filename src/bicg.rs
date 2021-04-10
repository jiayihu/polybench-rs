#![feature(min_const_generics)]
#![allow(non_snake_case)]

fn init_array<const M: usize, const N: usize>(
    m: usize,
    n: usize,
    A: &mut [[f32; N]; M],
    r: &mut [f32; M],
    p: &mut [f32; N],
) {
    for i in 0..n {
        p[i] = (i % n) as f32 / n as f32;
    }
    for i in 0..m {
        r[i] = (i % m) as f32 / m as f32;
        for j in 0..n {
            A[i][j] = (i * (j + 1) % m) as f32 / m as f32;
        }
    }
}

fn kernel_bicg<const M: usize, const N: usize>(
    m: usize,
    n: usize,
    A: &[[f32; N]; M],
    s: &mut [f32; N],
    q: &mut [f32; M],
    p: &[f32; N],
    r: &[f32; M],
) {
    for i in 0..n {
        s[i] = 0.0;
    }
    for i in 0..m {
        q[i] = 0.0;
        for j in 0..n {
            s[j] = s[j] + r[i] * A[i][j];
            q[i] = q[i] + A[i][j] * p[j];
        }
    }
}
#[no_mangle]
pub extern "C" fn bench() {
    const M: usize = 10;
    const N: usize = 10;

    let mut A = [[0_f32; N]; M];
    let mut s = [0_f32; N];
    let mut q = [0_f32; M];
    let mut p = [0_f32; N];
    let mut r = [0_f32; M];

    init_array(M, N, &mut A, &mut r, &mut p);
    kernel_bicg(M, N, &A, &mut s, &mut q, &p, &r);
}
