trait Inventory {
    type StockItem;

    fn find(&self, stock_id: &String) -> Self::StockItem;
    fn add(&mut self, item: Self::StockItem) -> String;
}

struct Chair {}

#[derive(Clone, Copy)]
struct Chairs {}

impl Inventory for Chairs {
    type StockItem = Chair;

    fn find(&self, stock_id: &String) -> Self::StockItem {
        Chair {}
    }

    fn add(&mut self, item: Self::StockItem) -> String {
        "ABC12345".to_string()
    }
}

fn main() {
    let mut catalog = Chairs {};
    let stock_id = catalog.add(Chair {});
    let item = catalog.find(&stock_id);
    println!("Stock Id: {:?}", stock_id)
}
