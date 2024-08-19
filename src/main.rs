
fn main() {

    use std::collections::HashMap;

    let list_of_numbers = vec![1, 2, 3, 4, 5, 4, 2, 1, 3, 5, 1, 3, 2, 2, 1];


    // Retornar a mediana de uma lista de números

    let mut list_of_numbers = list_of_numbers.clone();
    list_of_numbers.sort();

    let middle = list_of_numbers.len() / 2;

    let median = if list_of_numbers.len() % 2 == 0 {
        (list_of_numbers[middle - 1] + list_of_numbers[middle]) as f64 / 2.0
    } else {
        list_of_numbers[middle] as f64
    };

    println!("Median: {}", median);

    // Retornar a moda de uma lista de números

    let mut moda = HashMap::new();

    for number in list_of_numbers {
        let count = moda.entry(number).or_insert(0);
        *count += 1;
    }

    print!("{:?}", moda);
}

 