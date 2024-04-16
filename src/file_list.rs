
use std::fs::OpenOptions;
use std::path::Path;
use std::io::Read;
use std::io::BufReader;
use core::str;
use std::fs;
use directories::UserDirs;

pub fn file_list() -> Vec<String> {
    let path = UserDirs::new().unwrap();
    let home_path = format!("{}/Documents/logbook", path.home_dir().to_string_lossy());
    let mut array:Vec<String> = Vec::new();

    for file in fs::read_dir(&home_path).unwrap() {
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

pub fn file_reader(file_to_grab: String) ->  String {
    //1) read all the files into a vec of strings (check)
    //2) iterate through the array and fine the file name that matches with an entry in the vec of
    //   strings
    //3) return the string and write the file into another vec of strings
    //4) return that vec of strings
    let path = UserDirs::new().unwrap();
    let home_path = format!("{}/Documents/logbook", path.home_dir().to_string_lossy());
    let path_file = format!("{}/{}", &home_path, &file_to_grab);
    let mut array:Vec<String> = Vec::new();

    if !Path::new(&path_file).exists(){
        return "error".to_string();
    }

    for file in fs::read_dir(&home_path).unwrap() {
        array.push(file.unwrap().file_name().into_string().expect("fuck"));
    }
    array.sort();
    array.remove(array.len() - 1);

    let mut answer = String::new();
    for entry in array {
       if entry.to_string() == file_to_grab {
           answer = entry.to_string();
       }
    }

    let file_path = format!("{}/{}", home_path, answer);
    
    if !Path::new(&file_path).exists(){
        return "error".to_string();
    }
    else{
        let read_file = OpenOptions::new()
            .read(true)
            .open(&file_path)
            .unwrap();
        let mut read_file = BufReader::new(&read_file);
        let mut file_buf = String::new();
        read_file.read_to_string(&mut file_buf).expect("File not found");
        return file_buf
    }
}
