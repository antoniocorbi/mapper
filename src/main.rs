use std::fs::File;
use std::io::BufReader;
use topojson::{Arc, ArcIndexes, Geometry, Position, Topology, TransformParams, Value};

fn decode_arcs(arc: &Vec<Position>, transform: &Option<TransformParams>) -> Vec<(f64, f64)> {
    let mut x = 0.0;
    let mut y = 0.0;

    arc.iter()
        .map(|point| {
            // 1. Delta decoding (sumar al valor anterior)
            x += point[0];
            y += point[1];

            // 2. Aplicar transformación si existe
            if let Some(t) = transform {
                (
                    x * t.scale[0] + t.translate[0],
                    y * t.scale[1] + t.translate[1],
                )
            } else {
                (x, y)
            }
        })
        .collect()
}

fn decode_point(point: (f64, f64), transform: &Option<TransformParams>) -> (f64, f64) {
    let mut x = 0.0;
    let mut y = 0.0;

    x = point.0;
    y = point.1;

    // 2. Aplicar transformación si existe
    if let Some(t) = transform {
        (
            x * t.scale[0] + t.translate[0],
            y * t.scale[1] + t.translate[1],
        )
    } else {
        (x, y)
    }
}

fn process_polygon(p: &Vec<ArcIndexes>, topology: &Topology) {
    let arcs = &topology.arcs;
    let transform = &topology.transform;

    // println!("Processing polygon.");

    for ring in p {
        for &arc_index in ring {
            // 1. Determinar el índice real y si está invertido
            let (real_index, is_reversed) = if arc_index >= 0 {
                (arc_index as usize, false)
            } else {
                ((!arc_index) as usize, true) // !n es el bitwise NOT para i32
            };
            // 2. Obtener el poligono de la lista maestra
            let polygon: &Arc = &topology.arcs[real_index];
            //if let Some(arc_points) = topology.arcs.as_ref().and_then(|a| a.get(real_index)) {}
            // if is_reversed {
            //     let points_iter;
            //     points_iter = arcs.iter().rev()
            // } else {
            //     let points_iter;
            //     points_iter = arcs.iter();
            // };
            let polygon_points_iter: Box<dyn Iterator<Item = &Vec<f64>>> = if is_reversed {
                Box::new(polygon.iter().rev())
            } else {
                Box::new(polygon.iter())
            };

            for point in polygon_points_iter {
                // Aquí tienes tus f64: [x, y]
                let x = point[0];
                let y = point[1];
                let pdecoded = decode_point((x, y), transform);
                dbg!(pdecoded);
                // Nota: Si el TopoJSON tiene 'transform', estos puntos
                // siguen siendo enteros cuantizados. Deberás aplicar
                // la fórmula (n * scale) + translate.
            }
        }
    }
}

//fn process_geometry(g: &Geometry, transform: &Option<TransformParams>) {
fn process_geometry(g: &Geometry, topology: &Topology) {
    // if let Some(props) = &g.properties {
    //     if let Some(val) = props.get("name") {
    //         let country = match val {
    //             serde_json::value::Value::String(c) => c,
    //             _ => &String::new(),
    //         };
    //         println!("El país es: {}", country);
    //     } else {
    //         println!("La clave 'pais' no existe.");
    //     }
    // }

    if let Some(country) = &g.properties {
        print!("Country: {} -> ", country["name"]);
    }

    // let country = &g.properties.as_ref().unwrap()["name"];
    // print!("Country: {} -> ", g.properties.as_ref().unwrap()["name"]);
    match &g.value {
        Value::Point(point) => {
            println!("Found Point");
        }
        Value::MultiPoint(mp) => {
            println!("Found MultiPoint");
        }
        Value::Polygon(p) => {
            println!("Found Polygon");
            process_polygon(p, topology);
            // for ring in rings {
            //   render_ring(&ring, &arcs, &transform);
            // }
        }
        Value::MultiPolygon(mp) => {
            println!("Found MultiPolygon");
            // for polygon in polygons {
            //     for ring in polygon {
            //       render_ring(&ring, &arcs, &transform);
            //     }
            // }
        }
        Value::LineString(arcs) => {
            println!("Found LineStrings");
            // render_ring(&ring, &arcs, &transform);
        }
        Value::GeometryCollection(gc) => {
            println!("Found GeometryCollection with #{} elems.", gc.len());
            for internalg in gc {
                process_geometry(internalg, topology);
            }
        }
        _ => {
            // Otros tipos como Point o MultiPoint no usan la propiedad 'arcs'
            // dbg!(&geometry.value);
            println!("Tipo de geometría no soportado para dibujo por arcos.");
        }
    }
}

fn process_topology(topology: &Topology) {
    let transform = &topology.transform;
    let arcs = &topology.arcs;

    // dbg!(&topology.bbox);

    // 1. Acceder a los objetos
    for ng in &topology.objects {
        println!("Named Geometry: {}", ng.name);
        let geometry = &ng.geometry;

        // dbg!(geometry);
        process_geometry(geometry, topology);
    }
}

/// Función auxiliar para procesar una lista de índices de arcos (un "ring" o línea)
// fn render_ring(
//     ring_indices: &Vec<i64>,
//     all_arcs: &Vec<Vec<Vec<f64>>>,
//     transform: &Option<topojson::TransformParams>,
// ) {
//     for &arc_index in ring_indices {
//         // Manejo de índices negativos (Bitwise NOT en Rust para TopoJSON)
//         let (idx, reverse) = if arc_index < 0 {
//             ((!arc_index) as usize, true)
//         } else {
//             (arc_index as usize, false)
//         };
//
//         if let Some(raw_points) = all_arcs.get(idx) {
//             let mut coords = decode_arc(raw_points, transform);
//
//             if reverse {
//                 coords.reverse();
//             }
//
//             // Aquí enviarías 'coords' a tu motor gráfico
//             for (x, y) in coords {
//                 // draw_point(x, y);
//             }
//         }
//     }
// }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 0. PWD
    // let cwd = std::env::current_dir()?;
    // println!("CWD: {}", cwd.display());

    // 1. Cargar el archivo
    let file = File::open("/home/acorbi/projects/mapper/assets/world-lowres.topo.json")?;
    let reader = BufReader::new(file);

    // 2. Deserializar a la estructura Topology
    let topology: Topology = serde_json::from_reader(reader)?;

    // println!(
    //     "Transform: {:#?} ",
    //     topology
    //         .transform
    //         .as_ref()
    //         .ok_or("No transform available.")?
    // );

    process_topology(&topology);

    // 3. Acceder a los arcos globales
    // Los arcos son una lista de listas de posiciones: Vec<Vec<Vec<f64>>>
    // let arcs = &topology.arcs;
    //
    // println!("El archivo tiene {} arcos.", arcs.len());

    // 4. Ejemplo: Recorrer el primer arco para obtener puntos
    // for (i, arc) in arcs.iter().enumerate() {
    //     //if let Some(first_arc) = arcs.get(0) {
    //     println!("Arco: {i}\n-------");
    //     for position in arc {
    //         // position es un Vec<f64>, usualmente [x, y]
    //         let x = position[0];
    //         let y = position[1];
    //         println!("Punto del arco: {}, {}", x, y);
    //     }
    //     println!();
    // }

    Ok(())
}
