const MAX_VAL: u32 = 100_000;

fn main() {
    println!("value of x + y is: {}", another_function(5, 6));

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    control_flow();

    println!("loop example returned: {}", loop_ex());

    for_each_ex();

    range_rev_ex();
}

fn another_function(x: i32, y: i32) -> i64{
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    (x + y ) as i64
}

fn control_flow() {
    let cond = true;
    let number: i32 = if cond {
        5
    } else {
        6 //"six"
    };
    println!("val of number is: {}", number);
}

fn loop_ex () -> i32{
    let mut cnt = 0;

    loop{
        cnt += 1;
        if cnt == 10{
            break cnt;
        }
    }
}

fn for_each_ex () {
    println!("for each in array example...");
    let int_arr = [2,4,6,8,10,12];
    for el in int_arr.iter() {
        println! ("current val is: {}", el);
    }
}

fn range_rev_ex () {
    for number in (1..5).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}