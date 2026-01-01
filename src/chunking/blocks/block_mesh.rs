use cgmath::Vector3;



#[rustfmt::skip]
pub const BLOCK_FACE_DATA: [[Vector3<f32>; 4]; 6] = [
    [ // Front  (x+)
        Vector3::new( 0.5,  0.5,  0.5), // topleft      (x+1,y+1,z+1)
        Vector3::new( 0.5, -0.5,  0.5), // bottomleft   (x+1,y+0,z+1)
        Vector3::new( 0.5, -0.5, -0.5), // bottomright  (x+1,y+0,z+0)
        Vector3::new( 0.5,  0.5, -0.5), // topright     (x+1,y+1,z+0)
    ],
    [ // Back   (x-)
        Vector3::new(-0.5, -0.5, -0.5), // bottomright  (x+0,y+0,z+0)
        Vector3::new(-0.5, -0.5,  0.5), // bottomleft   (x+0,y+0,z+1)
        Vector3::new(-0.5,  0.5,  0.5), // topleft      (x+0,y+1,z+1)
        Vector3::new(-0.5,  0.5, -0.5), // topright     (x+0,y+1,z+0)
    ],
    [ // Left   (z+)
        Vector3::new( 0.5,  0.5,  0.5), // topleft      (x+1,y+1,z+1)
        Vector3::new(-0.5,  0.5,  0.5), // topright     (x+0,y+1,z+1)
        Vector3::new(-0.5, -0.5,  0.5), // bottomright  (x+0,y+0,z+1)
        Vector3::new( 0.5, -0.5,  0.5), // bottomleft   (x+1,y+0,z+1)
    ],
    [ // Right  (z-)
        Vector3::new( 0.5,  0.5, -0.5), // topleft      (x+1,y+1,z+0)
        Vector3::new( 0.5, -0.5, -0.5), // bottomleft   (x+1,y+0,z+0)
        Vector3::new(-0.5, -0.5, -0.5), // bottomright  (x+0,y+0,z+0)
        Vector3::new(-0.5,  0.5, -0.5), // topright     (x+0,y+1,z+0)
    ],
    [ // Top    (y+)
        Vector3::new( 0.5,  0.5,  0.5), // topleft      (x+1,y+1,z+1)
        Vector3::new( 0.5,  0.5, -0.5), // topright     (x+1,y+1,z+0)
        Vector3::new(-0.5,  0.5, -0.5), // bottomright  (x+0,y+1,z+0)
        Vector3::new(-0.5,  0.5,  0.5), // bottomleft   (x+0,y+1,z+1)
    ],
    [ // Bottom (y-)
        Vector3::new(-0.5, -0.5, -0.5), // bottomright  (x+0,y+0,z+0)
        Vector3::new( 0.5, -0.5, -0.5), // topright     (x+1,y+0,z+0)
        Vector3::new( 0.5, -0.5,  0.5), // topleft      (x+1,y+0,z+1)
        Vector3::new(-0.5, -0.5,  0.5), // bottomleft   (x+0,y+0,z+1)
    ],
];
