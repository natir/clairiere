//! Coitrees tree query

/* std use */
use std::io::BufRead as _;

/* crate use */
use rand::Rng as _;

/* project use */

fn main() {
    let mut reader =
        std::io::BufReader::new(std::fs::File::open(std::env::args().nth(1).unwrap()).unwrap());

    let mut tree = superintervals::SuperIntervals::new();
    let mut line = Vec::with_capacity(1024);
    while let Ok(bytes) = reader.read_until(b'\n', &mut line) {
        if bytes == 0 {
            break;
        }

        let mut split = line.split(|x| x == &b' ');
        let start: usize = atoi::atoi(split.nth(1).unwrap()).unwrap();
        let stop: usize = atoi::atoi(split.next().unwrap()).unwrap();

        tree.add(start as i32, stop as i32, true);

        line.clear();
    }

    tree.index();

    let mut rng: rand::rngs::SmallRng = rand::SeedableRng::seed_from_u64(42);

    let start_min: usize = 0;
    let start_max: usize = 200_000_000;
    let length_max = 2_000;
    let number_query = 1 << 9;

    for i in 0..number_query {
        let start = rng.gen_range(start_min..start_max) as i32;
        let length = rng.gen_range(0..length_max);

        let now = std::time::Instant::now();
        for _ in 0..100 {
            let mut result: Vec<bool> = Vec::with_capacity(1024);
            tree.find_overlaps(start, start + length, &mut result);
            criterion::black_box(result);
        }
        println!("superintervals,{},{}", i, now.elapsed().as_nanos());
    }
}
