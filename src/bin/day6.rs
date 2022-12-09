use std::fmt::Debug;

trait Duplicate {
    fn has_duplicates(&self) -> bool;
}
impl<T> Duplicate for [T] where T: Debug + Copy + PartialEq + Ord {
    fn has_duplicates(&self) -> bool {
        let mut tmp = self.to_vec();
        tmp.sort();
        tmp.windows(2).any(|a| a[0]==a[1])
    }
}

trait Signaling {
    fn marker_position(&self, len:usize) -> usize;
}
impl<T> Signaling for [T] where T : Debug + Copy + PartialEq + Ord {
    fn marker_position(&self, len: usize) -> usize {
        self.windows(len)
            .enumerate()
            .skip_while(|&(_,stm)| stm.has_duplicates() )
            .next()
            .map(|(i,_)| i + len)
            .unwrap_or_else(|| panic!("marker_position(): Ops!"))
    }
}

fn main() {
    let data = std::fs::read_to_string("src/bin/day6_input.txt").expect("");

    let out = data.bytes().collect::<Vec<_>>();
    println!("Marker Length @4 = {}", out.marker_position(4));
    println!("Marker Length @14 = {}", out.marker_position(14));
}