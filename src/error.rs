#[derive(Debug)]
pub struct TimeInvalidError {
    pub value: f64,
}

#[derive(Debug)]
pub struct NoFrequencyForIDError {
    pub id: usize,
}
