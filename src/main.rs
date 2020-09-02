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
        let command = input.trim();
        let mut child = Command::new(command).spawn().unwrap(); // 如果找不到binary也会失败的（一定是binary吗？有可能是脚本吗？）

        child.wait().unwrap(); // 等待child process退出。有可能出错的，比如child被别人kill了
    }
}
