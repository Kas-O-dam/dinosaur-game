
extern crate mairs;
extern crate time;
extern crate thread;
extern crate console;

use mairs::Field;
use std::time::Duration;
use std::thread::sleep;
use console::Term;


fn mainloop(){
    let terminal = Term::stdout();
    loop {
        let duration = Duration::new(0, 500_000_000);
        sleep(duration);
        terminal.clear_screen();
        //then, you can working 😌
    }
}

fn main(){
    let mut field = Field {x: 16, y: 8, seq: Vec::new(), default_char: ' '};
    let layer = field.horizontal([0, 16], 7, '#');
    field.seq.push(layer);
    mainloop();
}
