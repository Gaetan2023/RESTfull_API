fn puissance (x:f64,n:i32) -> f64 {

    let  mut r:f64 ;
    if n==0{

        r=1.0;
    }else {

        r = puissance(x,n/2);
        if n%2==0{

            r= r*r;
        }else {

            r =r*r*x;
        }

        } return r;
    
}


fn main() {



    println!("3 puissance 4 = {}",puissance(3.,4));
    println!("3.5 puissance 5 = {}",puissance(3.5,5));
    println!("2 puissance 0 = {}",puissance(2.,0));
    println!("2 puissance 1 = {}",puissance(2.,1));
    println!("2 puissance 2 = {}",puissance(2.,2));
    println!("2 puissance 3 = {}",puissance(2.,3));
    println!("2 puissance 10 = {}",puissance(2.,10));
    println!("2 puissance 32 = {}",puissance(2.,32));
    println!("3  puissance 7  = {}",puissance(3.,7));
}

