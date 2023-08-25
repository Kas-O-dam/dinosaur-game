
extern crate mairs;
extern crate time;
extern crate thread;
extern crate console;
extern crate rand;

//use std::fs::File;
//use std::io::Read;
use rand::prelude::*;
use mairs::Field;
use std::time::Duration;
use std::thread::sleep;
use console::Term;

fn cactus_generator() -> (usize, usize){
    let mut rng: ThreadRng = thread_rng();
    let cactus_size: usize = rng.gen_range(0..=1);
    let offset_to_cactus: usize = rng.gen_range(0..=16);
    (cactus_size, offset_to_cactus)
}

fn mainloop(field: &mut Field){
    let cactus: Vec<Vec<char>> = vec![
        vec![' ', ' ', '#', '#', ' ', ' '],
        vec!['#', ' ', '|', '|', ' ', ' '],
        vec!['\\', '\\', '|', '|', ' ', '#'],
        vec![' ', '\'', '|', '|', '/', '/'],
    ];
    let terminal = Term::stdout();
    let mut cactuses_positions:Vec<[usize; 2]> = vec![[26, 10]];
    loop {
        let duration = Duration::new(0, 250_000_000);
        sleep(duration); // sleeping
        let mut cactus_layer: Vec<Vec<char>> = field.build_layer(); // clearing for moving of cactuses
        match terminal.clear_screen() {
            Ok(_) => {},
            Err(_) => {eprintln!("[ ERROR ] failed clearing screen")}
        };
        if cactuses_positions[0][0] == 0 {cactuses_positions[0][0] = 27};
        cactuses_positions[0][0] -= 1;
        cactus_layer = field.paste(&cactus_layer, &cactus, cactuses_positions[0]);
        field.seq.pop();
        field.seq.push(cactus_layer);
        field.print();
        //then, you can working 😌
    }
}

fn main(){
    /*
      ##
    # ||
    \\|| #
     '||//
    vec![
        vec![' ', ' ', '#', '#', ' ', ' '],
        vec!['#', ' ', '|', '|', ' ', ' '],
        vec!['\\', '\\', '|', '|', ' ', '#'],
        vec![' ', '\'', '|', '|', '/', '/'],
    ];

    #@
    || #
    ||//

    <(° )
    <//=
    /|| 
    vec![
        vec!['<', '(', '°', ' ', ')'],
        vec!['<', '/', ' ', '=', ' '],
        vec!['/', '|', '|', ' ', ' ']
    ];
    */
    let dino: Vec<Vec<char>> = vec![
        vec!['<', '(', '°', ' ', ')'],
        vec!['<', '/', ' ', '=', ' '],
        vec!['/', '|', '|', ' ', ' ']
    ];
    let mut field: Field = Field {x: 32, y: 16, seq: Vec::new(), default_char: ' '};
    let background: Vec<Vec<char>> = field.horizontal([0, 32], 14, '*');
    let mut dino_layer: Vec<Vec<char>> = field.build_layer();
    dino_layer = field.paste(&dino_layer, &dino, [0, 11]);
    field.seq.push(background);
    field.seq.push(dino_layer);
    field.seq.push(Vec::new());
    //field.print();
    mainloop(&mut field);
}
