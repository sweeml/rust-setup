fn main() {
    let proceed = false;
    if proceed {
        println!("Proceeding");
    } else {
        println!("Not proceeding");
    }

    let height = 190;
    if height < 180 {
        println!("Tall");
    } else if height > 170 {
        println!("Average");
    } else {
        println!("Short");
    }

    let age: i32 = 45;
    if age < 18 {
        println!("Minor");
    } else if age < 60 {
        println!("Adult");
    } else {
        println!("Senior");
    }

}