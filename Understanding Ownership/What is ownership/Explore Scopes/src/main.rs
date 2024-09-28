fn main() {
    let a = 0;
    {
        let b = 1;
        let c = 2;
        {
            let d = 3;
            let e = 4;
            println!("{a}{b}{c}{d}{e}");
        }
        let f = 5;
        {
            let g = 6;
            println!("{a}{b}{c}{f}{g}")
        }
        let h = 7;
        println!("{a}{b}{c}{f}{h}")
    }
    let i = 8;
    println!("ai")
}
