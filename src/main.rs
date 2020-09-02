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

        // println!("{:?}", input.bytes());
        // 怎么处理ctrl+D呢？
        // 似乎ctrl+D是不等\n，直接把终端输入缓冲区里的东西交给应用程序
        // <https://unix.stackexchange.com/questions/110240/why-does-ctrl-d-eof-exit-the-shell>
        if input.is_empty() {
            eprintln!("^D");
            break;
        }
        // 测试的时候发现，没有输入任何内容的时候，按一下ctrl+D，应用程序就会马上反应没有读到任何东西，input.bytes()是空的；输入了一些内容、不按回车的时候，需要按两下ctrl+D，应用程序才会读到不带\n的内容
        // 有人说和tty的模式有关
        // <https://en.wikipedia.org/wiki/POSIX_terminal_interface>

        // 用printf a | cargo run测试的时候发现，最后会读到空input.bytes()

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
