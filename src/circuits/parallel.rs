use crate::circuits::series::UniSeries;

#[derive(PartialEq, Debug, Clone)]
pub struct UniParallel {
    pub components: Vec<UniSeries>,
}

impl UniParallel {
    pub fn new(components: Vec<UniSeries>) -> UniParallel {
        UniParallel { components }
    }
}

#[cfg(test)]
mod tests {
    use crate::circuits::parallel::UniParallel;

    #[test]
    fn test_new() {
        let components = vec![];
        let parallel = UniParallel::new(components);
        assert_eq!(parallel, UniParallel { components: vec![] });
    }
}