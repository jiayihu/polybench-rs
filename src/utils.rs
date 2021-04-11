pub fn make_positive_semi_definite<const N: usize>(A: &mut [[f32; N]; N]) {
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