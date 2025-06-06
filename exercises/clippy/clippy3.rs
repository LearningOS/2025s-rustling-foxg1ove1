// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.


#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    // if my_option.is_none() {
    //     my_option.unwrap();
    // }

    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6
    ];
    println!("My array! Here it is: {:?}", my_arr);

    // let mut my_empty_vec = vec![1, 2, 3, 4, 5].my_empty_vec.resize(0, 5); //resize 是一个方法，会修改原向量，且没有返回值
    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    my_empty_vec.clear(); // clear 是一个方法，会修改原向量，且没有返回值
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    // let temp = value_a;
    // value_a = value_b;
    // value_b = temp;
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}
