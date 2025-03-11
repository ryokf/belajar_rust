pub struct Product {
    pub name: String,
    pub price: u32
}

impl Product {
    pub fn get_data(&self){
        println!("Product {} dengan harga {}", self.name, self.price);
    }
}