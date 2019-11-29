use std::io::prelude::*;
use std::fs::File;

struct Vector3D {
  x: f32,
  y: f32,
  z: f32,
}

struct Light {
  direction: Vector3D,
  intensity: f32,
}

struct Sphere {
  position: Vector3D,
  color: Vector3D,
  radius: f32,
}

struct Ray {
  origin: Vector3D,
  direction: Vector3D,
}

impl Vector3D {
  fn mul(&self, scalar: &f32) -> Vector3D {
    Vector3D { x: self.x * scalar, y: self.y * scalar, z: self.z * scalar }
  }

  fn dot(&self, other: &Vector3D) -> f32 {
    self.x * other.x + self.y * other.y + self.z * other.z
  }

  fn add(&self, other: &Vector3D) -> Vector3D {
    Vector3D { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
  }

  fn sub(&self, other: &Vector3D) -> Vector3D {
    Vector3D { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
  }

  fn magnitude(&self) -> f32 {
    self.dot(self).sqrt()
  }

  fn normalize(&self) -> Vector3D {
    let s = 1.0 / self.magnitude();
    return Vector3D { x: self.x * s, y: self.y * s, z: self.z * s};
  }

  fn rgb(&self) -> [u8; 3] {
    return [
      (self.x * 255.0) as u8,
      (self.y * 255.0) as u8,
      (self.z * 255.0) as u8
    ];
  }
}

impl Sphere {
  fn intersects(&self, ray: &Ray) -> Option<f32> {
    let oc = self.position.sub(&ray.origin);
    let tca = oc.dot(&ray.direction);
    if tca < 0.0 { return None; }
    let l2oc = oc.dot(&oc);
    let sr2 = self.radius * self.radius;
    let d2 = l2oc - (tca * tca);

    if d2 > sr2 {
      return None;
    }

    let thc = (sr2 - d2).sqrt();
    let t0 = tca - thc;
    let t1 = tca + thc;

    if t0 < 0.0 && t1 < 0.0 {
        None
    } else if t0 < 0.0 {
        Some(t1)
    } else if t1 < 0.0 {
        Some(t0)
    } else {
        Some(if t0 < t1 { t0 } else { t1 })
    }
  }

  fn surface_normal(&self, hit_point: &Vector3D) -> Vector3D {
    return hit_point.sub(&self.position).normalize();
  }
}

impl Light {
}

fn main() -> std::io::Result<()> {
  const WIDTH: usize = 800;
  const HEIGHT: usize = 600;
  const F_WIDTH: f32 = WIDTH as f32;
  const F_HEIGHT: f32 = HEIGHT as f32;
  const ASPECT: f32 = F_WIDTH / F_HEIGHT;

  let mut pixels: Vec<u8> = Vec::with_capacity(WIDTH * HEIGHT * 3);

  let spheres = [
    Sphere {
      position: Vector3D { x: 0.0, y: 0.0, z: -5.0 },
      color: Vector3D { x: 1.0, y: 0.0, z: 0.0 },
      radius:   1.0
    },
    Sphere {
      position: Vector3D { x: 1.0, y: 0.1, z: -3.0 },
      color: Vector3D { x: 0.0, y: 0.0, z: 1.0 },
      radius:   0.1
    },
    Sphere {
      position: Vector3D { x: -1.0, y: 0.1, z: -3.0 },
      color: Vector3D { x: 0.0, y: 1.0, z: 0.0 },
      radius:   0.1
    }
  ];

  let mut ray = Ray {
    origin:    Vector3D { x: 0.0, y: 0.0, z: 0.0 },
    direction: Vector3D { x: 0.0, y: 0.0, z: 0.0 }
  };

  let lights = [
     Light {
      direction: Vector3D { x: -2.0, y: 10.0, z: -3.0 },
      intensity: 1.0,
    },
    // opposite angle
    Light {
      direction: Vector3D { x: 2.0, y: -10.0, z: -3.0 },
      intensity: 0.2,
    },
  ];

  for y in 0..HEIGHT {
    for x in 0..WIDTH {
      let mut pixel = [0, 0, 0];
      let rx = (((x as f32 + 0.5) / F_WIDTH) * 2.0 - 1.0) * ASPECT;
      let ry = 1.0 - ((y as f32 + 0.5) / F_HEIGHT) * 2.0;
      ray.direction = (Vector3D { x: rx, y: ry, z: -3.0 }).normalize();

      for sphere in &spheres {
        match sphere.intersects(&ray) {
          Some(distance) => {
            if distance >= 0.0 {
              let hit_point   = ray.origin.add(&ray.direction.mul(&distance));
              let normal      = sphere.surface_normal(&hit_point);

              let mut light_power = 0.0;
              for light in &lights {
                let light_dir   = light.direction.normalize().mul(&-1.0);
                light_power += normal.dot(&light_dir).max(0.0) * light.intensity;
              }

              pixel = sphere.color.mul(&light_power).rgb();
            }
          },
          None => {} // keep the pixel
        }
      }
      pixels.extend_from_slice(&pixel);
    }
  }

  let mut file = File::create("out.ppm")?;
  file.write_fmt(format_args!("P6 {} {} 255\n", WIDTH, HEIGHT))?;
  file.write_all(pixels.as_slice())?;
  Ok(())
}
