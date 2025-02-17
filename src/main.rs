fn main() {
    // Vec<String>
    let colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ];

    /*
     * Iter<String>, struct with 3 fields:
     *   1. Pointer to Data
     *   2. Pointer to Current Position (requires the binding to be mutable)
     *   3. Pointer to End
     */
    let mut colors_iter = colors.iter();

    // next() -> Some(red), point to Next Position
    println!("{:#?}", colors_iter.next());
    // next() -> Some(green), point to Next Position
    println!("{:#?}", colors_iter.next());
    // next() -> Some(blue), point to Next Position
    println!("{:#?}", colors_iter.next());
    // next() -> None, point to Next Position == End
    println!("{:#?}", colors_iter.next());
}
