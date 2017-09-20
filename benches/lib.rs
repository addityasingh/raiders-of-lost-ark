#![feature(test)]

extern crate rola;
extern crate test;

use rola::search;

mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_search_empty(b: &mut Bencher) {
        let contents = "\
            Rust:
            safe, fast, productive.
            Pick three.";
        let query = "direct";

        b.iter(|| search(contents, query));
    }

    #[bench]
    fn bench_search_nonempty(b: &mut Bencher) {
        let contents = "\
            Rust:
            safe, fast, productive.
            Pick three.";
        let query = "duct";

        b.iter(|| search(contents, query));
    }
}