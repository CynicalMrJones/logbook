use ftp::FtpStream;
use std::io::Cursor;

use crate::file_list::file_reader;

pub fn file_upload() {

    //Leaving off here
    //Need to iterate over whole logbook folder and upload each file individualy
    //Also need to be able to handle errors when not connected to this private network
    //Also hide the username and password
    let mut ftp_stream = FtpStream::connect("192.168.1.186:21").unwrap();
    ftp_stream.login("ftpuser", "IsMbnR12-").unwrap();

    ftp_stream.cwd("/Jacob/Documents/").unwrap();
    let file_to_read = file_reader("5-25-2024.txt".to_string());
    let mut reader = Cursor::new(file_to_read.as_bytes());
    ftp_stream.put("test_num_1.txt", &mut reader).unwrap(); 
    ftp_stream.quit().unwrap();

}
