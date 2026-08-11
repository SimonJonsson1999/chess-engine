

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bound {
    Exact,
    Lower,
    Upper,
}

#[derive(Clone, Copy, Debug)]
pub struct TTEntry {
    pub hash: u64,
    pub depth: u8,
    pub score: i32,
    pub bound: Bound,
}
pub struct TranspositionTable {
    entries: Vec<Option<TTEntry>>,
}

impl TranspositionTable {
    pub fn new(size_mb: usize) -> Self {
        let bytes = size_mb * 1024 * 1024;
        let entry_size = std::mem::size_of::<Option<TTEntry>>();
        let num_entries = bytes / entry_size;

        Self {
            entries: vec![None; num_entries],
        }
    }
    pub fn get(&self, hash: u64) -> Option<&TTEntry> {
        let index = hash as usize % self.entries.len();

        match &self.entries[index] {
            Some(entry) if entry.hash == hash => Some(entry),
            _ => None,
        }
    }
    pub fn store(&mut self, entry: TTEntry) {
        let index = entry.hash as usize % self.entries.len();
        self.entries[index] = Some(entry);
    }
    pub fn clear(&mut self) {
        self.entries.fill(None);
    }
}