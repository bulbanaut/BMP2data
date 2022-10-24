use std::fs::*;
use std::io::*;
/*
!в восьми пикселях инфа идёт с 54 позиции, 19 тройки 3 цветовых значения стоит складывать
TODO: добавить разбитие на тройки, перевод в бинарку, вывод в файл
*/
fn main() {
    println!("Hello, world!");
    let mut f = File::open("/home/bulbaman/testc4.bmp").expect("жопа");
    let mut data: Vec<u8> = Vec::new();
    f.read_to_end(&mut data).expect("двойная жопа");
    let mut index: usize = 0;
    while index < 53 { //удаляет ненужные данные из начала
        data.remove(index);
        index = index + 1;
    }
    println!("{}", data[0x2]);
    let mut count: u32 = 0;
    for a in data {
        println!("{a}");
        println!("count:{count}");
        count = count + 1;
    }
    println!("final:{count}");
}
