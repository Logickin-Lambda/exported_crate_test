use crate_creation_to_export_and_beyond::n_log_n::*;
fn main() {
    println!("Logickin tests recently published crate - crate_creation_to_export_and_beyond");

    let test_data = vec![7,5,4,8,6,8,5,6,2,1,9,7,2,4,3,5,7,6,2,1,7,8,9];
    let result = merge_sort(&test_data);

    println!("Yay! I have published the crate and load it as a new project!");
    println!("Before: {:?}", test_data);
    println!("After : {:?}", result);
}
