

#[cfg(feature = "display")]
pub fn display_image_w_label(data: &Vec<u8>, &Vec<u8>)  {
    let mut rng = rand::thread_rng();
    let num: usize = rng.gen_range(0..num_records);
    // Displaying in minifb window instead of saving as a .png
    let img_arr = data.slice(s!(num, .., .., ..));

    println!(
        "Data label: {}",
        return_label_from_one_hot(labels.slice(s![num, ..]).to_owned())
    );
    display_img(&img_arr.to_owned())?;
}

#[cfg(feature = "ndarray")]
fn return_label_from_one_hot(one_hot: Array1<u8>) -> String {
    if one_hot == array![1, 0, 0, 0, 0, 0, 0, 0, 0, 0] {
        "airplane".to_string()
    } else if one_hot == array![0, 1, 0, 0, 0, 0, 0, 0, 0, 0] {
        "automobile".to_string()
    } else if one_hot == array![0, 0, 1, 0, 0, 0, 0, 0, 0, 0] {
        "bird".to_string()
    } else if one_hot == array![0, 0, 0, 1, 0, 0, 0, 0, 0, 0] {
        "cat".to_string()
    } else if one_hot == array![0, 0, 0, 0, 1, 0, 0, 0, 0, 0] {
        "deer".to_string()
    } else if one_hot == array![0, 0, 0, 0, 0, 1, 0, 0, 0, 0] {
        "dog".to_string()
    } else if one_hot == array![0, 0, 0, 0, 0, 0, 1, 0, 0, 0] {
        "frog".to_string()
    } else if one_hot == array![0, 0, 0, 0, 0, 0, 0, 1, 0, 0] {
        "horse".to_string()
    } else if one_hot == array![0, 0, 0, 0, 0, 0, 0, 0, 1, 0] {
        "ship".to_string()
    } else if one_hot == array![0, 0, 0, 0, 0, 0, 0, 0, 0, 1] {
        "truck".to_string()
    } else {
        format!("Error: no valid label could be assigned to {}", one_hot)
    }
}