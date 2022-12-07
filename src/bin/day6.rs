use std::fmt::Debug;

trait Duplicate {
    fn has_duplicates(&self) -> bool;
}

impl<T> Duplicate for &[T] where T: Debug + PartialEq {
    fn has_duplicates(&self) -> bool {
        let len = self.len();
        !self.iter()
            .take(len-1)
            .enumerate()
            .all(|(i,e)| !self[i+1..].contains(e))
    }
}

fn main() {
    // let data = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    // let data = "nbvwbjplbgvbhsrlpgdmjqwftvncz";
    let data = "nnznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";

    let out = data.bytes()
        .collect::<Vec<_>>()
        .windows(4)
        .enumerate()
        .inspect(|e| println!("{:?}",e))
        .skip_while(|&(idx,stm)| stm.has_duplicates() )
        .next()
        .map(|(i,e)| i+4).unwrap();
    println!("{out}");
}