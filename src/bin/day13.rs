use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::iter::Peekable;
use std::str::FromStr;
use crate::ListItem::{L, N};

fn packets_in_right_order(input: &str) -> usize {
    input.split("\n\n")
        .into_iter()
        .map(|x| x.lines().collect::<Vec<_>>() )
        .map(|d|
            (ListItem::parse(d[0]), ListItem::parse(d[1]))
        )
        .enumerate()
        .filter_map(|(i,(l,r))|
            if l.lt(&r) { Some(i+1) } else { None }
        )
        .sum()
}

fn get_decoder_key(input: &str) -> usize {
    let mut inp: String = input.into();
    inp += "\n\n[[2]]\n[[6]]";

    let mut order = inp.split("\n\n")
        .into_iter()
        .flat_map(|x| x.lines() )
        .map(|d|
            ListItem::parse(d)
        )
        .fold(vec![], |mut out, item|{
            out.push(item);
            out
        });
    order.sort();
    order.iter().enumerate()
        .for_each(|(i,l)| println!("{}. {:?}",i+1,l));

    (order.iter().position(|item| L(vec![N(2)]).eq(item) ).unwrap() + 1)
        * (order.iter().position(|item| L(vec![N(6)]).eq(item) ).unwrap() + 1)

}

fn main() {
    // let mut input = "[1,1,3,1,1]\n[1,1,5,1,1]\n\n[[1],[2,3,4]]\n[[1],4]\n\n[9]\n[[8,7,6]]\n\n[[4,4],4,4]\n[[4,4],4,4,4]\n\n\
    // [7,7,7,7]\n[7,7,7]\n\n[]\n[3]\n\n[[[]]]\n[[]]\n\n[1,[2,[3,[4,[5,6,7]]]],8,9]\n[1,[2,[3,[4,[5,6,0]]]],8,9]".to_string();

    let input = std::fs::read_to_string("src/bin/day13_input.txt").expect("Ops!");

    let res = packets_in_right_order(input.as_str());
    println!("Correctly ordered packets = {:?}",res);
    let res = get_decoder_key(input.as_str());
    println!("Decoder Key = {:?}",res);

}

enum ListItem {
    N(u8),
    L(Vec<ListItem>)
}
impl ListItem {
    fn insert(&mut self, item:ListItem) {
        match (self,item) {
            (L(list), item) => list.push(item),
            (N(old), N(new)) => *old = new,
            (_,_) => unreachable!()
        }
    }

    fn parse(inp: &str) -> ListItem {

        struct Scanner<I: Iterator<Item=char>> {
            i: Peekable<I>,
        }
        impl<I: Iterator<Item=char>> Scanner<I> {
            fn new(s: I) -> Self {
                Scanner { i: s.peekable() }
            }
            fn parse_list(&mut self) -> ListItem {
                let mut s = String::new();
                let mut v = L(vec![]);
                loop {
                    match &self.i.peek() {
                        Some('[') => {
                            self.i.next();
                            v.insert(self.parse_list());
                        },
                        Some(&c@ '0'..='9') => s.push(c),
                        &c@
                        (Some(',') | Some(']')) if !s.is_empty() => {
                            v.insert(N(u8::from_str(s.as_str()).expect("")));
                            s.clear();
                            if ']'.eq(c.unwrap()) {
                                break v
                            }
                        },
                        Some(',') => {}
                        Some(']') => break v,
                        None => break v,
                        _ => unreachable!()
                    }
                    self.i.next();
                }
            }
        }
        let mut i = inp.chars().peekable();
        i.next();
        Scanner::new(i).parse_list()
    }
}

impl PartialEq<Self> for ListItem {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl Eq for ListItem {}

impl Ord for ListItem {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self,other) {
            (L(l), L(r)) => {
                let mut liter = l.iter();
                let mut riter = r.iter();

                loop {
                    match (liter.next(),riter.next()) {
                        (Some(l), Some(r)) =>
                            match l.cmp(r) {
                                Ordering::Equal => {}
                                ord@
                                (Ordering::Less | Ordering::Greater) => break ord,
                            },
                        (Some(_), None) => break Ordering::Greater,
                        (None, Some(_)) => break Ordering::Less,
                        (None,None) => break Ordering::Equal,
                    };
                }
            }
            (L(_), N(r)) => {
                let right = L(vec![N(*r)]);
                self.cmp(&right)
            }
            (N(l), L(_)) => {
                let left = L(vec![N(*l)]);
                left.cmp(other)
            }
            (N(l), N(r)) => l.cmp(r),
        }

    }
}

impl PartialOrd for ListItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Debug for ListItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            N(n) => write!(f,"{n}")?,
            L(v) => {
                write!(f,"[")?;
                let len = v.len();
                for (i,li) in v.iter().enumerate() {
                    li.fmt(f)?;
                    if i < len-1 { write!(f,",")?; }
                }
                write!(f,"]")?;
            }
        };
        Ok(())
    }
}
