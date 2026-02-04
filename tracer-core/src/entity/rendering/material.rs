use glam::{Vec3A, Vec4};

use crate::entity::{
    actor::{ActorTrait, DirectionalActorTrait},
    geometry::ray::{Ray, RayType},
    rendering::{Renderable, light::Light},
    scene::Scene,
};

pub trait MaterialTrait {
    fn calculate_illumination(
        &self,
        scene: &Scene,
        surface_normal: &Vec3A,
        ray: &Ray,
        light: &Light,
        light_ray: &Ray,
        start_color: &Vec4,
        current_depth: &usize,
    ) -> Vec4;
}

// ########################################

pub trait ColorMaterialTrait {
    fn get_color(&self) -> Vec4;
}

#[derive(Clone)]
pub struct ColorMaterial {
    color: Vec4,
}

impl ColorMaterial {
    pub fn new(color: Vec4) -> Self {
        Self { color: color }
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
        _start_color: &Vec4,
        _current_depth: &usize,
    ) -> Vec4 {
        self.color
    }
}

impl ColorMaterialTrait for ColorMaterial {
    fn get_color(&self) -> Vec4 {
        self.color
    }
}

// ########################################

#[derive(Clone)]
pub struct DiffuseMaterial {
    diffuse: f32,
}

pub trait DiffuseMaterialTrait {
    fn get_diffuse_coef(&self) -> f32;
}

impl DiffuseMaterial {
    pub fn new(diffuse: f32) -> Self {
        Self { diffuse: diffuse }
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
        start_color: &Vec4,
        _current_depth: &usize,
    ) -> Vec4 {
        let dot = light_ray.get_direction().dot(*surface_normal);
        let diffuse_vec = start_color * f32::max(0., dot);
        diffuse_vec * self.diffuse
    }
}

impl DiffuseMaterialTrait for DiffuseMaterial {
    fn get_diffuse_coef(&self) -> f32 {
        self.diffuse
    }
}

// ########################################

#[derive(Clone)]
pub struct SpecularMaterial {
    specular_reflection_coef: f32,
    shininess: f32,
}

pub trait SpecularMaterialTrait {
    fn get_specular_coef(&self) -> f32;
    fn get_shininess_coef(&self) -> f32;
}

impl MaterialTrait for SpecularMaterial {
    fn calculate_illumination(
        &self,
        _scene: &Scene,
        surface_normal: &Vec3A,
        _ray: &Ray,
        light: &Light,
        light_ray: &Ray,
        start_color: &Vec4,
        _current_depth: &usize,
    ) -> Vec4 {
        let income_vector = light_ray.get_position() - light.get_position();
        let income_vector_n = income_vector.normalize();

        let my_dot = income_vector_n.dot(*surface_normal);
        let my_len = 2. * my_dot;

        let temp_normal = surface_normal * my_len;
        let reflect_vector = temp_normal + income_vector_n;
        let reflect_vector_n = reflect_vector.normalize();

        let mut my_spec = f32::max(reflect_vector_n.dot(income_vector_n), 0.);
        my_spec = my_spec.powf(5.);

        let specular_color = Vec4::ONE * my_spec;
        (start_color + specular_color * self.specular_reflection_coef)
            .clamp(Vec4::ZERO, Vec4::new(255., 255., 255., 255.))
    }
}

impl SpecularMaterialTrait for SpecularMaterial {
    fn get_specular_coef(&self) -> f32 {
        self.specular_reflection_coef
    }
    fn get_shininess_coef(&self) -> f32 {
        self.shininess
    }
}

// ########################################

#[derive(Clone)]
pub struct ReflectiveMaterial {
    reflect_coef: f32,
    max_depth: usize,
}

pub trait ReflectiveMaterialTrait {
    fn get_reflection_coef(&self) -> f32;
    fn get_max_depth(&self) -> usize;
}

impl ReflectiveMaterialTrait for ReflectiveMaterial {
    fn get_reflection_coef(&self) -> f32 {
        self.reflect_coef
    }
    fn get_max_depth(&self) -> usize {
        self.max_depth
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
        start_color: &Vec4,
        current_depth: &usize,
    ) -> Vec4 {
        match *current_depth {
            x if x >= self.max_depth => *start_color,
            _ => {
                let direction = (ray.get_direction()
                    - surface_normal * 2. * ray.get_direction().dot(*surface_normal))
                .normalize();
                let start = light_ray.get_position() + 0.0001 * surface_normal;
                let relfection_ray = Ray::new(&start, &direction);
                let color = scene
                    .render(
                        &relfection_ray,
                        &light,
                        &RayType::Camera,
                        &(current_depth + 1),
                    )
                    .unwrap();
                start_color + self.reflect_coef * (color / (*current_depth as f32))
            }
        }
    }
}

// ########################################

#[derive(Clone)]
pub struct MaterialMixer {
    pub materials: Vec<MaterialType>,
}

impl MaterialMixer {
    pub fn new() -> Self {
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
        start_color: &Vec4,
        current_depth: &usize,
    ) -> Vec4 {
        let mut result_color = start_color.clone();

        self.materials.iter().for_each(|x| {
            result_color = x.calculate_illumination(
                scene,
                surface_normal,
                ray,
                light,
                light_ray,
                &result_color,
                current_depth,
            )
        });
        result_color
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
                start_color,
                current_depth,
            ),
            MaterialType::Diffuse(i) => i.calculate_illumination(
                scene,
                surface_normal,
                ray,
                light,
                light_ray,
                start_color,
                current_depth,
            ),
            MaterialType::Specular(i) => i.calculate_illumination(
                scene,
                surface_normal,
                ray,
                light,
                light_ray,
                start_color,
                current_depth,
            ),
            MaterialType::Reflective(i) => i.calculate_illumination(
                scene,
                surface_normal,
                ray,
                light,
                light_ray,
                start_color,
                current_depth,
            ),
            MaterialType::Mixer(i) => i.calculate_illumination(
                scene,
                surface_normal,
                ray,
                light,
                light_ray,
                start_color,
                current_depth,
            ),
        }
    }
}

pub trait MaterialBound {
    fn get_material(&self) -> &MaterialType;
}
