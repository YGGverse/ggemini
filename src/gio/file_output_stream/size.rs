/// Mutable bytes count
pub struct Size {
    pub chunk: usize,
    /// `None` for unlimited
    pub limit: Option<usize>,
    pub total: usize,
}

impl Default for Size {
    fn default() -> Self {
        Self {
            chunk: 0x10000, // 64KB
            limit: None,
            total: 0,
        }
    }
}
