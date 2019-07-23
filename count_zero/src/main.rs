use std::env;
use std::{thread, time};
extern crate chrono;
use chrono::prelude::*;
use chrono::offset::LocalResult;

fn main() {
    if env::args().count() != 4 {
        println!("请输入正确的参数: 年 月 日");
        return
    }
    let args: Vec<String> = env::args().collect();
    let year = args[1].parse::<i32>().unwrap();
    let mon = args[2].parse::<u32>().unwrap();
    let day = args[3].parse::<u32>().unwrap();
    let mut count_day = 0;
    let mut last_count_day = -1;    // 上一次计算出的结果, 如果相等不显示
    let goal = Local.ymd(year,mon,day).and_hms(0, 0, 0).timestamp();
    loop {
        let nowTimeStamp = Local::now().timestamp();
        count_day = ((goal - nowTimeStamp) / 86400) + 1;    // 距离目标天数
        if count_day != last_count_day {
            println!("距离 {} 年 {} 月 {} 日还有 {} 天",year, mon, day, count_day);
            last_count_day = count_day.clone();
        }
        thread::sleep(time::Duration::from_secs(1))
    }
}
