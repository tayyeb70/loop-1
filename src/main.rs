// fn main() {
//     let mut counter = 0;
//     let val_counter = loop{
//     println!("Ruqaya noor");
//     counter = counter + 1;
//     if (counter == 42){
//         break counter
//     }
// };
// println!("{}",val_counter);

// }

fn main () {
    // let mut counter = 0;
    // while counter < 5 {
    //     println!("Tayyeb javed");
    //     counter = counter + 1;
    // }

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println! ("The result is {}",result)
}
// let mut counter_1 = 0;
// let lottery_number = [1,23,45,6,78,9];

// while counter_1 < lottery_number.len() {
//     println!("{}",lottery_number[counter_1]);
//     counter_1 = counter_1 + 1;
//     }
// }

