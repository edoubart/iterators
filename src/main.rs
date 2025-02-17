/*
 * print_elements(..):
 * Prints each element in the vector one by one.
 */
fn print_elements(elements: &[String]) { // <- for Vector Slices, works in both
//fn print_elements(elements: &Vec<String>) {
    /*
     * Option 1:
     * Use a for loop. Automatically creates an iterator and calls 'next' on
     * it.
     * 'for' loops will ...
     *   - Automatically create an iterator for the vector;
     *   - Call 'next' on the iterator and unwrap the Option that comes back;
     *   - Break once 'next' returns a None.
     */
    //for element in elements {
    //    println!("{}", element);
    //}

    /*
     * Option 2:
     * Use iterator adaptors and consumers like 'for each', 'collect', 'map',
     * etc.
     * Iterators are "lazy". Nothing happens until ...
     *   - A) You call 'next';
     *   - B) You use a function that calls 'next' automatically.
     * 'for_each(..)' is an iterator consumer.
     * It will repeatedly call 'next()' on the iterator until it gets 'None'.
     */
    // "Lazy" iterator, idle
    //elements.iter()
    //    // B)
    //    .for_each(|el| println!("{}", el));

    /*
     * Iterator Adaptor
     * 'map(..)' is an iterator adaptor.
     * Adaptors create a step in a processing pipeline, but don't actually cause
     * any iteration.
     */
    // "Lazy" iterator, idle
    elements.iter()
        .map(|el| format!("{} {}", el, el))
        // B)
        .for_each(|el| println!("{}", el));
}

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
    //let mut colors_iter = colors.iter();

    //// next() -> Some(red), point to Next Position
    //println!("{:#?}", colors_iter.next());
    //// next() -> Some(green), point to Next Position
    //println!("{:#?}", colors_iter.next());
    //// next() -> Some(blue), point to Next Position
    //println!("{:#?}", colors_iter.next());
    //// next() -> None, point to Next Position == End
    //println!("{:#?}", colors_iter.next());

    /*
     * Call Site
     */
    //print_elements(&colors);

    /*
     * Vector Slices
     */
    print_elements(&colors[1..3]);
}
