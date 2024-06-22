use model3d_gl::Gl;
const JSON: &str = r##"
{
    "asset" : {
        "generator" : "Khronos glTF Blender I/O v1.4.40",
        "version" : "2.0"
    },
    "scene" : 0,
    "scenes" : [
        {
            "name" : "Scene",
            "nodes" : [
                0,
                1,
                2
            ]
        }
    ],
    "nodes" : [
        {
            "mesh" : 0,
            "name" : "Cube"
        },
        {
            "name" : "Light",
            "rotation" : [
                0.16907575726509094,
                0.7558803558349609,
                -0.27217137813568115,
                0.570947527885437
            ],
            "translation" : [
                4.076245307922363,
                5.903861999511719,
                -1.0054539442062378
            ]
        },
        {
            "name" : "Camera",
            "rotation" : [
                0.483536034822464,
                0.33687159419059753,
                -0.20870360732078552,
                0.7804827094078064
            ],
            "translation" : [
                7.358891487121582,
                4.958309173583984,
                6.925790786743164
            ]
        }
    ],
    "materials" : [
        {
            "doubleSided" : true,
            "name" : "Material",
            "pbrMetallicRoughness" : {
                "baseColorFactor" : [
                    0.800000011920929,
                    0.800000011920929,
                    0.800000011920929,
                    1
                ],
                "metallicFactor" : 0,
                "roughnessFactor" : 0.4000000059604645
            }
        }
    ],
    "meshes" : [
        {
            "name" : "Cube",
            "primitives" : [
                {
                    "attributes" : {
                        "POSITION" : 0,
                        "NORMAL" : 1,
                        "TEXCOORD_0" : 2
                    },
                    "indices" : 3,
                    "material" : 0
                }
            ]
        }
    ],
    "accessors" : [
        {
            "bufferView" : 0,
            "componentType" : 5126,
            "count" : 24,
            "max" : [
                1,
                1,
                1
            ],
            "min" : [
                -1,
                -1,
                -1
            ],
            "type" : "VEC3"
        },
        {
            "bufferView" : 1,
            "componentType" : 5126,
            "count" : 24,
            "type" : "VEC3"
        },
        {
            "bufferView" : 2,
            "componentType" : 5126,
            "count" : 24,
            "type" : "VEC2"
        },
        {
            "bufferView" : 3,
            "componentType" : 5123,
            "count" : 36,
            "type" : "SCALAR"
        }
    ],
    "bufferViews" : [
        {
            "buffer" : 0,
            "byteLength" : 288
        },
        {
            "buffer" : 0,
            "byteLength" : 288,
            "byteOffset" : 288
        },
        {
            "buffer" : 0,
            "byteLength" : 192,
            "byteOffset" : 576
        },
        {
            "buffer" : 0,
            "byteLength" : 72,
            "byteOffset" : 768
        }
    ],
    "buffers" : [
        {
            "byteLength" : 840,
            "uri" : "data:application/octet-stream;base64,AACAPwAAgD8AAIC/AACAPwAAgD8AAIC/AACAPwAAgD8AAIC/AACAPwAAgL8AAIC/AACAPwAAgL8AAIC/AACAPwAAgL8AAIC/AACAPwAAgD8AAIA/AACAPwAAgD8AAIA/AACAPwAAgD8AAIA/AACAPwAAgL8AAIA/AACAPwAAgL8AAIA/AACAPwAAgL8AAIA/AACAvwAAgD8AAIC/AACAvwAAgD8AAIC/AACAvwAAgD8AAIC/AACAvwAAgL8AAIC/AACAvwAAgL8AAIC/AACAvwAAgL8AAIC/AACAvwAAgD8AAIA/AACAvwAAgD8AAIA/AACAvwAAgD8AAIA/AACAvwAAgL8AAIA/AACAvwAAgL8AAIA/AACAvwAAgL8AAIA/AAAAAAAAAAAAAIC/AAAAAAAAgD8AAAAAAACAPwAAAAAAAAAAAAAAAAAAgL8AAACAAAAAAAAAAAAAAIC/AACAPwAAAAAAAAAAAAAAAAAAAAAAAIA/AAAAAAAAgD8AAAAAAACAPwAAAAAAAAAAAAAAAAAAgL8AAACAAAAAAAAAAAAAAIA/AACAPwAAAAAAAAAAAACAvwAAAAAAAAAAAAAAAAAAAAAAAIC/AAAAAAAAgD8AAAAAAACAvwAAAAAAAAAAAAAAAAAAgL8AAACAAAAAAAAAAAAAAIC/AACAvwAAAAAAAAAAAAAAAAAAAAAAAIA/AAAAAAAAgD8AAAAAAACAvwAAAAAAAAAAAAAAAAAAgL8AAACAAAAAAAAAAAAAAIA/AAAgPwAAAD8AACA/AAAAPwAAID8AAAA/AADAPgAAAD8AAMA+AAAAPwAAwD4AAAA/AAAgPwAAgD4AACA/AACAPgAAID8AAIA+AADAPgAAgD4AAMA+AACAPgAAwD4AAIA+AAAgPwAAQD8AACA/AABAPwAAYD8AAAA/AADAPgAAQD8AAAA+AAAAPwAAwD4AAEA/AAAgPwAAgD8AACA/AAAAAAAAYD8AAIA+AADAPgAAgD8AAAA+AACAPgAAwD4AAAAAAQAOABQAAQAUAAcACgAGABMACgATABcAFQASAAwAFQAMAA8AEAADAAkAEAAJABYABQACAAgABQAIAAsAEQANAAAAEQAAAAQA"
        }
    ]
}

"##;

pub fn new_of_glb<G: Gl>(
    render_context: &mut G,
    glb: &[u8],
    node_names: &[&str],
) -> Result<model3d_base::Instantiable<G>, String> {
    fn buf_reader(file: &mut &[u8], byte_length: usize) -> Result<Option<Vec<u8>>, std::io::Error> {
        use std::io::Read;
        let mut buffer = vec![0; byte_length];
        crate::console_log!("file {} {}", file.len(), byte_length);
        file.read_exact(&mut buffer).unwrap();
        Ok(Some(buffer))
    }

    let mut file = glb;
    crate::console_log!("Before call file {} ", file.len());
    let (mut gltf, opt_buffer_0) = model3d_gltf::glb_load(&mut file, &buf_reader, 16 * 1000 * 1000)
        .map_err(|e| format!("{e:?}"))?;

    let mut od = model3d_gltf::ObjectData::new(&gltf);
    for n in node_names {
        od.add_object(&gltf, gltf.get_node(n).unwrap());
    }
    od.derive_uses(&gltf);
    let buffers = od
        .gen_byte_buffers(&mut gltf, &model3d_gltf::buf_parse_fail, opt_buffer_0)
        .map_err(|e| format!("{e:?}"))?;
    let buffer_data = od.gen_buffer_data::<_, _, G>(&|x| &buffers[x]);
    let buffer_accessors = od.gen_accessors(&gltf, &|x| &buffer_data[x]);
    let vertices = od.gen_vertices(&gltf, &|x| &buffer_accessors[x]);
    let mut obj = od.gen_object(&gltf, &vertices);

    let material = model3d_base::BaseMaterial::rgba((1., 0., 0., 1.));
    let _ = obj.add_material(&material);

    obj.analyze();
    obj.into_instantiable(render_context).map_err(|(_, e)| e)
}

pub fn new<G: Gl>(render_context: &mut G) -> Result<model3d_base::Instantiable<G>, String> {
    let jv = serde_json::from_str::<serde_json::Value>(JSON).map_err(|e| format!("{e:?}"))?;
    let mut gltf = model3d_gltf::Gltf::of_json_value(jv).map_err(|e| format!("{e:?}"))?;
    let mut od = model3d_gltf::ObjectData::new(&gltf);
    od.add_object(&gltf, gltf.get_node("Cube").unwrap());
    od.derive_uses(&gltf);
    let buffers = od
        .gen_byte_buffers(&mut gltf, &model3d_gltf::buf_parse_fail, None)
        .map_err(|e| format!("{e:?}"))?;
    let buffer_data = od.gen_buffer_data::<_, _, G>(&|x| &buffers[x]);
    let buffer_accessors = od.gen_accessors(&gltf, &|x| &buffer_data[x]);
    let vertices = od.gen_vertices(&gltf, &|x| &buffer_accessors[x]);
    let mut obj = od.gen_object(&gltf, &vertices);

    let material = model3d_base::BaseMaterial::rgba((1., 0., 0., 1.));
    let _ = obj.add_material(&material);

    obj.analyze();
    obj.into_instantiable(render_context).map_err(|(_, e)| e)
}
