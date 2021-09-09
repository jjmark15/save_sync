pub(crate) fn default_file_contents() -> Vec<u8> {
    vec![b'X'; 249 * 1024]
}

pub(crate) fn oversized_file_contents() -> Vec<u8> {
    vec![b'X'; 250 * 1024]
}
