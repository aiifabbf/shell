use std::env;
use std::io::stdin;
use std::io::stdout;
use std::io::Write; // flush()是Write trait的接口，所以必须use Write
use std::process::Command;

fn main() {
    loop {
        print!("$ ");
        stdout().flush().unwrap(); // flush()竟然是有可能失败的……确实，因为Write trait不只是给stdout用的，比如TcpStream也实现了Write，所以是有可能失败的

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap(); // 同理，read也可能失败的

        // 怎么处理ctrl+D呢？

        let mut parts = input.trim().split_whitespace();
        if let Some(command) = parts.next() {
            let mut args = parts;

            match command {
                "cd" => {
                    // cd是个内建指令，所以单独拿出来实现
                    let directory = args.next().unwrap_or("/"); // 如果cd后面没有跟路径，就默认cd到/
                    if let Err(_) = env::set_current_dir(&directory) {
                        eprintln!("sh: cd: {}: No such file or directory", directory);
                        // 学bash
                    }
                }
                "exit" => {
                    break;
                }
                command => {
                    // println!("{:?}", command.bytes());
                    if let Ok(mut child) = Command::new(command).args(args).spawn() {
                        child.wait().unwrap(); // 等待child process退出。有可能出错的，比如child被别人kill了
                    } else {
                        // 如果找不到binary也会失败的（一定是binary吗？有可能是脚本吗？）
                        eprintln!("{}: command not found", command); // 学bash
                    }
                }
            }
        } // 如果是空格，那么像bash一样，再提示一次$
    }
}
