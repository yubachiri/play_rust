fn main() {
    let can_ref = 5;
    use_copy(can_ref);
    println!("can_ref is {}", can_ref);

    let cannot_ref: Vec<i32> = vec![5; 5];
    use_ref(cannot_ref);
    println!("cannot_ref is {}", cannot_ref[0]);
}

fn use_copy(num: i32) {
    println!("num is {}", num);
}

fn use_ref(nums: Vec<i32>) {
    println!("num is {}", nums[0]);
}
