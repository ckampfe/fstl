# fstl

Load a binary STL file very quickly

## SAFETY WARNING / should you use this

This library basically transmutes a slice of bytes (`&[u8]`) directly into a slice of triangles (`&[Triangle]`). This happens without parsing/lexing, meaning that loading the STL only costs approximately as much as it costs to perform the IO to load the STL into memory in the first place. The tradeoff is that in order to do this, this library uses `unsafe`.

To ensure correctness in this usage of `unsafe`, it makes a few assumptions about the input STL:
- The STL must be a binary STL; ASCII STLs are not supported.
- The STL header section must correctly report the number of triangles contained in the rest of the input.
- The length in bytes of the triangles section of the STL files is exactly equal to `NUMBER_OF_TRIANGLES * 50`.
- All numerics must be little-endian encoded.
- All numerics must fit within their corresponding `u32`, `f32`, and `u16` numerics in Rust (i.e., no overflow, etc). See the representation of `Triangle` in `lib.rs`.

Failing to meet some of these assumptions will cause the program to return an error.

More importantly, failing to meet other assumptions will cause the program to load and report incorrect values silently. For example, if numerics are not litte-endian encoded or they are too large for their corresponding in-memory type, the program will load them anyway and their values will be silently incorrect.

If you know your inputs meet the assumptions above, `fstl` might work for your use case. Otherwise, you ought to use something like [nom_stl](https://github.com/fast-radius/nom_stl).

I've fuzzed `fstl` with [afl++](https://github.com/AFLplusplus/AFLplusplus) and have (so far) not been able to cause it to crash or hang (see the `fuzz` directory). It also does not report any undefined behavior when running the tests under [Miri](https://github.com/rust-lang/miri). Fuzzing and Miri are not proof that `fstl` (or any other Rust program) does not exhibit UB, but they give you more information for you to make your own choice as to whether `fstl` is safe enough for your use case.
