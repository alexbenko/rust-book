fn main() {
    let score = 83;
    let grade = if score >= 90 {"A"} else {"Failed"};
    println!("Grade: {}", grade);

    if score >= 90{
        println!("A!")
    } else if score >= 80 {
        println!("B");
    } else {
        println!("No A :(")
    }

    loopy_doop();

    //If you have loops within loops, break and continue apply to the innermost loop at that point
    // unless you specify a label
    label_loop();

    return_from_loop();
    for_loop();
}


fn loopy_doop(){
    let mut a = 0;
    loop {
        a = a + 1;
        println!("Loops: {}", a);
        if a == 5{
            break
        }
    }
}

fn label_loop(){
    let mut z = 0;
    let target = 100;
    'count_up: loop{
        loop {
            println!("Current: {}", z);
            if z == 10 {
                println!("Label Break!");
                break 'count_up;
            } else if z == target {
                break;
            }
            z += 1;
        }
    }
}

fn return_from_loop(){
    let mut count = 0;
    let hundred = loop {
        count += 10;
        if count == 100 {
            break 100;
        }
    };

    println!("{}", hundred)
}

fn for_loop(){
    let aaa = [1,2,3,4,5];
    for a in aaa{
        println!("a: {}", a);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
}