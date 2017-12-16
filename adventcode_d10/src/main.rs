fn main() {
    let input = [
        129, 154, 49, 198, 200, 133, 97, 254, 41, 6, 2, 1, 255, 0, 191, 108
    ];

    let mut inc = 0;
    let mut pos = 0;

    let mut tab: [usize; 256] = [0; 256];
    for i in 0..256 {
        tab[i] = i;
    }

    let my_init: Vec<i32> = (0..256).collect();
    for my in &my_init {
        print!("{}, ", my);
    }
    println!("");

    println!("{}", my_init[32]);

    println!("{}", tab[0]);
}
