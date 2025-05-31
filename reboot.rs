use std::f64::consts;

fn stuff(x: f64) -> f64{
    return x+x*x;
}

fn add_ref(x: &mut i32){
    *x += 1;
}

fn main(){
    //let name = "guy";
    let mut total = 0.0;
    let mut count = 0;
    let arr = [1,1,342,1];
    for i in 0..6{
        add_ref(&mut count);
        total += i as f64 + stuff(0.1);
        let text = if i == 4 {&consts::PI.to_string()} else {&total.to_string()};
        println!("text: {}", text);
        println!("count: {}", count);
        println!("debug print {:?}", arr);
    }
}