use crate::circuits::series::Series;

#[derive(PartialEq, Debug, Clone)]
pub struct Parallel {
    pub side1: Vec<Series>,
    pub side2: Vec<Series>
}

impl Parallel {
    pub fn new(side1: Vec<Series>, side2: Vec<Series>) -> Parallel {
        Parallel { side1, side2 }
    }
}

#[cfg(test)]
mod tests {
    use crate::circuits::parallel::Parallel;

    #[test]
    fn test_new() {
        let side1 = vec![];
        let side2 = vec![];
        let parallel = Parallel::new(side1, side2);
        assert_eq!(parallel, Parallel { side1: vec![], side2: vec![] });
    }
}