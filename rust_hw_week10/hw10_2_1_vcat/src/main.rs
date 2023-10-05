fn vcat(list1: &[String], list2: &[String]) -> Vec<String>{
    let mut output_vec: Vec<String> = Vec::new();
    for line in list1{
        output_vec.push(line.to_string())
    }
    for line in list2{
        output_vec.push(line.to_string())
    }
    output_vec
}

fn main() {
    let data = ["<--", "#####", "<=="].map(|v| v.to_string());
    let data2 = ["777", "AAAAAAAA", "4443"].map(|v| v.to_string());
    vcat(&data, &data2);
}

#[test]
fn test_img_cat() {
    let emp = ["".to_string(); 0];
    assert_eq!(vcat(&emp, &emp), [""; 0]);
    let data = ["<--", "#####", "<=="].map(|v| v.to_string());
    assert_eq!(vcat(&emp, &data), data);
    assert_eq!(vcat(&data, &emp), data);
    assert_eq!(
        vcat(&data, &data),
        ["<--", "#####", "<==", "<--", "#####", "<=="]
    );
}