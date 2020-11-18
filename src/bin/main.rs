extern crate server;
use server::ThreadPool;

use std::net::TcpListener;
use std::io::prelude::*;
use std::net::TcpStream;
use std::fs::File;
use time;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:80").unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming(){
        let stream = stream.unwrap();
        pool.execute(||{
            handle_connection(stream);
        });
    }
}
//从请求头中解析文件路径
fn second_word(s: &String)->&str{
    let byte = s.as_bytes();
    let mut first:usize = 0;
    let second:usize;
    for (i,&elem) in byte.iter().enumerate(){
        if elem == b' '{
            if first == 0 {
                first = i;
            }else{
                second = i;
                return &s[(first + 1)..second];
            }
        }
    }
    &s[..]
}
//从路径中解析文件拓展名
fn get_ext(s: &str)->&str{
    let byte = s.as_bytes();
    for (i,&elem) in byte.iter().enumerate(){
        if elem == b'.'{
            return &s[(i + 1)..];
        }
    }
    &s[..]
}

fn safe_check(s: &str)->bool{
    let byte = s.as_bytes();
    if byte.len() == 0{
        return false;
    }
    let mut prev = byte[0];
    //检测是否返回上层
    for (i,&elem) in byte.iter().enumerate(){
        if elem == b'.'{
            if prev == elem{
                return false;
            }
        }
        prev = elem;
    }
    true
}

fn handle_connection(mut stream: TcpStream){
    //index根目录
    let root = "/var/www";
    let mut buffer = [0;512];
    stream.read(&mut buffer).unwrap();
    let buffer_to_s = String::from_utf8_lossy(&buffer[..]).to_string();
    let file_name = second_word(&buffer_to_s);
    //输出时间
    let now = time::now();
    let f_now = time::strftime("%Y-%m-%dT%H:%M:%S", &now).unwrap();
    //检查非法访问
    if !safe_check(&file_name){
        stream.write("HTTP/1.1 403 FORBIDDEN\r\n\r\n".as_bytes()).unwrap();
        stream.flush().unwrap();
        return;
    }
    println!("<b>{:?} GET {}</b>",f_now, file_name);

    if file_name == "/"{
        let mut file = match File::open(format!("{}/index.html",root)){
            Ok(_f) => _f,
            Err(_) => {
                stream.write("HTTP/1.1 404 NOT FOUND\r\n\r\n".as_bytes()).unwrap();
                stream.flush().unwrap();
                return;
            }
        };

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}",contents);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }else{
        let ext = get_ext(&file_name);
        let mut file = match File::open(format!("{}{}",root,file_name)){
            Ok(_f) => _f,
            Err(_) => {
                stream.write("HTTP/1.1 404 NOT FOUND\r\n\r\n".as_bytes()).unwrap();
                stream.flush().unwrap();
                return;
            }
        };
        if ext == "html"{
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();

            let response = format!("HTTP/1.1 200 OK\r\n\r\n{}",contents);
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }else{
            let mut buffer = [0;65535];
            while let std::io::Result::Ok(len) = file.read(&mut buffer){
    			if len == 0 {
    				break;
    			}
    			else{
                    match stream.write(&buffer){
                        Ok(_) => {

                        },
                        Err(_) => {
                            break;
                        }
                    };
    			}
            }
            stream.flush().unwrap();
        }
    }
}
