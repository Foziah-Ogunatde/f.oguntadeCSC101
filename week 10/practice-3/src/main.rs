fn main() {
   let v = vec![20, 30, 40, 50];

   let v2 = v.clone();
   let _v2_return = display(v2.clone());
   println!("in main {:?}", v);
}

fn display(v:Vec<i32>)->Vec<i32> {
    // to return the vector to it's original owner
    println!("inside display {:?}", v);
    return v;
}