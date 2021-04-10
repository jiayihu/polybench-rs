#![feature(min_const_generics)]
#![no_std]
#![no_main]
#![allow(non_snake_case)]
use core::panic::PanicInfo;

fn init_array(n: usize, A: &mut [[f32; 50]; 50]) {
    for i in 0..n {
        for j in 0..n {
            A[i][j] = (i * (j + 2) + 2) as f32 / n as f32;
        }
    }
}

fn kernel_seidel_2d(tsteps: usize, n: usize, A: &mut [[f32; 50]; 50]) {
    for _ in 0..tsteps {
        for i in 1..(n - 1) {
            for j in 1..(n - 1) {
                A[i][j] = (A[i - 1][j - 1]
                    + A[i - 1][j]
                    + A[i - 1][j + 1]
                    + A[i][j - 1]
                    + A[i][j]
                    + A[i][j + 1]
                    + A[i + 1][j - 1]
                    + A[i + 1][j]
                    + A[i + 1][j + 1])
                    / 9.0;
            }
        }
    }
}

#[no_mangle]
fn start() {
    const N: usize = 50;
    const TSTEPS: usize = 12;

    let mut A = [[0_f32; N]; N];

    init_array(N, &mut A);
    kernel_seidel_2d(TSTEPS, N, &mut A)
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
