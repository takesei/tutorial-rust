#[derive(Debug)]
struct Vector2D {
    x: f64,
    y: f64,
}

impl Vector2D {
    fn norm(&self) -> f64 {
        (self.x.powf(2.) + self.y.powf(2.)).sqrt()
    }

    fn unit() -> Self {
        Self { x: 1., y: 0. }
    }
}

pub fn sample() {
    let mut x: i32 = 5;
    readout_int(x, 'x');
    x = 6;
    readout_int(x, 'x');

    const A: i32 = 10;
    readout_int(A, 'A');

    {
        // Scope is set in block
        // Shadowing ... redifinition of var x
        let x = x + 100;
        println!("Shadowing:: value is {}", x)
    }
    readout_int(x, 'x');
    println!("The value is {}", 6_f64 / 1.5);

    let t: (usize, usize, usize) = (1, 2, 3);
    let (x1, x2, x3) = t;
    println!(
        "vec is ({} {} {}), contains {}, {}, {}",
        x1, x2, x3, t.0, t.1, t.2
    );

    let a: [isize; 3] = [1, 2, 3];
    let aa = [3; 5];
    println!("{}, {}", a[0], aa[3]);

    let s = get_ref();
    let mut cp_s = s;
    // println!("{}", s);
    println!("{}", cp_s);
    pass_ref(&mut cp_s);

    let v = Vector2D::unit();
    println!("Vector {:?} has norm {}", v, v.norm())
}

fn readout_int(val: i32, name: char) -> String {
    println!("The value of {} is {}", name, val);
    format!("{}={}", name, val)
}

fn get_ref() -> String {
    String::from("hogehoge")
}

fn pass_ref(s: &mut String) {
    s.push_str("@hogehoge");
}
