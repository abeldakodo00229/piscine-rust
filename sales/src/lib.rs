#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}
impl Store {
    pub fn new(products: Vec<(String, f32)>) -> Store {
        Store { products }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cart {
    pub items: Vec<(String, f32)>,
    pub receipt: Vec<f32>,
}
impl Cart {
    pub fn new() -> Cart {
        Cart {
            items: Vec::new(),
            receipt: Vec::new(),
        }
    }
    pub fn insert_item(&mut self, s: &Store, ele: String) {
        if let Some(price) = s.products.iter().find(|(name, _)| *name == ele).map(|(_, price)| *price) {
            self.items.push((ele, price));
        }
    }
    pub fn generate_receipt(&mut self) -> Vec<f32> {
        self.items.sort_by(|(_, price1), (_, price2)| price1.partial_cmp(price2).unwrap());
        let prices: Vec<_> = self.items.iter()
        .filter_map(|(_, price)| Some(*price)) 
        .collect(); 
        let sum : f32 = prices.iter().sum();
        let sump : f32 = prices[0..prices.len()/3].iter().sum();

        for i in prices {
            let y = ((i*(((sum - sump)/ sum))) * 100.0).round() / 100.0;
            self.receipt.push(y);
        }
        self.receipt.clone()
    }
} 