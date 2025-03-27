/// Mutable bytes count
pub struct Size {
    pub chunk: usize,
    /// `None` for unlimited
    pub limit: Option<usize>,
    pub total: usize,
}
