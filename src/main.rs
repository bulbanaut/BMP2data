use std::fs::*;
use std::io::*;
/*
в восьми пикселях инфа идёт с 54 позиции, 19 тройки 3 цветовых значения стоит складывать
TODO: вывод в файл, возможность считывать и записывать множество файлов за запуск
!логика сломана, переделать
*/
fn main() {
    println!("kill me");
    let mut f = File::open("/home/bulbaman/testc4.bmp").expect("жопа");
    let mut data: Vec<u8> = Vec::new();
    f.read_to_end(&mut data).expect("двойная жопа");
    let mut index: usize = 0;
    while index < 54 { //удаляет ненужные данные из начала
        data.remove(0);
        index = index + 1;
    }
    //println!("{}", data[0x2]);
    let mut count: u32 = 1;
    let mut total_color: u32 = 0;
    let mut binary: Vec<bool> = Vec::new();
    for a in data {
        println!("count:{count}");
        println!("{a}");
        if count % 3 == !0 {
            total_color = total_color + a as u32;
        } else {
            total_color = total_color + a as u32;
            if total_color == 0 {
                binary.push(false);
            } else {
                binary.push(true);
            }
            total_color = 0;
        }
        count = count + 1;
    }
    println!("final:{count}");
    count = 0;
    for a in binary {
        println!("{a}");
        count = count + 1;
    }
    println!("{count}");
}
