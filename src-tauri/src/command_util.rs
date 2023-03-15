use std::process::Command;

pub fn  init_command(){
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .arg("ls")
            .output()
            .expect("failed to execute process")
    }else {
        Command::new("sh")
            .output()
            .expect("failed to execute process")
    };
    let data = String::from_utf8_lossy(&output.stdout);
    println!("{:#?}", data)

}
// pub fn execute_cmd(cmd:String){
//     let mut input_str = cmd;
//     let stdin = std::io::stdin();
//     stdin.read_line(&mut input_str).expect("输入出错");
//     println!("您的输入是：{}", input_str);//5
// }