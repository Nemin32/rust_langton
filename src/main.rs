static SCALE: usize = 64; // Scale of the map as an unchangable constant

struct ANT {
    x: usize,
    y: usize, //Posiion of our ant
    dir: i8, //The ant's direction
    complete: bool, // Have we finished?
}

impl ANT {
    fn movement(&mut self) {
        match self.dir {
            //We check if our ant can move in the current direction, if he can't we set complete to be true
            0 => if self.y -1 > 0 {self.y-=1} else {self.complete = true}, //Up
            1 => if self.x +1 < SCALE {self.x+=1} else {self.complete = true}, //Right
            2 => if self.y +1 < SCALE {self.y+=1} else {self.complete = true}, //Down
            3 => if self.x -1 > 0 {self.x-=1} else {self.complete = true}, //Left
            _ => panic!("Wrong direction!"), //This should never happen, but Rust demands to have a case for it.
        }
    }

    fn logic(&mut self, arr: &mut Vec<Vec<bool>>) {
        arr[self.y][self.x] = !arr[self.y][self.x]; //Flip the bool: True -> False and vice-versa

        if arr[self.y][self.x] == true { //We check the map's boolean at the ant's coordinates.
            if self.dir +1 > 3 {self.dir = 0;} else {self.dir += 1;} //Turn the ant right
        } else {
            if self.dir -1 < 0 {self.dir = 3;} else {self.dir -= 1;} //Turn the ant left
        }

        self.movement(); //Do one step
    }
}

fn print_arr(arr: &Vec<Vec<bool>>) {
    for y in 0..SCALE {
        for x in 0..SCALE { //Loop trough our entire map
            match arr[y][x] {
                false => print!("."), //Print the appropriate character
                true => print!("#"),
            }
        }
        println!("");
    }
    println!(""); //Print one more newline character, else it would look ugly
}

fn main() {
    let mut playground: Vec<Vec<bool>> = Vec::new(); //A simple 2D Vector made up from Bools

    let mut test = ANT {x: SCALE/2, y: SCALE/2, dir: 0, complete: false}; //A test ant

    init_map(&mut playground); //We set up our Vector to have a size of SCALE*SCALE

    while !test.complete { //Until our ant hits the edge of the map
        test.logic(&mut playground); //Get a new direction and move one step
        print_arr(&playground); //Show the current iteration on the Console
    }

}

fn init_map(arr: &mut Vec<Vec<bool>>) { 
    for _ in 0..SCALE { //SCALE times...
        let temparr: Vec<bool> = vec![false; SCALE];
        arr.push(temparr); //... push a Vector with SCALE number of elements into our map
    }
}
