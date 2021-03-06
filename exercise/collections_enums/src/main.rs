
#![allow(dead_code,unused_variables,unused_mut)]

use std::collections::HashMap;

enum Shot {
    Bullseye,
    Hit(f64),
    Miss,
}

impl Shot {
    fn points(self) -> i32 {
        match self {
            Shot::Bullseye => 5,
            // Shot::Hit(x) => {
            //     if x < 3.0 {
            //         2
            //     }else{
            //         1
            //     }
            // },
            Shot::Hit(x) if x<3.0 => 2,
            Shot::Hit(x) =>1,

            Shot::Miss => 0,
        }
    }
}
fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    println!("{:?}", v);

    let mut h: HashMap<u8, bool> = HashMap::new();
    h.insert(5,true);
    h.insert(6,false);
    let have_five = h.remove(&5).unwrap();
    println!("{:?}", have_five);

    let arrow_coords : Vec<Coord> = get_arrow_coords(5);
    let mut shots:Vec<Shot> = Vec::new();

    // 2. For each coord in arrow_coords:
    //
    //   A. Call `coord.print_description()`
    //   B. Append the correct variant of `Shot` to the `shots` vector depending on the value of
    //   `coord.distance_from_center()`
    //      - Less than 1.0 -- `Shot::Bullseye`
    //      - Between 1.0 and 5.0 -- `Shot::Hit(value)`
    //      - Greater than 5.0 -- `Shot::Miss`
    for coord in arrow_coords {
        coord.print_description();
        let shot = match coord.distance_from_center() {
            x if x < 1.0 => Shot::Bullseye,
            x if x < 5.0 => Shot::Hit(x),
            _ => Shot::Miss,
            

        };
        shots.push(shot);
    }

    



    // 3. Finally, loop through each shot in shots and add its points to total

    let mut total = 0;

    for shot in shots {
        total += shot.points();
    }
    println!("Final point total is {}", total);

    // Option
    let some_number :Option<i32> = Some(5)
}

#[derive(Debug)]
struct Coord {
    x: f64,
    y: f64,
}

impl Coord {
    fn distance_from_center(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
    }
    fn print_description(&self) {
        println!(
            "coord is {:.1} away, at ({:.1}, {:.1})",
            self.distance_from_center(),
            self.x,
            self.y);
    }

}

// Generate some random coordinates
fn get_arrow_coords(num: u32) -> Vec<Coord> {
    let mut coords: Vec<Coord> = Vec::new();
    for _ in 0..num {
        let coord = Coord {
            x: (rand::random::<f64>() - 0.5) * 12.0,
            y: (rand::random::<f64>() - 0.5) * 12.0,
        };
        coords.push(coord);
    }
    coords
}