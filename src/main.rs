fn main() {
    learn_13_if_expression();
}

fn learn_13_if_expression() {
    let score = 40;
    // if expression
    if score < 50 {
        println!("your grade is E");
    } else if score < 60 {
        println!(" your grade is D");
    } else if score < 70 {
        println!(" your grade is C");
    } else if score < 80 {
        println!(" your grade is B");
    } else if score < 90 {
        println!(" your grade is A");
    } else {
        println!("your grade undefined");
    }

    // if expression in let statement
    // rust dapat mengembalikan nilai dari if expression
    let grade = if score < 50 {
        "E"
    } else if score < 60 {
        "D"
    } else if score < 70 {
        "C"
    } else if score < 80 {
        "B"
    } else if score < 90 {
        "A"
    } else {
        "Undefined"
    };

    println!("your grade is {}", grade);
    
}
