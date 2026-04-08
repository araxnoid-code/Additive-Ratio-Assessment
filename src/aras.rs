pub enum TipeKategori {
    BENEFIT,
    COST,
}

pub fn aras_start<const ALTERNATIVE: usize, const KATEGORI: usize>(
    alternative: [&'static str; ALTERNATIVE],
    data: [[f64; KATEGORI]; ALTERNATIVE],
    tipe_kategori: [TipeKategori; KATEGORI],
    bobot: [f64; KATEGORI],
) -> (Vec<f64>, Vec<&'static str>) {
    let data = data.to_vec();
    // Alternative Optimal
    let mut alternative_optimal_raw: [Option<f64>; KATEGORI] = [None; KATEGORI];
    let panjang_kolom = data.len();
    for k in 0..KATEGORI {
        for i in 0..panjang_kolom {
            let x = data[i][k];
            if let Some(value) = &mut alternative_optimal_raw[k] {
                match tipe_kategori[k] {
                    TipeKategori::COST => *value = value.min(x),
                    TipeKategori::BENEFIT => *value = value.max(x),
                }
            } else {
                alternative_optimal_raw[k] = Some(x);
            }
        }
    }
    let alternative_optimal = alternative_optimal_raw.map(|x| x.unwrap());

    let mut data_concat = [vec![alternative_optimal], data].concat();

    // Matrix Ternormalisasi
    let m = data_concat.len();
    let n = KATEGORI;
    let mut sum_normalize: [f64; KATEGORI] = [0.; KATEGORI];

    for row in 0..m {
        for coll in 0..n {
            let x = data_concat[row][coll];
            match tipe_kategori[coll] {
                TipeKategori::COST => sum_normalize[coll] += 1.0 / x,
                TipeKategori::BENEFIT => sum_normalize[coll] += x,
            };
        }
    }

    for row in 0..m {
        for coll in 0..n {
            match tipe_kategori[coll] {
                TipeKategori::COST => {
                    data_concat[row][coll] = (1.0 / data_concat[row][coll]) / sum_normalize[coll]
                }
                TipeKategori::BENEFIT => {
                    data_concat[row][coll] = data_concat[row][coll] / sum_normalize[coll]
                }
            };
        }
    }

    // BOBOT MATRIX TERNORMALISASI
    for row in 0..m {
        for coll in 0..n {
            data_concat[row][coll] *= bobot[coll]
        }
    }

    // NILAI FUNGSI OPTIMUN
    let fungsi_optimum = data_concat
        .iter()
        .map(|row| row.iter().sum::<f64>())
        .collect::<Vec<f64>>();

    // prioritas kelayakan
    let s0 = fungsi_optimum[0];
    let kelayakan = fungsi_optimum[1..]
        .iter()
        .map(|x| x / s0)
        .collect::<Vec<f64>>();

    let mut ranking_raw = kelayakan
        .iter()
        .enumerate()
        .map(|(i, x)| (i, *x))
        .collect::<Vec<(usize, f64)>>();

    ranking_raw.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    let ranking = ranking_raw
        .iter()
        .map(|(idx, _)| alternative[*idx])
        .collect::<Vec<&'static str>>();

    (kelayakan, ranking)
}
