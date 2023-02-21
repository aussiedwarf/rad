use glam::*;

pub struct Camera{
  //matrix_camera: Mat4,
  matrix_view: Mat4,
  matrix_projection: Mat4,
  matrix_vp: Mat4,
  viewport_size: Vec2,
  viewport_offset: Vec2,
  screen_size: Vec2,
  screen_offset: Vec2,
  position: Vec3,
  target: Vec3,
  up: Vec3,
  near: f32,
  far: f32,
}

impl Camera{
  pub fn update(&mut self){

    //let z = Vec3::normalize(self.position - self.target);
    //let x = Vec3::cross(self.up, z);
    //let y = z - x;
    
    self.matrix_view = Mat4::look_at_rh(self.position, self.target, self.up);
    
    self.matrix_vp = self.matrix_projection * self.matrix_view;
  }

  pub fn set_viewport(&mut self, 
    a_viewport_size: Vec2,
    a_viewport_offset: Vec2,
    a_screen_size: Vec2,
    a_screen_offset: Vec2){

    self.viewport_size = a_viewport_size;
    self.viewport_offset = a_viewport_offset;
    self.screen_size = a_screen_size;
    self.screen_offset = a_screen_offset;
  }

  pub fn set_perspective(&mut self, a_fovy: f32, a_aspect: f32,
    a_near: f32, a_far: f32){
    self.matrix_projection = Mat4::perspective_rh_gl(a_fovy, a_aspect, a_near, a_far);
  }

  pub fn set_ortho(&mut self, 
    a_left: f32,
    a_right: f32,
    a_bottom: f32,
    a_top: f32,
    a_near: f32,
    a_far: f32)
  {
    self.matrix_projection = Mat4::orthographic_rh_gl(a_left, a_right, a_bottom, a_top, a_near, a_far);
  }

  pub fn new() -> Self{
    Camera{
      matrix_view: Mat4::IDENTITY,
      matrix_projection: Mat4::IDENTITY,
      matrix_vp: Mat4::IDENTITY,
      viewport_size: Vec2::ZERO,
      viewport_offset: Vec2::ZERO,
      screen_size: Vec2::ZERO,
      screen_offset: Vec2::ZERO,
      position: Vec3::ZERO,
      target: Vec3::Z,
      up: Vec3::Y,
      near: 0.01,
      far: 1.0,
    }
  }
}
