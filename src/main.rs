fn main() {

    let a: u8 = 1;
    let b = a.to_string();
    let c= my_awesome_vec(b);
    
    println!("Hello, Cringelord {}, {}!", a , c[0]);
}

fn my_awesome_vec(c:String)->Vec<String>{
    let mut b = Vec::new();
    b.push(c);
    b
}