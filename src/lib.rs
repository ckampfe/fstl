//! UINT8[80]    – Header                 -     80 bytes                           
//! UINT32       – Number of triangles    -      4 bytes
//!
//! foreach triangle                      - 50 bytes:
//!     REAL32[3] – Normal vector             - 12 bytes
//!     REAL32[3] – Vertex 1                  - 12 bytes
//!     REAL32[3] – Vertex 2                  - 12 bytes
//!     REAL32[3] – Vertex 3                  - 12 bytes
//!     UINT16    – Attribute byte count      -  2 bytes
//! end

use std::error::Error;

const HEADER_LENGTH_BYTES: usize = 80;
const NUMBER_OF_TRIANGLES_LENGTH_BYTES: usize = 4;

macro_rules! ensure_length {
    ($bytes:expr, $len:expr) => {
        if $bytes.len() < $len {
            return Err(format!("Not enough bytes, needed: {}", $len - $bytes.len()).into());
        }
    };
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Triangle {
    pub normal_vector: [f32; 3], // 12
    pub vertices: [[f32; 3]; 3], // 36
    _attribute_byte_count: u16,  // 2
}

pub fn parse_stl(bytes: &[u8]) -> Result<&[Triangle], Box<dyn Error>> {
    ensure_length!(bytes, HEADER_LENGTH_BYTES);

    let (_header, rest) = bytes.split_at(HEADER_LENGTH_BYTES);

    ensure_length!(rest, NUMBER_OF_TRIANGLES_LENGTH_BYTES);

    let (number_of_triangles, rest) = rest.split_at(NUMBER_OF_TRIANGLES_LENGTH_BYTES);

    let number_of_triangles = u32::from_le_bytes(number_of_triangles.try_into().unwrap()) as usize;

    let expected_remaining_bytes = number_of_triangles * std::mem::size_of::<Triangle>();

    ensure_length!(rest, expected_remaining_bytes);

    let (prefix, triangles, _rest) = unsafe { rest.align_to::<Triangle>() };

    assert!(prefix.is_empty(), "Data was not aligned");

    Ok(triangles)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;

    #[test]
    fn does_binary_from_file() {
        // credit: https://www.thingiverse.com/thing:26227
        let mut root_vase = vec![];
        std::fs::File::open("./fixtures/Root_Vase.stl")
            .unwrap()
            .read_to_end(&mut root_vase)
            .unwrap();
        dbg!(root_vase.len());

        let triangles = parse_stl(&mut root_vase).unwrap();

        assert_eq!(triangles.len(), 596_736);

        assert_eq!(
            triangles[999],
            Triangle {
                normal_vector: [0.49034846, -0.87116635, -0.025051892,],
                vertices: [
                    [8.868585, -18.981253, 47.16066,],
                    [8.118673, -19.399504, 47.02687,],
                    [8.171986, -19.36216, 46.77175,],
                ],
                _attribute_byte_count: 0u16
            }
        )
    }
}
