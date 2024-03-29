// --- What ---
// This program is not a demonstration of any particular Rust aspect. This
// program is a demonstration of a possible implementation for a 3D grid using
// rhombic dodecahedral units instead of cubes. This implementation takes cues
// from "https://www.redblobgames.com/grids/hexagons/"
//
// The coordinate system is built up with "tesseract coordinates", with a struct
// called RCoord which utilizes w, x, y, and z axes. Moving in a positive
// direction along an axis is possible for three adjacent faces. The axis being
// traversed increases its corresponding coordinate and decreases one of the
// other three. This satisfies the constraint for every valid coordinate point
// that w + x + y + z = 0.

// --- Why ---
// I've seen hexagonal prisms used as an alternative to cubes for
// "Minecraft-like" games. I've also seen attempts at spherical worlds, which
// splinter like the bottom of photo spheres as you dig into the world. It would
// be interesting to see a world built out with rhombic dodecahedrons, where the
// rotation is slightly more subtle than a 90 degree rotation over the edge of
// cube planet.

use std::cmp::*;

pub fn rhombic_dodeca() {
    use RDir::*;

    let mut t = RCoord::from([1, 0, 0, -1]);
    t.as_coord_from([0, 1, -1, 0]);

    let x = t.add_from([2, 0, 0, -2]);
    t = x.add(&t.neg());
    println!("{:?}\n", t.coord());

    let mut t = RCoord::from([0, 0, 0, 0]);
    println!("{:?}", t.coord());
    t.move_from_to(W, X);
    println!("{:?}", t.coord());
    t.move_from_to(X, Y);
    println!("{:?}", t.coord());
    t.move_from_to(Y, Z);
    println!("{:?}", t.coord());
    t.move_from_to(X, W);
    println!("{:?}", t.coord());
    t.move_from_to(Y, X);
    println!("{:?}", t.coord());
    t.move_from_to(Z, Y);
    println!("{:?}", t.coord());

    let x = t.clone();
    t.move_from_to(Z, W);
    t.move_from_to(Y, X);
    println!(
        "The manhattan distance from {:?} to {:?} is {}\n",
        x.coord(),
        t.coord(),
        x.distance(&t)
    );

    for rc in x.get_all_adjacent() {
        println!("{:?}", rc.coord());
    }
    println!("\n");

    let x = RCoord::from([0, 0, 0, 0]);
    for rc in x.line_to(&RCoord::from([5, 5, -5, -5])) {
        println!("{:?}", rc.coord())
    }
    println!("\n");

    // Work in progress
    for rc in x.get_in_range(1) {
        println!("{:?} is in range", rc.coord());
    }
}

#[derive(Clone)]
struct RCoord {
    w: i32,
    x: i32,
    y: i32,
    z: i32,
}

impl RCoord {
    fn from(a: [i32; 4]) -> RCoord {
        RCoord::if_valid(RCoord {
            w: a[0],
            x: a[1],
            y: a[2],
            z: a[3],
        })
    }

    fn coord(&self) -> [i32; 4] {
        [self.w, self.x, self.y, self.z]
    }

    fn add_from(&self, a: [i32; 4]) -> RCoord {
        RCoord::from([self.w + a[0], self.x + a[1], self.y + a[2], self.z + a[3]])
    }

    fn add(&self, rc: &RCoord) -> RCoord {
        self.add_from(rc.coord())
    }

    fn neg(&self) -> RCoord {
        RCoord::from([-self.w, -self.x, -self.y, -self.z])
    }

    fn abs(&self) -> [i32; 4] {
        [self.w.abs(), self.x.abs(), self.y.abs(), self.z.abs()]
    }

    fn get_from_to(&self, f: RDir, t: RDir) -> RCoord {
        self.add_from(RDir::from_to(f, t))
    }

    fn move_from_to(&mut self, f: RDir, t: RDir) {
        self.as_coord(&self.get_from_to(f, t));
    }

    fn distance_from(&self, a: [i32; 4]) -> i32 {
        self.add(&RCoord::from(a).neg()).abs().iter().sum::<i32>() / 2
    }

    fn distance(&self, rc: &RCoord) -> i32 {
        self.distance_from(rc.coord())
    }

    fn as_coord_from(&mut self, a: [i32; 4]) {
        self.w = a[0];
        self.x = a[1];
        self.y = a[2];
        self.z = a[3];

        self.check()
    }

    fn as_coord(&mut self, rc: &RCoord) {
        self.as_coord_from(rc.coord());
    }

    fn if_valid(rc: RCoord) -> RCoord {
        rc.check();
        rc
    }

    fn check(&self) {
        if self.coord().iter().sum::<i32>() != 0 {
            panic!(
                "\n!Invalid coordinates!\n\t{:?} should sum to 0\n",
                self.coord()
            );
        }
    }

    fn lerp_to(&self, rc: &RCoord, t: f64) -> RCoord {
        let mut x = [
            lerp(self.w as f64, rc.w as f64, t).round() as i32,
            lerp(self.x as f64, rc.x as f64, t).round() as i32,
            lerp(self.y as f64, rc.y as f64, t).round() as i32,
            lerp(self.z as f64, rc.z as f64, t).round() as i32,
        ];
        let y = self.coord();
        if x.iter().sum::<i32>() != 0 {
            for i in 0..3 {
                if x[i] == x[i + 1] {
                    match x[i].cmp(&y[i]) {
                        Ordering::Less => x[i] += 1,
                        Ordering::Equal => {}
                        Ordering::Greater => x[i] -= 1,
                    }
                    break;
                }
            }
        }
        RCoord::from(x)
    }

    // The 45 degree diagonals contain duplicate coords at the moment as, unlike
    // hex coords, rhombic dodecahedral cells at a MH dist. of 2 from each other
    // can share a vertex.
    fn line_to(&self, rc: &RCoord) -> Vec<RCoord> {
        let distance = self.distance(rc);
        let mut v: Vec<RCoord> = Vec::new();
        for i in 0..=distance {
            v.push(self.lerp_to(rc, 1.0 / (distance as f64) * i as f64))
        }

        v
    }

    // Work in progress
    fn get_in_range(&self, range: i32) -> Vec<RCoord> {
        let mut v: Vec<RCoord> = Vec::new();
        for x in -range..=range {
            for y in max(-range, -x - range)..=min(range, -x + range) {
                for z in max(-range, -x - y - range)..=min(range, -x - y + range) {
                    let w = -x - y - z;
                    v.push(self.add(&RCoord::from([w, x, y, z])));
                }
            }
        }

        v
    }

    fn get_all_adjacent(&self) -> [RCoord; 12] {
        use RDir::*;
        [
            self.get_from_to(W, X),
            self.get_from_to(W, Y),
            self.get_from_to(W, Z),
            self.get_from_to(X, W),
            self.get_from_to(X, Y),
            self.get_from_to(X, Z),
            self.get_from_to(Y, W),
            self.get_from_to(Y, X),
            self.get_from_to(Y, Z),
            self.get_from_to(Z, W),
            self.get_from_to(Z, X),
            self.get_from_to(Z, Y),
        ]
    }
}

fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}

#[derive(Debug)]
enum RDir {
    W,
    X,
    Y,
    Z,
}

impl RDir {
    fn from_to(f: RDir, t: RDir) -> [i32; 4] {
        use RDir::*;
        match (f, t) {
            (W, X) => [-1, 1, 0, 0],
            (W, Y) => [-1, 0, 1, 0],
            (W, Z) => [-1, 0, 0, 1],

            (X, W) => [1, -1, 0, 0],
            (X, Y) => [0, -1, 1, 0],
            (X, Z) => [0, -1, 0, 1],

            (Y, W) => [1, 0, -1, 0],
            (Y, X) => [0, 1, -1, 0],
            (Y, Z) => [0, 0, -1, 1],

            (Z, W) => [1, 0, 0, -1],
            (Z, X) => [0, 1, 0, -1],
            (Z, Y) => [0, 0, 1, -1],

            (_, _) => panic!("\nCannot go from and to the same direction.\n"),
        }
    }
}
