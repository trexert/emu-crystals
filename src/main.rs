#![feature(generic_const_exprs)]
#![feature(adt_const_params)]
mod kyber;

fn main() {
    println!("{}", size_of::<Key<1024>>())
}
