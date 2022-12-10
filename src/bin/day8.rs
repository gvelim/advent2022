
fn main() {
    let data = "30373\n25512\n65332\n33549\n35390";

    data.lines()
        .map(|line|
            line.bytes().map(|n| n - b'0').collect::<Vec<_>>()
        )
        .inspect(|a| println!("{:?}",a))
        .all(|_|true);
}