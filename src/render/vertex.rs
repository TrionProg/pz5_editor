#[derive(Copy,Clone)]
struct VertexP2{
    position:[f32;2],
}

implement_vertex!(VertexP2, position);

#[derive(Copy,Clone)]
struct VertexP2N2{
    position:[f32;2],
    normal:[f32;2],
}

implement_vertex!(VertexP2N2, position, normal);

#[derive(Copy,Clone)]
struct VertexP2N2TC{
    position:[f32;2],
    normal:[f32;2],
    tex_coords:[f32;2],
}

implement_vertex!(VertexP2N2TC, position, normal, tex_coords);

#[derive(Copy,Clone)]
struct VertexP3{
    position:[f32;3],
}

implement_vertex!(VertexP3, position);

#[derive(Copy,Clone)]
struct VertexP3N3{
    position:[f32;3],
    normal:[f32;3],
}

implement_vertex!(VertexP3N3, position, normal);

#[derive(Copy,Clone)]
struct VertexP3N3TC{
    position:[f32;3],
    normal:[f32;3],
    tex_coords:[f32;2],
}

implement_vertex!(VertexP3N3TC, position, normal, tex_coords);
