#[derive(Default, PartialEq, Debug, Copy, Clone)]
pub(crate) enum VoltagePolarity {
    #[default]
    Positive,
    Negative,
}