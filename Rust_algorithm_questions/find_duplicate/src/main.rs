use std::collections::HashMap;
use rand::Rng;

fn find_duplicate(nums: Vec<i32>) -> Vec<i32>
{
    let mut hash : HashMap<&i32,usize>= HashMap::new();
    let mut lst : Vec<i32> = Vec::new();

    for i in &nums
    {
        let counter = hash.entry(i).or_insert(0);
        *counter += 1;
    }

    for (a,b) in hash.iter()
    {
        if *b >= 2
        {
            lst.push(**a);
        }

    }
    lst
}

fn main()
{
    let mut rng = rand::thread_rng();
    let mut result : Vec<i32> = Vec::new();
    for _ in 0..20
    {
        let random : i32 = rng.gen_range(0..=10);
        result.push(random);
    }
    println!("{:?}",result);
    let print_value = find_duplicate(result);
    println!("{:?}",print_value);
}
