use std::alloc::System;
#[global_allocator]
static GLOBAL: System = System;

fn main() {
    println!("foo");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
