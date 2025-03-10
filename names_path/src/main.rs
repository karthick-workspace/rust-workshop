mod union_demo;

#[derive(Debug)]
struct Gene<'a, T: Copy> {
    dna: &'a [T],
}

impl<'a, T: Copy> Gene<'a, T> {
    fn first(gene: &Self) -> Option<T> {
        if let &[f, ..] = gene.dna {
            Some(f)
        } else {
            None
        }
    }
}


fn turbofish_demo<'a>() {
    let g = Gene::<'a, u8> {
        dna: &[1, 2, 3]
    };

    println!("{g:?}");

    let f = Gene::<'a, u8>::first(&g);
    println!("First Element: {f:?}");
}

fn main() {
    turbofish_demo();
    union_demo::union_demo();
}
