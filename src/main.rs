extern crate kiss3d;
extern crate nalgebra as na;

// use na::{Vector3, UnitQuaternion, Translation};
use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::nalgebra::{Translation3, UnitQuaternion, Vector3};

struct Cube {
    scale: f32,
    x: f32,
    y: f32,
    z: f32,
}

fn main() {
    let mut sponge: Vec<Cube> = Vec::new();
    let cube = Cube {
        scale: 1.0,
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    sponge.push(cube);
    sponge = generate(sponge, 1, 3);

    let mut window = Window::new("Menger Sponge");

    let mut g = window.add_group();

    for cube in sponge {
        let mut c = g.add_cube(0.0, 0.0, 0.0);
        c.append_translation(&Translation3::new(cube.x, cube.y, cube.z));
        c.set_local_scale(cube.scale, cube.scale, cube.scale);
        c.set_color(0.5, 0.5, 0.5);
    }

    window.set_light(Light::StickToCamera);

    let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);

    while window.render() {
        g.prepend_to_local_rotation(&rot)
    }
}

fn generate(mut sponge: Vec<Cube>, iter: i32, max_iter : i32) -> Vec<Cube> {
    if iter > max_iter {
        return sponge;
    }
    let mut new_sponge: Vec<Cube> = Vec::new();

    for cube in sponge {
        for x in -1_i32..2 {
            for y in -1_i32..2 {
                for z in -1_i32..2 {
                    let sum = x.abs() + y.abs() + z.abs();
                    if sum > 1 {
                        let new_scale = cube.scale / 3.0;
                        let small_cube = Cube {
                            scale: cube.scale / 3.0,
                            x: cube.x + x as f32 * new_scale,
                            y: cube.y + y as f32 * new_scale,
                            z: cube.z + z as f32 * new_scale,
                        };
                        new_sponge.push(small_cube);
                    }
                }
            }
        }
    }
    sponge = new_sponge;
    return generate(sponge, iter + 1, max_iter);
}