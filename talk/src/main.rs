// extern crate regex;

use std::io;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use std::vec::Vec;

struct Question {
    id: i64,
    que: String,
    ans: Vec<Answer>,
}

struct Answer {
    ans: String,
    class: u8,
}

impl Question {
    fn new() -> Question {
        Question {
            id: timestamp1(),
            que: String::new(),
            ans: Vec::new(),
        }
    }
    fn set_que(&mut self, que: String) {
        self.que = que;
    }
    fn set_ans(&mut self, answer: Answer) {
        self.ans.push(answer);
    }
}

fn main() {
    fn print_refs(x: &i32, y: &i32) {
        println!("{},{}", x, y);
    }
    let (four, nine) = (4, 9);
    print_refs(&four, &nine);
}

fn create_question() -> Question {
    let que = input_que();
    let ans = input_ans();
    let mut question = Question::new();
    question.set_que(que);
    question.set_ans(ans);
    return question;
}

// 从用户创建一个问题和答案
fn create_question_from_stdin() -> Question {
    let que = input_que();
    let ans = input_ans();
    let mut question = Question::new();
    question.set_que(que);
    question.set_ans(ans);
    return question;
}

//输入问题
fn input_que() -> String {
    let mut buf = String::new();
    println!("请输入你的问题");
    read_from_stdin(&mut buf).unwrap();
    return buf;
}

// 输入答案
fn input_ans() -> Answer {
    let mut buf2 = String::new();
    println!("请输入你的答案",);
    read_from_stdin(&mut buf2).unwrap();

    let mut class = String::new();
    println!("请输入情感级别(0-10)",);
    read_from_stdin(&mut class).unwrap();
    let class: u8 = class.trim().parse().unwrap();
    return Answer {
        ans: buf2,
        class: class,
    };
}

// 读取用户输入
fn read_from_stdin(buf: &mut String) -> io::Result<()> {
    if let Err(e) = io::stdin().read_line(buf) {
        return Err(e);
    };
    Ok(())
}

// 获取当前时间戳
fn timestamp1() -> i64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("time went backwards");
    let ms = since_the_epoch.as_secs() as i64 * 1000i64
        + (since_the_epoch.subsec_nanos() as f64 / 1_000_000.0) as i64;
    return ms;
}
