#![feature(trace_macros)]

trace_macros!(true);

mod metalang;
pub use metalang::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
