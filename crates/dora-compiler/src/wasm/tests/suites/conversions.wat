(module
  (func (export "i64.extend_i32_s") (param $x i32) (result i64) (i64.extend_i32_s (local.get $x)))
  (func (export "i64.extend_i32_u") (param $x i32) (result i64) (i64.extend_i32_u (local.get $x)))
  (func (export "i32.wrap_i64") (param $x i64) (result i32) (i32.wrap_i64 (local.get $x)))
  (func (export "i32.trunc_f32_s") (param $x f32) (result i32) (i32.trunc_f32_s (local.get $x)))
  (func (export "i32.trunc_f32_u") (param $x f32) (result i32) (i32.trunc_f32_u (local.get $x)))
  (func (export "i32.trunc_f64_s") (param $x f64) (result i32) (i32.trunc_f64_s (local.get $x)))
  (func (export "i32.trunc_f64_u") (param $x f64) (result i32) (i32.trunc_f64_u (local.get $x)))
  (func (export "i64.trunc_f32_s") (param $x f32) (result i64) (i64.trunc_f32_s (local.get $x)))
  (func (export "i64.trunc_f32_u") (param $x f32) (result i64) (i64.trunc_f32_u (local.get $x)))
  (func (export "i64.trunc_f64_s") (param $x f64) (result i64) (i64.trunc_f64_s (local.get $x)))
  (func (export "i64.trunc_f64_u") (param $x f64) (result i64) (i64.trunc_f64_u (local.get $x)))
  (func (export "i32.trunc_sat_f32_s") (param $x f32) (result i32) (i32.trunc_sat_f32_s (local.get $x)))
  (func (export "i32.trunc_sat_f32_u") (param $x f32) (result i32) (i32.trunc_sat_f32_u (local.get $x)))
  (func (export "i32.trunc_sat_f64_s") (param $x f64) (result i32) (i32.trunc_sat_f64_s (local.get $x)))
  (func (export "i32.trunc_sat_f64_u") (param $x f64) (result i32) (i32.trunc_sat_f64_u (local.get $x)))
  (func (export "i64.trunc_sat_f32_s") (param $x f32) (result i64) (i64.trunc_sat_f32_s (local.get $x)))
  (func (export "i64.trunc_sat_f32_u") (param $x f32) (result i64) (i64.trunc_sat_f32_u (local.get $x)))
  (func (export "i64.trunc_sat_f64_s") (param $x f64) (result i64) (i64.trunc_sat_f64_s (local.get $x)))
  (func (export "i64.trunc_sat_f64_u") (param $x f64) (result i64) (i64.trunc_sat_f64_u (local.get $x)))
  (func (export "f32.convert_i32_s") (param $x i32) (result f32) (f32.convert_i32_s (local.get $x)))
  (func (export "f32.convert_i64_s") (param $x i64) (result f32) (f32.convert_i64_s (local.get $x)))
  (func (export "f64.convert_i32_s") (param $x i32) (result f64) (f64.convert_i32_s (local.get $x)))
  (func (export "f64.convert_i64_s") (param $x i64) (result f64) (f64.convert_i64_s (local.get $x)))
  (func (export "f32.convert_i32_u") (param $x i32) (result f32) (f32.convert_i32_u (local.get $x)))
  (func (export "f32.convert_i64_u") (param $x i64) (result f32) (f32.convert_i64_u (local.get $x)))
  (func (export "f64.convert_i32_u") (param $x i32) (result f64) (f64.convert_i32_u (local.get $x)))
  (func (export "f64.convert_i64_u") (param $x i64) (result f64) (f64.convert_i64_u (local.get $x)))
  (func (export "f64.promote_f32") (param $x f32) (result f64) (f64.promote_f32 (local.get $x)))
  (func (export "f32.demote_f64") (param $x f64) (result f32) (f32.demote_f64 (local.get $x)))
  (func (export "f32.reinterpret_i32") (param $x i32) (result f32) (f32.reinterpret_i32 (local.get $x)))
  (func (export "f64.reinterpret_i64") (param $x i64) (result f64) (f64.reinterpret_i64 (local.get $x)))
  (func (export "i32.reinterpret_f32") (param $x f32) (result i32) (i32.reinterpret_f32 (local.get $x)))
  (func (export "i64.reinterpret_f64") (param $x f64) (result i64) (i64.reinterpret_f64 (local.get $x)))
)
