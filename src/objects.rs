use model3d_gl::Gl;

pub fn new<G: Gl>(render_context: &mut G) -> model3d_base::Instantiable<G> {
    let mut vertices = model3d_base::ExampleVertices::new();
    let material = model3d_base::BaseMaterial::rgba((1., 0., 0., 1.));

    let mut obj: model3d_base::Object<G> = model3d_base::Object::new();

    // Using the set of indices/vertex data defined create primitives (a triangle)
    let m_id = obj.add_material(&material);

    // Add vertices to the set
    model3d_base::example_objects::triangle::new::<G>(&mut vertices, 0.5);
    model3d_base::example_objects::tetrahedron::new::<G>(&mut vertices, 0.5);

    // Create a triangle object with an empty skeleton
    let v_id = obj.add_vertices(vertices.borrow_vertices(0));
    let mesh = model3d_base::example_objects::triangle::mesh(v_id, m_id);
    obj.add_component(None, None, mesh);

    // Create a tetrahedron object with an empty skeleton
    let v_id = obj.add_vertices(vertices.borrow_vertices(1));
    let mesh = model3d_base::example_objects::tetrahedron::mesh(v_id, m_id);
    let transformation = model3d_base::Transformation::new().set_translation([0.5, 0., 0.]);
    obj.add_component(None, Some(transformation), mesh);

    obj.analyze();
    obj.create_client(render_context);
    obj.into_instantiable()
}
