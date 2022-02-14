use fstl;

fn main() {
    afl::fuzz!(|data: &[u8]| {
        let _ = fstl::parse_stl(&data);
    });
}
