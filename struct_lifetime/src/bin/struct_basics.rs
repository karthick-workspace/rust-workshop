struct Vector {
    elem: Box<[f64]>,
    sz: usize,
}

impl Vector {
    fn new(s: usize) -> Vector {
        Vector {
            elem: vec![0.0; s].into_boxed_slice(),
            sz: s,
        }
    }

    fn get(&self, i: usize) -> f64 {
        self.elem[i]
    }

    fn set(&mut self, i: usize, value: f64) {
        self.elem[i] = value;
    }

    fn size(&self) -> usize {
        self.sz
    }
}

fn main() {
    let mut vec = Vector::new(6);

    for i in 0..vec.size() {
        vec.set(i, i as f64);
    }

    for i in 0..vec.size() {
        println!("vec[{}] = {}", i, vec.get(i))
    }
}
