use std::vec::Vec;
use rand::Rng;
use std::{thread, time};
use std::env;

#[derive(Debug)]
struct Board {
    width: i32,
    height: i32,
    cells: Vec<u8>
}
impl Board {
    fn create(width: i32, height: i32) -> Board {
        let mut board = Board {
            width,
            height,
            cells: Vec::with_capacity((width * height) as usize)
        };
        for _i in 0..(width * height) {
            board.cells.push(0);
        }

        return board;
    }

    fn get_cell(&self, x: i32, y: i32) -> Option<&u8> {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.cells.get((x + (y * self.width)) as usize)
        } else {
            Option::from(&(0 as u8))
        }
    }

    fn set_cell(&mut self, x: i32, y: i32, state: u8) {
        self.cells[(x + (y * self.width)) as usize] = state;
    }

    fn print(&self) {
        print!("\x1B[2J\x1B[1;1H");
        for y in 0..(self.height) {
            for x in 0..(self.width) {
                if let Some(z) = self.get_cell(x, y) {
                    if *z == 0 {
                        print!(".");
                    } else {
                        print!("#");
                    }
                } else {
                    print!(".");
                }
            }
            print!("\n");
        }
    }

    fn tick(self) -> Board {
        let mut future = Board::create(self.width, self.height);
        for x in 0..self.width {
            for y in 0..self.height {
                let mut alive_neighbours = 0;

                for nx in -1..2 {
                    for ny in -1..2 {
                        if let Some(neighbour) = self.get_cell(x + nx, y + ny) {
                            alive_neighbours += neighbour;
                        }
                    }
                }

                if let Some(myself) = self.get_cell(x, y) {
                    alive_neighbours -= myself;

                    if alive_neighbours < 2 && *myself == 1 {
                        future.set_cell(x, y, 0);
                    } else if *myself == 1 && alive_neighbours > 3 {
                        future.set_cell(x, y, 0);
                    } else if *myself == 0 && alive_neighbours == 3 {
                        future.set_cell(x, y, 1);
                    } else {
                        future.set_cell(x, y, *myself);
                    }
                }
            }
        }

        return future;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut width = 80;
    let mut height = 24;
    if args.len() == 3 {
        if let Ok(new_width) = args[1].parse::<i32>() {
            if let Ok(new_height) = args[2].parse::<i32>() {
                width = new_width;
                height = new_height;
            }
        }
    }

    let mut rng = rand::thread_rng();
    let mut board = Board::create(width, height);

    for i in 0..(board.cells.len()) {
        board.cells[i] = rng.gen_range(0, 2);
    }

    let fifty_millis = time::Duration::from_millis(500);

    loop {
        board.print();
        board = board.tick();
        thread::sleep(fifty_millis);
    }
}
