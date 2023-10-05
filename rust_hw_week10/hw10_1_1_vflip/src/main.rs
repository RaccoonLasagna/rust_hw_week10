fn vflip(list: &[String]) -> Vec<&String>{
    let mut output_vec = Vec::new();
    let input_length = list.len();
    for line in (0..input_length).rev(){
        let current_string = &list[line];
        output_vec.push(current_string);
    }
    print!("{:?}", output_vec);
    output_vec
}

fn main() {
    let data = ["<--", "#####", "<=="].map(|v| v.to_string());
    vflip(&data);
}

#[test]
fn test_img_flip() {
    let emp = ["".to_string(); 0];
    assert_eq!(vflip(&emp), [""; 0]);
    let data = ["<--", "#####", "<=="].map(|v| v.to_string());
    assert_eq!(vflip(&data), ["<==", "#####", "<--"]);
}
