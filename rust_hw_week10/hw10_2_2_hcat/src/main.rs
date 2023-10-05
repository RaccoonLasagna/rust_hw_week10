fn hcat(list1_input: &[String], list2_input: &[String]) -> Vec<String> {
    let mut output_vec: Vec<String> = Vec::new();
    
    // clone em because I want to modify the lists
    let mut list1 = list1_input.clone().to_vec();
    let mut list2 = list2_input.clone().to_vec();

    // make them equal in length
    while list1.len() < list2.len(){
        list1.push("".to_string())
    }
    while list2.len() < list1.len(){
        list2.push("".to_string())
    }

    let input_length = list1.len();
    
    // count the longest line
    let mut longest_line = 0;
    for line in 0..input_length{
        if longest_line < list1[line].len(){
            longest_line = list1[line].len()
        }
    }

    // add spacing behind the original strings
    let mut spaced_list1: Vec<String> = Vec::new();
    for line in list1{
        let mut gap = String::new();
        // spacing
        for _ in 0..longest_line-line.len(){
            gap += " ";
        }
        spaced_list1.push((line.to_string() + &gap).to_string());
    }

    // add the second list to the first after spacing
    for linenum in 0..list2.len(){
        // spaced list 1 + list 2, then trim the trailing spaces, then turn them to string
        output_vec.push(((spaced_list1[linenum].to_string() + &list2[linenum]).trim_end()).to_string());
    }

    // println!("Spaced list 1: {:?}", spaced_list1);
    // println!("List 2: {:?}", list2);
    println!("Output: {:?}", output_vec);
    output_vec
}

fn main() {
    let emp = ["".to_string(); 0];
    hcat(&emp, &emp);
    let data = ["<--", "#####", "<=="].map(|v| v.to_string());
    hcat(&data, &data[..2]);
    hcat(&data[..2], &data);
}

#[test]
fn test_img_cat() {
    let emp = ["".to_string(); 0];
    assert_eq!(hcat(&emp, &emp), [""; 0]);
    let data = ["<--", "#####", "<=="].map(|v| v.to_string());
    assert_eq!(hcat(&data, &data[..2]), ["<--  <--", "##########", "<=="]);
    assert_eq!(hcat(&data[..2], &data), ["<--  <--", "##########", "     <=="]);
}
