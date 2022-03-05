// run-rustfix

fn main() {
    let mut x = vec![1usize];
    x.last_mut().unwrap() = 2;
    //~^ ERROR invalid left-hand side of assignment
    x.last_mut().unwrap() += 1;
    //~^ ERROR binary assignment operation `+=` cannot be applied to type `&mut usize`
}
