fn main() {
    let file = std::fs::read_to_string("file.txt").unwrap();

    file.lines()
        .enumerate()
        .filter(|(index, _)| index % 2 == 0)
        .skip(2)
        .take(2)
        .for_each(|(_, line)| println!("{}", line));
}
