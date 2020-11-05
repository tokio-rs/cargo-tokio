mod ci;

pub use ci::{TokioCIStageBuilder, TokioCIStep};

#[cfg(test)]
mod tests {
    #[test]
    fn bogus_test() {
        assert_eq!(1, 1)
    }
}
