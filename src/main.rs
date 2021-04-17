fn main() {
    lyrics();
}

fn lyrics() {
    let index = 0;
    let contador = 1;
    let phrase_array = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];
    mostrar_letra(phrase_array, index, contador);
}

fn mostrar_letra(phrase_array: [&str; 12], mut index: i32, mut contador: i32) {
    for _elem in phrase_array.iter() {
        println!(
            "| on the {} day of Christmas, My true love sent to me",
            index + 1
        );
        println!("|__*{}!", _elem);
        for elementrev in phrase_array[..index as usize].iter().rev() {
            println!("|{} __*{}", contador, elementrev);
            contador += 1;
        }
        contador = 1;
        println!("------");
        index += 1;
    }
}
