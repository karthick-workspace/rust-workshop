#[repr(C)]
union RiskyUnion {
    are_u: u32,
    press_f: f32,
}

pub fn union_demo() {
    let u = RiskyUnion { are_u: 1 };
    let read_u = unsafe { u.are_u };
    let read_f = unsafe { u.press_f };

    println!("{read_u} , {read_f}");
}