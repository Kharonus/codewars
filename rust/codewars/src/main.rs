mod katas;

fn main() {
    katas::geometry::find_nb(4183059834009);

    katas::bartender::get_drink_by_profession("Jabroni");

    katas::strings::stringy(5);
    katas::strings::ends_with("abc", "bc");
    katas::strings::break_camel_case("camelCase");

    katas::pin::validate_pin("12345");

    katas::bus::number(&[(10, 0), (3, 5), (5, 8)]);

    katas::numbers::narcissistic(153);
    katas::numbers::high_and_low("1 2 3 4 5");
}
