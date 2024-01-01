//Twelve days of Christmas lyrics (Rust practice)

fn main() {
    let s = ["a partridge in a pear tree", "two turtle doves", "three french hens",
             "four calling birds", "five golden rings", "six geese a-laying",
             "seven swans a-swiming", "eight maids a-milking", "nine ladies dancing",
             "ten lords a-leaping", "eleven pipers piping", "twelve drummers drumming"];

    let d = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh",
             "eighth", "ninth", "tenth", "eleventh", "twelfth"];

    let mut temp: i32 = 0;

    for verse in 0..12 {
        println!("\nOn the {} day of Christmas,\nmy true love gave to me", d[verse]);
        if verse == 0 {
            println!("{}", s[verse]);
        } else {
            temp = verse as i32;

            while temp != -1 {
                if temp==0 {
                    print!("And ");
                }
                println!("{}", s[temp as usize]);

                temp -= 1;
            }
        }
    }
}
