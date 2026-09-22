fn main() {
    println!("Hello, world!");
    decl_var();
    array();
}
//testing comment

fn decl_var(){
    let x = 10;
    println!("x is {}", x);
}

fn array(){
    let mut letters = ['a', 'b', 'c'];
    letters[0] = 'x';
    let first_letter = letters[0];
    println!("first_letter is {}", first_letter);

    let numbers: [i32; 5];
    numbers = [0; 5];
    let index: usize = numbers.len();
    println!("last number is {}", numbers[index - 1]);
}