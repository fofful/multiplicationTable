
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    let i_min = args[1].parse::<i32>().unwrap();
    let i_max = args[2].parse::<i32>().unwrap();
    let j_min = args[3].parse::<i32>().unwrap();
    let j_max = args[4].parse::<i32>().unwrap();
    let mut max_num = i_max * j_max;
    let mut count = 0;
    while max_num > 0 {
        max_num = max_num / 10;
        count = count + 1;
    };
    let mut j_max_copy = j_max.clone();
    let mut j_count = 0;
    while j_max_copy > 0 {
        j_max_copy = j_max_copy / 10;
        j_count = j_count + 1;
    };
    println!("count is {}", count);
    for j_c in 0..j_count{
        print!(" ");
    }
    for i in i_min..i_max{
        let num = i;
            let mut num_copy = num.clone();
            let mut head = 0;
            while num_copy > 0 {
                num_copy= num_copy / 10;
                head = head + 1;
            };
            for c in head..count+1{
                print!(" ");
            }
            print!("{}", num);
    }
    println!("");

    for i in i_min..i_max{
        let mut i_curr_copy = i.clone();
        let mut i_count = 0;
        while i_curr_copy > 0 {
            i_curr_copy = i_curr_copy / 10;
            i_count = i_count + 1;
        };
        for j_c in i_count..j_count{
            print!(" ");
        }
        print!("{}", i);
        for j in j_min..j_max{
            let num = i * j;
            let mut num_copy = num.clone();
            let mut head = 0;
            while num_copy > 0 {
                num_copy= num_copy / 10;
                head = head + 1;
            };
            for c in head..count+1{
                print!(" ");
            }
            print!("{}", num);
        }
        println!("");
    }
}