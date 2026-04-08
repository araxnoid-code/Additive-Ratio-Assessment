use aras::TipeKategori;

fn main() {
    let alternative = ["A", "B", "C", "D", "E"];

    let data = [
        [1.0, 2.0, 5.0, 3.0],
        [3.0, 3.0, 5.0, 2.0],
        [1.0, 3.0, 5.0, 4.0],
        [2.0, 1.0, 4.0, 4.0],
        [4.0, 4.0, 4.0, 3.0],
    ];

    let tipe_kategori = [
        TipeKategori::COST,
        TipeKategori::BENEFIT,
        TipeKategori::BENEFIT,
        TipeKategori::BENEFIT,
    ];

    let bobot = [0.2, 0.1, 0.4, 0.3];

    let (kelayakan, ranking) = aras::aras_start(alternative, data, tipe_kategori, bobot);
    println!("{:?}", kelayakan);

    // ranking sudah di urutkan dari tertinggi
    println!("{:?}", ranking);
}
