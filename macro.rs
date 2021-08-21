macro_rules! hello {
    () => { println!("Hello world!!!!"); }
}

macro_rules! distance {
    ($a: ident,
     v1 => $b: expr,
     v2 => $c: expr) => {
        let $a = {
            if $b >= $c {
                $b - $c
            } else {
                $c - $b
            }
        };
    }
}

macro_rules! sum {
    ($a: ident, $b: expr, $($x: expr), *) => {
        let $a = {
            let mut temp = $b;
            $(
                temp = temp + $x;
            )*
            temp
        };
    }
}

fn main() {
    hello!();

    distance!(x, v1 => 3 + 2, v2 => 4 + 9);
    distance!(y, v1 => 9 + 1, v2 => 1 + 2);
    println!("{}, {}", x, y);

    sum!(s, 2, 3, 4, 5, 6, 6);
    println!("{}", s);

    sum!(s, 2.3, 3.5, 4.9, 5.6, 6.0, 6.1);
    println!("{}", s);

}
