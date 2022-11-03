#[derive(Default, Debug, Clone)]
pub struct ResetRange {
    pub(crate) begin_seq_num: u32,
    pub(crate) end_seq_num: u32,
    pub(crate) chunk_end_seq_num: Option<u32>,
}
