use std::fs;

fn elf() {
    let list1: Vec<i32> = Vec::new();
    let list2: Vec<i32> = Vec::new();
    let puzzleErr = fs::read_to_string("./puzzle/01-1.txt");
    match puzzleErr {
        Err(e) => panic!("{e}"),
        Ok(res) => {
            println!("found puzzle")
        }
    };

}