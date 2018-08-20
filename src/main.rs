fn main() {
    let mut counter = 0;

    let result = loop{
        println!("counter: {}", counter );
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    // assert equivalent... still not sure what this does :\
    assert_eq!( result, 20 );
    println!( "\ncounter after break: {}\n", counter );

    // the above is the hard way of doing a while construction
    // now here's the easy way
    let mut number = 3;

    while number != 0 {
        println!( "{}!", number );

        number = number - 1;
    }

    println!( "LIFTOFF!!!" );

    // looping through a collection
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    // can be done with a while, but error prone
    // can cause panic if index length is incorrect
    // adds runtime performing conditional check on every elem on every iter
    println!("\nwhile style");
    while index < 5 {
        println!("the value is: {}", a[index] );
        index += 1;
    }

    // for loop is more concise
    // no refactoring needed if size of a changes
    println!("\nfor the win");
    for element in a.iter() {
        println!("the value is: {}", element );
    }

    println!("");

    // even for specific number of iterations,
    // a for loop using a range is cleaner and more practical
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFT'EM!");
}
