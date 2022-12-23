use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::iter::Peekable;
use std::str::FromStr;
use crate::ListItem::{L, N};

fn main() {
let input = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]".to_string();

    input.split("\n\n")
        .into_iter()
        .map(|x| x.lines().collect::<Vec<_>>() )
        .map(|d|
                 (ListItem::parse(d[0]), ListItem::parse(d[1]))
        )
        .enumerate()
        .filter_map(|(i,(l,r))|
            if l.lt(&r) {
                Some(i)
            } else { None }
        )
        .inspect(|l| println!("{:?}",l))
        .all(|_| true);

    let l = ListItem::parse("[9]");
    let r = ListItem::parse("[[8,7,6]]");
    println!("{:?}",l.partial_cmp(&r));
}

impl PartialEq<Self> for ListItem {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl PartialOrd for ListItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self,other) {
            (L(l), L(r)) => {
                println!("(L(l), L(r)) - {:?},{:?}",self,other);
                let mut liter = l.iter();
                let mut riter = r.iter();

                loop {
                    match (liter.next(),riter.next()) {
                        (Some(l), Some(r)) =>
                            match l.partial_cmp(r).unwrap() {
                                Ordering::Equal => {}
                                ord@(Ordering::Less |
                                Ordering::Greater) => break Some(ord),
                            },
                        (Some(_), None) => break Some(Ordering::Greater),
                        (None, Some(_)) => break Some(Ordering::Less),
                        (None,None) => break Some(Ordering::Equal),
                    };
                }
            }
            (L(_), N(r)) => {
                println!("(L(_), N(r)) - {:?},{:?}",self,other);
                let right = L(vec![N(*r)]);
                self.partial_cmp(&right)
            }
            (N(l), L(_)) => {
                println!("(N(l), L(_)) - {:?},{:?}",self,other);
                let left = L(vec![N(*l)]);
                left.partial_cmp(other)
            }
            (N(l), N(r)) => {
                println!("(N(l), N(r) - {:?},{:?}",self,other);
                Some(l.cmp(r))
            },
        }
    }
}


enum ListItem {
    N(u8),
    L(Vec<ListItem>)
}
impl ListItem {
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
                            v.insert( self.parse_list());
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
    fn insert(&mut self, item:ListItem) {
        match (self,item) {
            (L(list), item) => list.push(item),
            (N(old), N(new)) => *old = new,
            (_,_) => unreachable!()
        }
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
