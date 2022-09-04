use bedmap::bed_map;

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    let opener = |p| std::fs::File::open(p).unwrap();
    for l in bed_map(opener(&argv[1]), opener(&argv[2])) {
        println!("{}", l.unwrap());
    }
}
