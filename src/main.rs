/*
 * explode(..):
 * Turns a Vec<String> into a Vec<Vec<String>>,
 * This function is a pure function as it doesn't have any side effect.
 */
fn explode(elements: &[String]) -> Vec<Vec<String>> {
    elements.iter()
        .map(
            |el| el.chars().map(|c| c.to_string()).collect()
        )
        .collect()
}

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

/*
 * shorten_strings(..):
 * Shortens each string in the vector to 1 character.
 */
fn shorten_strings(elements: &mut [String]) {
    // 'iter_mut(..)' will give you a mutable reference to each element
    elements.iter_mut()
        .for_each(|el| el.truncate(1))
}

/*
 * to_uppercase(..):
 * Return a new vector with each element capitalized.
 * This function is a pure function as it doesn't have any side effect.
 */
fn to_uppercase(elements: &[String]) -> Vec<String> {
    // Option 1:
    //let upcased: Vec<String> = elements.iter()
    //    // 'map(..)' is an iterator adaptor.
    //    .map(|el| el.to_uppercase())
    //    // 'collect()' is an iterator consumer. It will automatically call
    //    // 'next()'.
    //    // Collect elements in a brand new data structure of type Vec<String>.
    //    .collect();
    //
    //upcased

    // Option 2:
    //elements.iter()
    //    .map(|el| el.to_uppercase())
    //    .collect()
 
    // Option 3:
    //elements.iter()
    //    .map(|el| el.to_uppercase())
    //    .collect::<Vec<_>>()

    // Option 4 (favorite):
    elements.iter()
        .map(|el| el.to_uppercase())
        .collect::<Vec<String>>() // <- "Turbofish" :)
}

/*
 * move_elements(..):
 * Moves elements from one vector to another.
 */
fn move_elements(vec_a: Vec<String>, vec_b: &mut Vec<String>) {
    vec_a.into_iter()
        .for_each(|el| vec_b.push(el))
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
    //print_elements(&colors[1..3]);

    //shorten_strings(&mut colors);
    //shorten_strings(&mut colors[1..3]);
    //println!("{:#?}", colors);

    //let uppercased = to_uppercase(&colors);
    //println!("{:#?}", uppercased);

    //let mut destination = vec![];
    //move_elements(colors, &mut destination);
    //println!("Destination: {:#?}", destination);

    let exploded = explode(&colors);
    println!("{:#?}", exploded);
}
