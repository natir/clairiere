//! Example that build an interval tree from first file and use the second file to query

/* std use */
use std::io::BufRead as _;

/* crate use */

/* project use */

fn main() {
    let mut annotations =
        std::io::BufReader::new(std::fs::File::open(std::env::args().nth(1).unwrap()).unwrap());
    let mut variants =
        std::io::BufReader::new(std::fs::File::open(std::env::args().nth(2).unwrap()).unwrap());

    let mut tree = superintervals::SuperIntervals::new();
    let mut line = Vec::new();
    while let Ok(bytes) = annotations.read_until(b'\n', &mut line) {
        if bytes == 0 {
            break;
        }

        let mut split = line.split(|x| x == &b' ');
        let start = atoi::atoi(split.nth(1).unwrap()).unwrap();
        let stop = atoi::atoi(split.next().unwrap()).unwrap();

        tree.add(start, stop, true);

        line.clear();
    }

    tree.index();

    while let Ok(bytes) = variants.read_until(b'\n', &mut line) {
        if bytes == 0 {
            break;
        }
        let mut split = line.split(|x| x == &b' ');
        let start = atoi::atoi(split.nth(1).unwrap()).unwrap();
        let stop = atoi::atoi(split.next().unwrap()).unwrap();

        let mut result = Vec::new();
        tree.find_overlaps(start, stop, &mut result);
        criterion::black_box(result);

        line.clear();
    }
}
