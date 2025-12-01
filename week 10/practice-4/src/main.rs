fn main() {
   let v = vec![20, 40, 60, 80, 100];
   print_vector(v);
   println!("{}", v[0]);
}
fn print_vector(x: Vec<i32>)->impl Clone() {
   println!("Inside print_vector function {:?}", x);    
}
