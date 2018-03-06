use nom::{double_s, alphanumeric};

use super::{Vector3D, Facet, Solid};

named!(parse_vector<&str, Vector3D>, do_parse!(
    ws!(tag!("vertex")) >>
    vx: ws!(double_s) >> tag!(",") >>
    vy: ws!(double_s) >> tag!(",") >>
    vz: ws!(double_s) >>
    (Vector3D{x: vx, y: vy, z: vz})
));

named!(parse_many_vertices<&str, Vec<Vector3D>>, do_parse!(
        v1: parse_vector >>
        v2: parse_vector >>
        v3: parse_vector >>
        ({ vec![v1, v2, v3] })
));

named!(parse_facet<&str, Facet>, do_parse!(
    ws!(tag!("facet")) >> ws!(tag!("normal")) >>
    nx: ws!(double_s) >>
    ny: ws!(double_s) >>
    nz: ws!(double_s) >>
    ws!(tag!("outer")) >> ws!(tag!("loop")) >>
    vertices: parse_many_vertices >>
    ws!(tag!("endloop")) >> ws!(tag!("endfacet")) >>
    (
        Facet{
            normal: Vector3D{x: nx, y: ny, z: nz},
            vertices: vertices,
            attribute: 0,
        }
    )
));

named!(parse_solid<&str, Solid>, do_parse!(
    ws!(tag!("solid")) >>
    name: ws!(alphanumeric) >>
    faces: many1!(parse_facet) >>
    ws!(tag!("endsolid")) >>
    name2: ws!(alphanumeric) >>
    (
        Solid{
            name: name.to_string(),
            faces: faces,
        }
    )
));

#[cfg(test)]
mod test_vertex {
    use nom::{IResult, Needed, ErrorKind};

    use super::{Vector3D, parse_vector};

    #[test]
    fn parse_vector1() {
        let input = "vertex 1.2, 6.78, -109.1";
        let expected_output = IResult::Done("", Vector3D{x: 1.2, y: 6.78, z: -109.1});
        let result = parse_vector(input);

        assert_eq!(result, expected_output);
    }

    #[test]
    fn parse_vector2() {
        let input = "vertex 1.2, 6.78, -109.1, ";
        let expected_output = IResult::Done(", ", Vector3D{x: 1.2, y: 6.78, z: -109.1});
        let result = parse_vector(input);

        assert_eq!(result, expected_output);
    }

    #[test]
    fn parse_vector3() {
        let input = "vertex 1.2, 6.78, ";
        let expected_output = IResult::Incomplete(Needed::Size(19));
        let result = parse_vector(input);

        assert_eq!(result, expected_output);
    }

    #[test]
    fn parse_vector4() {
        let input = "vertex 1.2, ";
        let expected_output = IResult::Incomplete(Needed::Size(13));
        let result = parse_vector(input);

        assert_eq!(result, expected_output);
    }

    #[test]
    fn parse_vector5() {
        let input = "vertex";
        let expected_output = IResult::Incomplete(Needed::Size(7));
        let result = parse_vector(input);

        assert_eq!(result, expected_output);
    }

    #[test]
    fn parse_vector6() {
        let input = "";
        let expected_output = IResult::Incomplete(Needed::Size(6));
        let result = parse_vector(input);

        assert_eq!(result, expected_output);
    }

    #[test]
    fn parse_vector7() {
        let input = "vector";
        let expected_output = IResult::Error(ErrorKind::Tag);
        let result = parse_vector(input);

        assert_eq!(result, expected_output);
    }

    #[test]
    fn parse_vector8() {
        let input = "vertex -1.0, -2.0, -3.0";
        let expected_output = IResult::Done("", Vector3D{x: -1.0, y: -2.0, z: -3.0});
        let result = parse_vector(input);

        assert_eq!(result, expected_output);
    }

    #[test]
    fn parse_vector9() {
        let input = "vertex 4.0, 5.0, 6.0";
        let expected_output = IResult::Done("", Vector3D{x: 4.0, y: 5.0, z: 6.0});
        let result = parse_vector(input);

        assert_eq!(result, expected_output);
    }
}

#[cfg(test)]
mod test_many_vertices {
    use nom::{IResult};

    use super::{Vector3D, parse_many_vertices};

    #[test]
    fn parse_vertices1() {
        let input = "vertex 1.0, 2.0, 3.0 vertex 4.0, 5.0, 6.0 vertex 7.0, 8.0, 9.0";
        let expected_output = IResult::Done("",
            vec![
                Vector3D{x: 1.0, y: 2.0, z: 3.0},
                Vector3D{x: 4.0, y: 5.0, z: 6.0},
                Vector3D{x: 7.0, y: 8.0, z: 9.0},
            ]);
        let result = parse_many_vertices(input);

        assert_eq!(result, expected_output);
    }

    fn parse_vertices2() {
        let input = "vertex 1.0, 2.0, 3.0\nvertex 4.0, 5.0, 6.0\nvertex 7.0, 8.0, 9.0";
        let expected_output = IResult::Done("",
            vec![
                Vector3D{x: 1.0, y: 2.0, z: 3.0},
                Vector3D{x: 4.0, y: 5.0, z: 6.0},
                Vector3D{x: 7.0, y: 8.0, z: 9.0},
            ]);
        let result = parse_many_vertices(input);

        assert_eq!(result, expected_output);
    }
}

#[cfg(test)]
mod test_facet {
    use nom::{IResult, ErrorKind};

    use super::{Vector3D, Facet, parse_facet};

    #[test]
    fn parse_facet1() {
        let input = r"
            facet normal 0.0 1.0 0.0
                outer loop
                    vertex 1.0, 2.0, 3.0
                    vertex 4.0, 5.0, 6.0
                    vertex 7.0, 8.0, 9.0
                endloop
            endfacet
        ";
        let expected_output = IResult::Done("",
            Facet{
                normal: Vector3D{x: 0.0, y: 1.0, z: 0.0},
                vertices: vec![
                    Vector3D{x: 1.0, y: 2.0, z: 3.0},
                    Vector3D{x: 4.0, y: 5.0, z: 6.0},
                    Vector3D{x: 7.0, y: 8.0, z: 9.0},
                ],
                attribute: 0,
            }
        );
        let result = parse_facet(input);

        assert_eq!(result, expected_output);
    }

    #[test]
    fn parse_facet2() {
        let input = r"
            facet 0.0 1.0 0.0
                outer loop
                    vertex 1.0, 2.0, 3.0
                    vertex 4.0, 5.0, 6.0
                    vertex 7.0, 8.0, 9.0
                endloop
            endfacet
        ";
        let expected_output = IResult::Error(ErrorKind::Tag);
        let result = parse_facet(input);

        assert_eq!(result, expected_output);
    }

    #[test]
    fn parse_facet3() {
        let input = r"
            facet normal 1.0 0.0
                outer loop
                    vertex 1.0, 2.0, 3.0
                    vertex 4.0, 5.0, 6.0
                    vertex 7.0, 8.0, 9.0
                endloop
            endfacet
        ";
        let expected_output = IResult::Error(ErrorKind::Alt);
        let result = parse_facet(input);

        assert_eq!(result, expected_output);
    }
}

#[cfg(test)]
mod test_solid {
    use nom::{IResult, Needed, Err, ErrorKind};

    use super::{Vector3D, Facet, Solid, parse_solid};

    #[test]
    fn parse_solid1() {
        let input = r"
            solid plane
                facet normal 0.0 0.0 1.0
                    outer loop
                        vertex 3.0, 2.0, 1.0
                        vertex 6.0, 5.0, 4.0
                        vertex 9.0, 8.0, 7.0
                    endloop
                endfacet
            endsolid plane
        ";
        let expected_output = IResult::Done("",
            Solid{
                name: "plane".to_string(),
                faces: vec![
                    Facet{
                        normal: Vector3D{x: 0.0, y: 0.0, z: 1.0},
                        vertices: vec![
                            Vector3D{x: 3.0, y: 2.0, z: 1.0},
                            Vector3D{x: 6.0, y: 5.0, z: 4.0},
                            Vector3D{x: 9.0, y: 8.0, z: 7.0},
                        ],
                        attribute: 0
                    }
                ]
            }
        );
        let result = parse_solid(input);
        assert_eq!(result, expected_output);
    }

    #[test]
    fn parse_solid2() {
        let input = r"
            solid plane2
                facet normal 0.0 0.0 1.0
                    outer loop
                        vertex 3.0, 2.0, 1.0
                        vertex 6.0, 5.0, 4.0
                        vertex 9.0, 8.0, 7.0
                    endloop
                endfacet

                facet normal 0.0 1.0 1.0
                    outer loop
                        vertex 3.5, -2.0, 0.0
                        vertex 2.2, 8.0, -9.0
                        vertex -3.0, 8.0, 1.5
                    endloop
                endfacet
            endsolid plane2
        ";
        let expected_output = IResult::Done("",
            Solid{
                name: "plane2".to_string(),
                faces: vec![
                    Facet{
                        normal: Vector3D{x: 0.0, y: 0.0, z: 1.0},
                        vertices: vec![
                            Vector3D{x: 3.0, y: 2.0, z: 1.0},
                            Vector3D{x: 6.0, y: 5.0, z: 4.0},
                            Vector3D{x: 9.0, y: 8.0, z: 7.0},
                        ],
                        attribute: 0
                    },
                    Facet{
                        normal: Vector3D{x: 0.0, y: 1.0, z: 1.0},
                        vertices: vec![
                            Vector3D{x: 3.5, y: -2.0, z: 0.0},
                            Vector3D{x: 2.2, y: 8.0, z: -9.0},
                            Vector3D{x: -3.0, y: 8.0, z: 1.5},
                        ],
                        attribute: 0
                    },
                ]
            }
        );
        let result = parse_solid(input);
        assert_eq!(result, expected_output);
    }
}
