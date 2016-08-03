static SCALE: usize = 8;

struct HANGYA {
    x: usize,
    y: usize,
    dir: i32,
    complete: bool, //0,1,2,3 = Fel Jobbra Le Balra
}

impl HANGYA {
    fn movement(&mut self) {
        match self.dir {
            0 => if self.y -1 > 0 {self.y-=1} else {self.complete = true},
            1 => if self.x +1 < SCALE {self.x+=1} else {self.complete = true},
            2 => if self.y +1 < SCALE {self.y+=1} else {self.complete = true},
            3 => if self.x -1 > 0 {self.x-=1} else {self.complete = true},
            _ => println!("Nem tamogatott irany!"),
        }
    }

    fn logic(&mut self, arr: &mut Vec<Vec<bool>>) {

            if arr[self.y][self.x] == true {
                arr[self.y][self.x] = false;
                if self.dir +1 > 3 {self.dir = 0;} else {self.dir += 1;}
                self.movement();
            } else {
                arr[self.y][self.x] = true;
                if self.dir -1 < 0 {self.dir = 3;} else {self.dir -= 1;}
                self.movement();
            }

    }
}

fn print_arr(arr: &Vec<Vec<bool>>) {
    for y in 0..SCALE {
        for x in 0..SCALE {
            match arr[y][x] {
                false => print!("."),
                true => print!("#"),
            }
        }
        println!("");
    }

    println!("");
}

fn main() {
    let mut terkep: Vec<Vec<bool>> = Vec::new();

    let mut teszt = HANGYA {x: SCALE/2, y: SCALE/2, dir: 0, complete: false};

    init_map(&mut terkep);

    //terkep[3][3] = true;
    while !teszt.complete {
        teszt.logic(&mut terkep);
        print_arr(&terkep);
    }
}

fn init_map(arr: &mut Vec<Vec<bool>>) {
    for _ in 0..SCALE {
        let temparr: Vec<bool> = vec![false; SCALE];

        arr.push(temparr);
    }
}
