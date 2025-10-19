/// Mutable bytes count
pub struct Size {
    pub chunk: usize,
    pub limit: usize,
    pub total: usize,
}

impl Default for Size {
    fn default() -> Self {
        Self {
            chunk: 0x10000, // 64KB
            limit: 0xfffff, // 1 MB
            total: 0,
        }
    }
}
