/// Contains argument flags and search chunks.
pub struct State {
    pub all_files: bool,
    pub has_search: bool,
    pub help: bool,
    pub is_tty: bool,
    pub porcelain: bool,
    pub search_chunks: Vec<String>
}

pub fn default_state() -> State {
    return State {
        all_files: false,
        has_search: false,
        help: false,
        is_tty: true,
        porcelain: false,
        search_chunks: vec!(),
    };
}
