mod user;
mod product;

use user::*;
use product::Product;

fn main() {
    learn_22_module();
}

fn learn_22_module() {
    let user = User {
        username: "username".to_string(),
        email: "email".to_string(),
        age: 20
    };
    user.register();
    user.get_data();

    let product = Product {
        name: "product".to_string(),
        price: 100000
    };
    product.get_data();
}
