use std::io;
use std::fs;
use std::error::Error;
use std::io:: Write;

pub fn see_history(id: String){
    let file_path = format!("./data/his/{}.txt", id);
    loop {
        println!("1.모든 전적 확인\t2.전적 입력\t3.특정 유저와의 전적\t4.로그아웃");
        let mut menu = String::new();
        io::stdin().read_line(&mut menu).expect("입력 실패");
        let input = menu.trim();

        match input {
            "1" => {
                println!("모든 전적");
                if let Err(e) = all(&file_path) {
                    println!("Error: {}", e);
                }
            }
            "2" => {
                println!("전적 입력");
                if let Err(e) = save_his(&file_path) {
                    println!("Error: {}", e);
                }
            }
            "3" => {
                println!("특정 유저와의 전적");
                find_his(&file_path);
            }
            "4" => break,
            _ => {
                println!("적절하지 않은 입력입니다.")
            }
        }
        println!("");
    }
}

fn all(file_path: &str) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(file_path)?;

    for line in contents.lines() {
        let parts: Vec<&str> = line.split(',').collect();
        let id = parts.get(0).unwrap_or(&"N/A");
        let win = parts.get(1).unwrap_or(&"N/A");
        let lose = parts.get(2).unwrap_or(&"N/A");
        let draw = parts.get(3).unwrap_or(&"N/A");

        if id != &"N/A" && win != &"N/A" && lose != &"N/A" && draw != &"N/A" {
            println!("상대ID:{} 승리:{} 패배:{} 무승부:{}", id, win, lose, draw);
        }
    }

    Ok(())
}

fn save_his(file_path: &str) -> io::Result<()> {
    let contents = fs::read_to_string(file_path)?;

    print!("Enemy ID: ");
    io::stdout().flush(); 
    let mut enemy_id = String::new();
    io::stdin().read_line(&mut enemy_id).expect("입력 실패");
    let enemy_id = enemy_id.trim().to_string(); 

    print!("Win: ");
    io::stdout().flush();
    let mut win = String::new();
    io::stdin().read_line(&mut win).expect("입력 실패");
    let win: i32 = win.trim().parse().expect("숫자를 입력하세요!"); 

    print!("Loss: ");
    io::stdout().flush();
    let mut loss = String::new();
    io::stdin().read_line(&mut loss).expect("입력 실패");
    let loss: i32 = loss.trim().parse().expect("숫자");

    print!("Draw: ");
    io::stdout().flush();
    let mut draw = String::new();
    io::stdin().read_line(&mut draw).expect("입력 실패");
    let draw: i32 = draw.trim().parse().expect("숫자");

    let new_data = format!("{},{},{},{}\n", enemy_id, win, loss, draw);

    let enemy_path = format!("./data/usr/{enemy_id}.txt");
    if !std::path::Path::new(&enemy_path).exists() {
        println!("{enemy_id}: 존재하지 않는 유저입니다.");
        return Ok(());
    } 

    let mut found = false;
    let mut updated_contents: Vec<String> = contents
        .lines()//한줄 씩 read
        .map(|line| { 
            let parts: Vec<&str> = line.split(',').collect(); //데이터 분리리
            if let (Some(id), Some(w), Some(l), Some(d)) = (parts.get(0), parts.get(1), parts.get(2), parts.get(3)) {
                if id == &enemy_id.as_str() {
                    found=true;
                    let existing_win: i32 = w.parse().unwrap_or(0);
                    let existing_loss: i32 = l.parse().unwrap_or(0);
                    let existing_draw: i32 = d.parse().unwrap_or(0);
                    let new_win: i32 = win + existing_win;
                    let new_loss: i32 = loss + existing_loss;
                    let new_draw: i32 = draw + existing_draw;
                    return format!("{},{},{},{}\n", enemy_id, new_win, new_loss, new_draw);
                }
            }
            line.to_string()
        })
        .collect();

    // 만약 기존에 enemy_id가 없었다면 새로운 줄을 추가
    if !found {
        updated_contents.push(new_data);
    }

    // 파일에 다시 작성
    let mut file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)  // 기존 내용을 덮어씁니다.
        .open(file_path)?;
    
    for line in updated_contents {
        writeln!(file, "{}", line)?;
    }

    Ok(())
}
fn find_his(file_path: &str) -> Result<(), Box<dyn std::error::Error>>{
    let contents = fs::read_to_string(file_path)?;

    print!("Enemy ID: ");
    io::stdout().flush(); 
    let mut enemy_id = String::new();
    io::stdin().read_line(&mut enemy_id).expect("입력 실패");
    let enemy_id = enemy_id.trim().to_string(); 


    let enemy_path = format!("./data/usr/{enemy_id}.txt");
    if !std::path::Path::new(&enemy_path).exists() {
        println!("{enemy_id}: 존재하지 않는 유저입니다.");
        return Ok(());
    } 

    let updated_contents: Vec<&str> = contents.lines().filter(|line| line.contains(&enemy_id)).collect();

    for line in updated_contents{
        let parts: Vec<&str> = line.split(',').collect();
        let id = parts.get(0).unwrap_or(&"N/A");
        let win = parts.get(1).unwrap_or(&"N/A");
        let lose = parts.get(2).unwrap_or(&"N/A");
        let draw = parts.get(3).unwrap_or(&"N/A");

        if id != &"N/A" && win != &"N/A" && lose != &"N/A" && draw != &"N/A" {
            println!("상대ID:{} 승리:{} 패배:{} 무승부:{}", id, win, lose, draw);
        }
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
}
