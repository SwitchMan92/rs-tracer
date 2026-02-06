use glam::{FloatExt, Vec3A, Vec4};

use crate::entity::{
    actor::{ActorTrait, DirectionalActorTrait},
    geometry::ray::{Ray, RayType},
    rendering::{Renderable, light::Light},
    scene::Scene,
};

pub trait MaterialTrait {
    #[allow(clippy::too_many_arguments)]
    fn calculate_illumination(
        &self,
        scene: &Scene,
        surface_normal: &Vec3A,
        ray: &Ray,
        light: &Light,
        light_ray: &Ray,
        shadow_coef: &f32,
        start_color: &Vec4,
        current_depth: &usize,
    ) -> Vec4;
}

// ########################################

#[derive(Clone)]
pub struct ColorMaterial {
    color: Vec4,
}

impl ColorMaterial {
    pub fn new(color: Vec4) -> Self {
        Self { color }
    }
}

impl MaterialTrait for ColorMaterial {
    fn calculate_illumination(
        &self,
        _scene: &Scene,
        _surface_normal: &Vec3A,
        _ray: &Ray,
        _light: &Light,
        _light_ray: &Ray,
        _shadow_coef: &f32,
        _start_color: &Vec4,
        _current_depth: &usize,
    ) -> Vec4 {
        self.color
    }
}

// ########################################

#[derive(Clone)]
pub struct DiffuseMaterial {
    diffuse: f32,
}

impl DiffuseMaterial {
    pub fn new(diffuse: f32) -> Self {
        Self { diffuse }
    }
}

impl MaterialTrait for DiffuseMaterial {
    fn calculate_illumination(
        &self,
        _scene: &Scene,
        surface_normal: &Vec3A,
        _ray: &Ray,
        _light: &Light,
        light_ray: &Ray,
        shadow_coef: &f32,
        start_color: &Vec4,
        _current_depth: &usize,
    ) -> Vec4 {
        let dot = light_ray.get_direction().dot(*surface_normal);
        let diffuse_vec = start_color * f32::max(0., dot);
        ((start_color * diffuse_vec * self.diffuse) * shadow_coef).clamp(Vec4::ZERO, Vec4::ONE)
    }
}

// ########################################

#[derive(Clone)]
pub struct SpecularMaterial {
    specular_reflection_coef: f32,
    shininess: f32,
}

impl SpecularMaterial {
    pub fn new(specular_reflection_coef: f32, shininess: f32) -> Self {
        Self {
            specular_reflection_coef,
            shininess,
        }
    }
}

impl MaterialTrait for SpecularMaterial {
    fn calculate_illumination(
        &self,
        _scene: &Scene,
        surface_normal: &Vec3A,
        ray: &Ray,
        _light: &Light,
        light_ray: &Ray,
        _shadow_coef: &f32,
        start_color: &Vec4,
        _current_depth: &usize,
    ) -> Vec4 {
        let distance = light_ray.get_direction().length();
        let light_dir = light_ray.get_direction() / distance;
        let half_vector = (light_dir + ray.get_direction()).normalize();
        let ndoth = surface_normal.dot(half_vector);
        let specular_intensity = ndoth.saturate().powf(self.shininess);

        (start_color + specular_intensity * Vec4::ONE * self.specular_reflection_coef)
            .clamp(Vec4::ZERO, Vec4::ONE)
    }
}

// ########################################

#[derive(Clone)]
pub struct ReflectiveMaterial {
    reflect_coef: f32,
    max_depth: usize,
}

impl ReflectiveMaterial {
    pub fn new(reflect_coef: f32, max_depth: usize) -> Self {
        Self {
            reflect_coef,
            max_depth,
        }
    }
}

impl MaterialTrait for ReflectiveMaterial {
    fn calculate_illumination(
        &self,
        scene: &Scene,
        surface_normal: &Vec3A,
        ray: &Ray,
        light: &Light,
        light_ray: &Ray,
        _shadow_coef: &f32,
        start_color: &Vec4,
        current_depth: &usize,
    ) -> Vec4 {
        match *current_depth {
            x if x >= self.max_depth => *start_color,
            _ => {
                // let direction = (ray.get_direction()
                //     - surface_normal * 2. * surface_normal.dot(ray.get_direction()))
                // .normalize();

                let relfection_ray =
                    Ray::new(&light_ray.get_position(), &-light_ray.get_direction());
                let color = scene.render(
                    &relfection_ray,
                    light,
                    &RayType::Camera,
                    &(current_depth + 1),
                );

                match color {
                    None => *start_color,
                    Some(color) => {
                        (start_color + self.reflect_coef * color).clamp(Vec4::ZERO, Vec4::ONE)
                    }
                }
            }
        }
    }
}

// ########################################

#[derive(Clone)]
pub struct MaterialMixer {
    pub materials: Vec<MaterialType>,
}

impl Default for MaterialMixer {
    fn default() -> Self {
        Self {
            materials: Vec::<MaterialType>::with_capacity(1),
        }
    }
}

impl MaterialTrait for MaterialMixer {
    fn calculate_illumination(
        &self,
        scene: &Scene,
        surface_normal: &Vec3A,
        ray: &Ray,
        light: &Light,
        light_ray: &Ray,
        shadow_coef: &f32,
        start_color: &Vec4,
        current_depth: &usize,
    ) -> Vec4 {
        let mut result_color = *start_color;

        self.materials.iter().for_each(|x| {
            result_color = x.calculate_illumination(
                scene,
                surface_normal,
                ray,
                light,
                light_ray,
                shadow_coef,
                &result_color,
                current_depth,
            )
        });
        result_color.clamp(Vec4::ZERO, Vec4::ONE)
    }
}

// ########################################

#[derive(Clone)]
pub enum MaterialType {
    Color(ColorMaterial),
    Diffuse(DiffuseMaterial),
    Specular(SpecularMaterial),
    Reflective(ReflectiveMaterial),
    Mixer(MaterialMixer),
}

impl MaterialTrait for MaterialType {
    fn calculate_illumination(
        &self,
        scene: &Scene,
        surface_normal: &Vec3A,
        ray: &Ray,
        light: &Light,
        light_ray: &Ray,
        _shadow_coef: &f32,
        start_color: &Vec4,
        current_depth: &usize,
    ) -> Vec4 {
        match self {
            MaterialType::Color(i) => i.calculate_illumination(
                scene,
                surface_normal,
                ray,
                light,
                light_ray,
                _shadow_coef,
                start_color,
                current_depth,
            ),
            MaterialType::Diffuse(i) => i.calculate_illumination(
                scene,
                surface_normal,
                ray,
                light,
                light_ray,
                _shadow_coef,
                start_color,
                current_depth,
            ),
            MaterialType::Specular(i) => i.calculate_illumination(
                scene,
                surface_normal,
                ray,
                light,
                light_ray,
                _shadow_coef,
                start_color,
                current_depth,
            ),
            MaterialType::Reflective(i) => i.calculate_illumination(
                scene,
                surface_normal,
                ray,
                light,
                light_ray,
                _shadow_coef,
                start_color,
                current_depth,
            ),
            MaterialType::Mixer(i) => i.calculate_illumination(
                scene,
                surface_normal,
                ray,
                light,
                light_ray,
                _shadow_coef,
                start_color,
                current_depth,
            ),
        }
    }
}

pub trait MaterialBound {
    fn get_material(&self) -> &MaterialType;
}
