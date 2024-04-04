
use core::str;
use std::fs;

pub fn file_list() -> Vec<String> {
    let mut array:Vec<String> = Vec::new();

    for file in fs::read_dir("/home/juicy/Documents/logbook/").unwrap() {
        array.push(file.unwrap().file_name().into_string().expect("fuck"));
    }

    array.sort();
    array.remove(array.len() - 1);

    array.sort_by(|a, b| {
        let date_a: Vec<&str> = a.split('-').collect();
        let date_b: Vec<&str> = b.split('-').collect();

        let year_a: i32 = date_a[2].trim_end_matches(".txt").parse().unwrap();
        let month_a: u32 = date_a[0].parse().unwrap();
        let day_a: u32 = date_a[1].parse().unwrap();

        let year_b: i32 = date_b[2].trim_end_matches(".txt").parse().unwrap();
        let month_b: u32 = date_b[0].parse().unwrap();
        let day_b: u32 = date_b[1].parse().unwrap();

        let date_tuple_a = (year_a, month_a, day_a);
        let date_tuple_b = (year_b, month_b, day_b);

        date_tuple_a.cmp(&date_tuple_b)
    });
    return array;
}
