fn hflip(list: &[String]) -> Vec<String>{
    let mut output_vec = Vec::new();
    let input_length = list.len();
    
    // count the longest line
    let mut longest_line = 0;
    for line in 0..input_length{
        if longest_line < list[line].len(){
            longest_line = list[line].len()
        }
    }

    // flip each line + spacing in front
    for line in list{
        let mut gap = String::new();
        // spacing
        for _ in 0..longest_line-line.len(){
            gap += " ";
        }
        // reverse string
        let reversed_string = line.chars().rev().collect::<String>();
        output_vec.push((gap + &reversed_string).to_string());
    }

    output_vec
}

fn main() {
    let data = ["<--", "#####", "<=="].map(|v| v.to_string());
    print!("{:?}", hflip(&data));
}

#[test]
fn test_img_flip() {
    let emp = ["".to_string(); 0];
    assert_eq!(hflip(&emp), [""; 0]);
    let data = ["<--", "#####", "<=="].map(|v| v.to_string());
    assert_eq!(hflip(&data), ["  --<", "#####", "  ==<"]);
}
