use crate::rendering::scene::Scene;

pub trait Renderer{
    fn renderMesh(mesh: Mesh, Vec<Deltas>);
}