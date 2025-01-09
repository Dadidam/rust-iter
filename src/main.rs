// iterating using basic for loop
fn print_elements_loop(elements: &Vec<String>) {
    for element in elements {
        println!("{}", element)
    }
}

// iterating using iterator consumers and adaptors
fn print_elements_consumers(elements: &[String]) {
    elements
        .iter()
        .map(|el| format!("{} {}", el, el))
        .for_each(|el| println!("{}", el));
}

fn shorten_strings(elements: &mut [String]) {
    elements.iter_mut().for_each(|el| el.truncate(1));
}

fn to_uppercase(elements: &[String]) -> Vec<String> {
    elements
        .iter()
        .map(|el| el.to_uppercase())
        .collect()
}

fn main() {
    let mut colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ];

    print_elements_loop(&colors);
    print_elements_consumers(&colors);

    // shorten_strings(&mut colors);

    let uppercased = to_uppercase(&colors);
    println!("{:#?}", uppercased);

    // let mut colors_iter = colors.iter();

    // println!("{:#?}", colors_iter.next());
    // println!("{:#?}", colors_iter.next());
    // println!("{:#?}", colors_iter.next());
    // println!("{:#?}", colors_iter.next());
}

