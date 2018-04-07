
use std::io::BufRead;

fn main() {
    let input = std::io::stdin();
    let mut handler = input.lock();
    let mut items = read(&mut handler, 1);
    let cases: usize = items[0].parse().unwrap();
    for case in 1 .. cases + 1 {
        items = read(&mut handler, 2);
        let r: u8 = items[0].parse().unwrap();
        let c: u8 = items[1].parse().unwrap();
        let mut cake = Cake::new(r, c);
        for _rr in 0 .. r as usize {
            items = read(&mut handler, 1);
            for letter in items[0].bytes() {
                let cell = Cell::new(letter);
                cake.add_cell(cell);
            }
        }
        cake.gen_possibles();
        println!("Case #{}: {:#?}", case, cake);
    }
}

pub fn read(input: &mut BufRead, parts: usize) -> Vec<String> {
    let mut line = String::new();
    input.read_line(&mut line).unwrap();
    let mut items: Vec<String> = Vec::with_capacity(parts);
    for item in line.split_whitespace().take(parts) {
        items.push(String::from(item));
    }
    items
}

const LETTER_UNDEF: u8 = b'?';

#[derive(Debug)]
struct Cell {
    letter: u8,
    locked: bool,
    possibles: Vec<u8>,
}

impl Cell {
    pub fn new(letter: u8) -> Cell {
        Cell {
            letter,
            locked: (letter != LETTER_UNDEF),
            possibles: vec![],
        }
    }
}

#[derive(Debug)]
struct Cake {
    r: u8,
    c: u8,
    cells: Vec<Cell>,
    all_letters: Vec<u8>,
}

impl Cake {
    pub fn new(r: u8, c: u8) -> Cake {
        Cake {
            r,
            c,
            cells: vec![],
            all_letters: vec![],
        }
    }

    pub fn add_cell(&mut self, cell: Cell) {
        if cell.locked {
            self.all_letters.push(cell.letter);
        }
        self.cells.push(cell);
    }

    pub fn gen_possibles(&mut self) {
        for cell_index in 0 .. self.cells.len() {
            let cell_letter = self.cells[cell_index].letter;
            if self.cells[cell_index].locked {
                let mut cc = cell_index;
                while cc >= (self.c as usize) {
                    cc -= self.c as usize;
                    if self.cells[cc].letter != LETTER_UNDEF { break; }
                    self.cells[cc].possibles.push(cell_letter);
                }
                cc = cell_index;
                while (cc % self.c as usize) < (self.c as usize - 1) {
                    cc += 1;
                    if self.cells[cc].letter != LETTER_UNDEF { break; }
                    self.cells[cc].possibles.push(cell_letter);
                }
                cc = cell_index;
                while cc < self.c as usize * (self.r as usize - 1) {
                    cc += self.c as usize;
                    if self.cells[cc].letter != LETTER_UNDEF { break; }
                    self.cells[cc].possibles.push(cell_letter);
                }
                cc = cell_index;
                while (cc % self.c as usize) > 0 {
                    cc -= 1;
                    if self.cells[cc].letter != LETTER_UNDEF { break; }
                    self.cells[cc].possibles.push(cell_letter);
                }
            }
        }
    }
}



