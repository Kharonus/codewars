mod katas;

fn main() {
    let drink = katas::bartender::get_drink_by_profession("Jabroni");
    println!("{}", drink);

    let str = katas::stringy_strings::stringy(5);
    println!("{}", str);

    let valid = katas::pin::validate_pin("12345");
    println!("{}", valid);

    katas::bus::number(&[(10, 0), (3, 5), (5, 8)]);
}
