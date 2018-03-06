use std::str;

use nom::{le_u16, le_u32, le_f32};

use super::{Vector3D, Facet, Solid};

named!(parse_vector<&[u8], Vector3D>, do_parse!(
    x: le_f32 >>
    y: le_f32 >>
    z: le_f32 >>
    (Vector3D{x: x as f64, y: y as f64, z: z as f64})
));

named!(parse_facet<&[u8], Facet>, do_parse!(
    normal: parse_vector >>
    v1: parse_vector >>
    v2: parse_vector >>
    v3: parse_vector >>
    attribute: le_u16 >>
    (
        Facet{
            normal: normal,
            vertices: vec![v1, v2, v3],
            attribute: attribute,
        }
    )
));

named!(parse_solid<&[u8], Solid>, do_parse!(
    header: take!(80) >>
    num_of_triangles: le_u32 >>
    faces: many1!(parse_facet) >>
    ({
        let maybe_str = str::from_utf8(header);
        let name = if let Ok(name) = maybe_str {name.trim()} else {"unnamed"};

        Solid{
            name: name.to_string(),
            faces: faces,
        }
    })
));

#[cfg(test)]
mod test_vector {
    use nom::{IResult};
    use byteorder::{LE, WriteBytesExt};

    use super::{Vector3D, parse_vector};

    #[test]
    fn parse_vector1() {
        let mut input: Vec<u8> = vec![];

        input.write_f32::<LE>(-1.0).unwrap();
        input.write_f32::<LE>(2.0).unwrap();
        input.write_f32::<LE>(-3.5).unwrap();

        let expected_output = IResult::Done(&b""[..], Vector3D{x: -1.0, y: 2.0, z: -3.5});
        let result = parse_vector(&input);

        assert_eq!(result, expected_output);
    }
}

#[cfg(test)]
mod test_facet {
    use nom::{IResult};
    use byteorder::{LE, WriteBytesExt};

    use super::{Vector3D, Facet, parse_facet};

    #[test]
    fn parse_facet1() {
        let mut input: Vec<u8> = vec![];

        input.write_f32::<LE>(0.0).unwrap();
        input.write_f32::<LE>(1.0).unwrap();
        input.write_f32::<LE>(0.0).unwrap();

        input.write_f32::<LE>(1.0).unwrap();
        input.write_f32::<LE>(2.0).unwrap();
        input.write_f32::<LE>(3.5).unwrap();

        input.write_f32::<LE>(4.5).unwrap();
        input.write_f32::<LE>(-5.0).unwrap();
        input.write_f32::<LE>(6.0).unwrap();

        input.write_f32::<LE>(-7.0).unwrap();
        input.write_f32::<LE>(8.5).unwrap();
        input.write_f32::<LE>(9.5).unwrap();

        input.write_u16::<LE>(12345).unwrap();

        let expected_output = IResult::Done(&b""[..], Facet{
            normal: Vector3D{x: 0.0, y: 1.0, z: 0.0},
            vertices: vec![
                Vector3D{x: 1.0, y: 2.0, z: 3.5},
                Vector3D{x: 4.5, y: -5.0, z: 6.0},
                Vector3D{x: -7.0, y: 8.5, z: 9.5},
            ],
            attribute: 12345,
        });
        let result = parse_facet(&input);

        assert_eq!(result, expected_output);
    }
}

#[cfg(test)]
mod test_solid {
    use nom::{IResult};
    use byteorder::{LE, WriteBytesExt};

    use super::{Vector3D, Facet, Solid, parse_solid};

    #[test]
    fn parse_solid1() {
        let mut input: Vec<u8> = vec![];

        input.extend("car".to_string().into_bytes());
        input.extend(vec![32;77]); // fill rest with spaces

        input.write_u32::<LE>(1).unwrap();

        input.write_f32::<LE>(0.0).unwrap();
        input.write_f32::<LE>(1.0).unwrap();
        input.write_f32::<LE>(0.0).unwrap();

        input.write_f32::<LE>(1.0).unwrap();
        input.write_f32::<LE>(2.0).unwrap();
        input.write_f32::<LE>(3.5).unwrap();

        input.write_f32::<LE>(4.5).unwrap();
        input.write_f32::<LE>(-5.0).unwrap();
        input.write_f32::<LE>(6.0).unwrap();

        input.write_f32::<LE>(-7.0).unwrap();
        input.write_f32::<LE>(8.5).unwrap();
        input.write_f32::<LE>(9.5).unwrap();

        input.write_u16::<LE>(12345).unwrap();

        let expected_output = IResult::Done(&b""[..], Solid{
            name: "car".to_string(),
            faces: vec![
                Facet{
                    normal: Vector3D{x: 0.0, y: 1.0, z: 0.0},
                    vertices: vec![
                        Vector3D{x: 1.0, y: 2.0, z: 3.5},
                        Vector3D{x: 4.5, y: -5.0, z: 6.0},
                        Vector3D{x: -7.0, y: 8.5, z: 9.5},
                    ],
                    attribute: 12345,
                },
            ],
        });
        let result = parse_solid(&input);

        assert_eq!(result, expected_output);
    }

    #[test]
    fn parse_solid2() {
        let mut input: Vec<u8> = vec![];

        input.extend("train".to_string().into_bytes());
        input.extend(vec![32;75]); // fill rest with spaces

        input.write_u32::<LE>(2).unwrap();

        // First face
        input.write_f32::<LE>(0.0).unwrap();
        input.write_f32::<LE>(1.0).unwrap();
        input.write_f32::<LE>(0.0).unwrap();

        input.write_f32::<LE>(1.0).unwrap();
        input.write_f32::<LE>(2.0).unwrap();
        input.write_f32::<LE>(3.5).unwrap();

        input.write_f32::<LE>(4.5).unwrap();
        input.write_f32::<LE>(-5.0).unwrap();
        input.write_f32::<LE>(6.0).unwrap();

        input.write_f32::<LE>(-7.0).unwrap();
        input.write_f32::<LE>(8.5).unwrap();
        input.write_f32::<LE>(9.5).unwrap();

        input.write_u16::<LE>(12345).unwrap();


        // Second face
        input.write_f32::<LE>(1.0).unwrap();
        input.write_f32::<LE>(1.0).unwrap();
        input.write_f32::<LE>(0.0).unwrap();

        input.write_f32::<LE>(9.0).unwrap();
        input.write_f32::<LE>(6.0).unwrap();
        input.write_f32::<LE>(3.5).unwrap();

        input.write_f32::<LE>(8.5).unwrap();
        input.write_f32::<LE>(-5.0).unwrap();
        input.write_f32::<LE>(2.0).unwrap();

        input.write_f32::<LE>(7.0).unwrap();
        input.write_f32::<LE>(4.0).unwrap();
        input.write_f32::<LE>(-1.0).unwrap();

        input.write_u16::<LE>(55555).unwrap();

        let expected_output = IResult::Done(&b""[..], Solid{
            name: "train".to_string(),
            faces: vec![
                Facet{
                    normal: Vector3D{x: 0.0, y: 1.0, z: 0.0},
                    vertices: vec![
                        Vector3D{x: 1.0, y: 2.0, z: 3.5},
                        Vector3D{x: 4.5, y: -5.0, z: 6.0},
                        Vector3D{x: -7.0, y: 8.5, z: 9.5},
                    ],
                    attribute: 12345,
                },
                Facet{
                    normal: Vector3D{x: 1.0, y: 1.0, z: 0.0},
                    vertices: vec![
                        Vector3D{x: 9.0, y: 6.0, z: 3.5},
                        Vector3D{x: 8.5, y: -5.0, z: 2.0},
                        Vector3D{x: 7.0, y: 4.0, z: -1.0},
                    ],
                    attribute: 55555,
                },
            ],
        });
        let result = parse_solid(&input);

        assert_eq!(result, expected_output);
    }
}
