use crate::tracer::{
    materials::{DiffuseLight, Lambertian},
    objects::{BoxEntity, RectXY, RectXZ, RectYZ},
    textures::ConstantTexture,
    transforms::FlipNormals,
    Camera, HitableList, Vec3,
};

use std::sync::Arc;

pub fn cornell_box() -> (HitableList, Camera) {
    let green = Lambertian::new_arc(ConstantTexture::new(Vec3::new(0.12, 0.45, 0.15)));
    let red = Lambertian::new_arc(ConstantTexture::new(Vec3::new(0.65, 0.05, 0.05)));
    let white = Lambertian::new_arc(ConstantTexture::new(Vec3::new(0.73, 0.73, 0.73)));
    let white2 = Lambertian::new_arc(ConstantTexture::new(Vec3::new(0.73, 0.73, 0.73)));
    let white3 = Lambertian::new_arc(ConstantTexture::new(Vec3::new(0.73, 0.73, 0.73)));
    let light = DiffuseLight::new_arc(ConstantTexture::new(Vec3::new(15.0, 15.0, 15.0)));
    let look_from = Vec3::new(278.0, 278.0, -800.0);
    let look_at = Vec3::new(278.0, 278.0, 0.0);

    (
        HitableList {
            hitables: vec![
                FlipNormals::new(RectYZ::new(0.0, 555.0, 0.0, 555.0, 555.0, green)),
                RectYZ::new(0.0, 555.0, 0.0, 555.0, 0.0, red),
                RectXZ::new(213.0, 343.0, 227.0, 332.0, 554.0, light),
                FlipNormals::new(RectXZ::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone())),
                RectXZ::new(0.0, 555.0, 0.0, 555.0, 0.0, white.clone()),
                FlipNormals::new(RectXY::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone())),
                BoxEntity::new(Vec3::new(130.0, 0.0, 65.0), Vec3::new(295.0, 165.0, 230.0), white.clone()),
                BoxEntity::new(Vec3::new(265.0, 0.0, 295.0), Vec3::new(430.0, 330.0, 460.0), white.clone())
            ],
        },
        Camera::new(
            look_from,
            look_at,
            Vec3::new(0.0, 1.0, 0.0),
            37.0,
            1.0,
            0.0,
            10.0,
        ),
    )
}