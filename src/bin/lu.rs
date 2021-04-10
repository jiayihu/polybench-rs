#![feature(min_const_generics)]
#![no_std]
#![no_main]
#![allow(non_snake_case)]

use core::panic::PanicInfo;

fn make_positive_semi_definite<const N: usize>(A: &mut [[f32; N]; N]) {
    let mut b = [[0_f32; N]; N];
    let n = N;

    for t in 0..n {
        for r in 0..n {
            for s in 0..n {
                b[r][s] += A[r][t] * A[s][t];
            }
        }
    }
    for r in 0..n {
        for s in 0..n {
            A[r][s] = b[r][s];
        }
    }
}

fn init_array<const N: usize>(n: usize, A: &mut [[f32; N]; N]) {
    for i in 0..n {
        for j in 0..=i {
            A[i][j] = (-(j as isize) % n as isize) as f32 / n as f32 + 1.0;
        }
        for j in (i + 1)..n {
            A[i][j] = 0.0;
        }
        A[i][i] = 1.0;
    }

    make_positive_semi_definite(A);
}

fn kernel_lu<const N: usize>(n: usize, A: &mut [[f32; N]; N]) {
    for i in 0..n {
        for j in 0..i {
            for k in 0..j {
                A[i][j] -= A[i][k] * A[k][j];
            }
            A[i][j] /= A[j][j];
        }
        for j in i..n {
            for k in 0..i {
                A[i][j] -= A[i][k] * A[k][j];
            }
        }
    }
}
#[no_mangle]
fn start() {
    const N: usize = 10;

    let mut A = [[0_f32; N]; N];

    init_array(N, &mut A);
    kernel_lu(N, &mut A);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
