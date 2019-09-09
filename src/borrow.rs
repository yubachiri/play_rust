fn main() {
    let can_copy = 5;
    use_copy(can_copy);
    println!("can_ref is {}", can_copy);

    let cannot_ref: Vec<i32> = vec![5; 5];
    use_ref(cannot_ref);
    // println!("cannot_ref is {}", cannot_ref[0]); // これは所有権がないので読めない
}

fn use_copy(num: i32) {
    println!("num is {}", num);
}

fn use_ref(nums: Vec<i32>) {
    println!("num is {}", nums[0]);
}
