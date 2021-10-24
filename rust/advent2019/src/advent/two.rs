use adventlib;

pub fn program_alarm(file: &String) {
    println!("1202 Program Alarm\n");

    let values = adventlib::read_file_as_list_of_i64(file);

    println!("{:?}", values);

}