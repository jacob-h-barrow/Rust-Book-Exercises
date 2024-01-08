use std::collections::HashMap;

struct Cashier {
    n: i32,
    length: i32,
    discount: i32,
    items: HashMap<i32, i32>,
}


impl Cashier {

    fn new(n: i32, discount: i32, products: Vec<i32>, prices: Vec<i32>) -> Self {
        let mut mapper: HashMap<i32, i32> = HashMap::new();

        products
            .iter()
            .zip(prices.iter())
            .for_each(|(&p, &pr)| { mapper.entry(p).or_insert(pr); });

        Self {
            n,
            length: 0,
            discount,
            items: mapper,
        }
    }
    
    fn apply_discount(&self, bill: f64) -> f64 {
        bill * ((100 - self.discount) as f64 / 100.0)
    }

    fn get_bill(&mut self, product: Vec<i32>, amount: Vec<i32>) -> f64 {
        self.length += 1;

        let result: f64 = product
                            .iter()
                            .zip(amount.iter())
                            .map(|(&item, &taken)| self.items[&item] as f64 * taken as f64)
                            .sum();

        if self.length % self.n == 0 {
            self.apply_discount(result)
        } else {
            result
        }
    }
}
