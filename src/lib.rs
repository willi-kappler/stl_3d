#[macro_use]
extern crate nom;
extern crate byteorder;

mod parse_text;
mod parse_binary;

/*
    STL format:

    http://www.fabbers.com/tech/STL_Format

    https://en.wikipedia.org/wiki/STL_(file_format)

    https://all3dp.com/what-is-stl-file-format-extension-3d-printing/

    http://cedifa.de/wp-content/uploads/2014/05/07_3D-Modell-Formate.pdf


    ASCII:

    solid name

        facet normal nx ny nz
            outer loop
                vertex x, y, z
                vertex x, y, z
                vertex x, y, z
            endloop
        endfacet

        facet normal nx ny nz
            outer loop
                vertex x, y, z
                vertex x, y, z
                vertex x, y, z
            endloop
        endfacet

        ...

    endsolid name


    Binary:

    UINT8[80] – Header
    UINT32 – Number of triangles

    foreach triangle
        REAL32[3] – Normal vector
        REAL32[3] – Vertex 1
        REAL32[3] – Vertex 2
        REAL32[3] – Vertex 3
        UINT16 – Attribute byte count
    end


*/


#[derive(PartialEq, Debug)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(PartialEq, Debug)]
pub struct Facet {
    pub normal: Vector3D,
    pub vertices: Vec<Vector3D>,
    pub attribute: u16
}

#[derive(PartialEq, Debug)]
pub struct Solid {
    pub name: String,
    pub faces: Vec<Facet>,
}
