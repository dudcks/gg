use std::io;
use std::fs;
use std::fs::File;
use std::io::Write;
use history;
use encry;

fn signin(){
    let mut id = String::new();

    print!("아이디:");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin().read_line(&mut id).expect("Failed to read line");

    id = id.trim().to_string();

    let file_path = format!("./data/usr/{}.txt", id);
    let his_path = format!("./data/his/{}.txt", id);

    if std::path::Path::new(&file_path).exists() {
        println!("이지 존재하는 ID 입니다.");
    } else {
        print!("비밀반호:");
        io::stdout().flush().expect("Failed to flush stdout");
        let mut password = String::new();
        io::stdin().read_line(&mut password).expect("Failed to read line");
        password=password.trim().to_string();

        let hashed_password = encry::encrypt_pwd(password); 

        let mut sign = File::create(&file_path).expect("Failed to create file");
        let _his = File::create(&his_path).expect("Failed to create file");

        sign.write_all(hashed_password.as_bytes()).expect("Failed to write hashed password");

        println!("계정 생성  성공!");
    }
}

fn login() -> Option<String>{
    let mut id = String::new();

    print!("아이디: ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin().read_line(&mut id).expect("Failed to read line");
    id = id.trim().to_string();

    print!("비밀번호: ");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut password = String::new();
    io::stdin().read_line(&mut password).expect("Failed to read line");
    password=password.trim().to_string();

    let file_path = format!("./data/usr/{}.txt", id);

    match fs::read_to_string(&file_path) {
        Ok(contents) => {
            if let Some(saved_password) = contents.lines().next() {
                let hashed_input_password = encry::encrypt_pwd(password); //암호화화

                if saved_password == hashed_input_password {
                    println!("로그인 성공!\n");
                    return Some(id);
                } else {
                    println!("로그인 실패: 비밀번호가 일치하지 않습니다.");
                }
            } else {
                println!("로그인 실패: 파일에 저장된 비밀번호가 없습니다.");
            }
        }
        Err(_) => {
            println!("로그인 실패: ID를 찾을 수 없습니다.");
        }
    }

    None
}


pub fn run(){
    loop{
        println!("1.로그인\t2.회원가입\t3.종료");
         let mut menu = String::new();
         io::stdin().read_line(&mut menu).expect("입력 실패");
         let input = menu.trim();

         match input {
            "1"=>{
                match login(){
                    Some(a) => history::see_history(a),
                    None => println!("메뉴 선택화면으로 돌아갑니다.")
                }
            }
            "2"=>{
                signin();
            }
            "3" => {
                println!("종료");
                break;
            }
            _ => {
                println!("잘못된 입력입니다.")
            }
         }
         println!("");
    }   
}

