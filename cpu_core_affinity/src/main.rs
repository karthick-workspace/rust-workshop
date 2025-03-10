use std::vec;

fn main() {
    let core_ids = core_affinity::get_core_ids().unwrap();

    let handles = core_ids
        .into_iter()
        .map(|id| {
            std::thread::spawn(move || {
                let success = core_affinity::set_for_current(id);
                if success {
                    println!("thread is running on core {:?}", id);
                } else {
                    println!("failed to set thread on core {:?}", id);
                }
            })
        })
        .collect::<Vec<_>>();

    for handle in handles.into_iter() {
        handle.join().unwrap();
    }
}
