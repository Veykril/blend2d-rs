use crate::{
    error::{errcode_to_result, Result},
    geometry::Point,
};
use ffi::BLMatrix2DOp::*;

#[doc(hidden)]
bl_enum! {
    pub enum Matrix2DOp {
        Reset = BL_MATRIX2D_OP_RESET,
        Assign = BL_MATRIX2D_OP_ASSIGN,
        Translate = BL_MATRIX2D_OP_TRANSLATE,
        Scale = BL_MATRIX2D_OP_SCALE,
        Skew = BL_MATRIX2D_OP_SKEW,
        Rotate= BL_MATRIX2D_OP_ROTATE,
        RotatePoint = BL_MATRIX2D_OP_ROTATE_PT,
        Transform = BL_MATRIX2D_OP_TRANSFORM,
        PostTranslate = BL_MATRIX2D_OP_POST_TRANSLATE,
        PostScale = BL_MATRIX2D_OP_POST_SCALE,
        PostSkew = BL_MATRIX2D_OP_POST_SKEW,
        PostRotate = BL_MATRIX2D_OP_POST_ROTATE,
        PostRotatePoint = BL_MATRIX2D_OP_POST_ROTATE_PT,
        PostTransform = BL_MATRIX2D_OP_POST_TRANSFORM,
    }
    Default => Reset
}

// Row-Major
#[repr(transparent)]
#[derive(Default, Copy, Clone)]
pub struct Matrix2D([f64; ffi::BLMatrix2DValue::BL_MATRIX2D_VALUE_COUNT as usize]);

impl Matrix2D {
    #[inline]
    pub fn new(m00: f64, m01: f64, m10: f64, m11: f64, m20: f64, m21: f64) -> Self {
        Matrix2D([m00, m01, m10, m11, m20, m21])
    }

    #[inline]
    pub fn identity() -> Matrix2D {
        Matrix2D([1.0, 0.0, 0.0, 1.0, 0.0, 0.0])
    }

    #[inline]
    pub fn translation(x: f64, y: f64) -> Matrix2D {
        Matrix2D([1.0, 0.0, 0.0, 1.0, x, y])
    }

    #[inline]
    pub fn translation_point<P: Point>(p: &P) -> Matrix2D {
        let p = p.into_f64();
        Matrix2D([1.0, 0.0, 0.0, 1.0, p[0], p[1]])
    }

    #[inline]
    pub fn scaling(x: f64, y: f64) -> Matrix2D {
        Matrix2D([x, 0.0, 0.0, y, 0.0, 0.0])
    }

    #[inline]
    pub fn scaling_point<P: Point>(p: &P) -> Matrix2D {
        let p = p.into_f64();
        Matrix2D([p[0], 0.0, 0.0, p[1], 0.0, 0.0])
    }

    #[inline]
    pub fn rotation(angle: f64, x: f64, y: f64) -> Matrix2D {
        let mut this = Matrix2D::identity();
        this.reset_to_rotation(angle, x, y);
        this
    }

    #[inline]
    pub fn rotation_point<P: Point>(angle: f64, p: &P) -> Matrix2D {
        let p = p.into_f64();
        let mut this = Matrix2D::identity();
        this.reset_to_rotation(angle, p[0], p[1]);
        this
    }

    #[inline]
    pub fn skewing(x: f64, y: f64) -> Matrix2D {
        let mut this = Matrix2D::identity();
        this.reset_to_skewing(x, y);
        this
    }

    #[inline]
    pub fn skewing_point<P: Point>(p: &P) -> Matrix2D {
        let p = p.into_f64();
        let mut this = Matrix2D::identity();
        this.reset_to_skewing(p[0], p[1]);
        this
    }

    #[inline]
    pub fn sin_cos(sin: f64, cos: f64, tx: f64, ty: f64) -> Matrix2D {
        Matrix2D([cos, sin, -sin, cos, tx, ty])
    }

    #[inline]
    pub fn sin_cos_point<P: Point>(sin: f64, cos: f64, t: &P) -> Matrix2D {
        let t = t.into_f64();
        Self::sin_cos(sin, cos, t[0], t[1])
    }

    #[inline]
    pub fn reset(&mut self) {
        *self = Self::identity();
    }

    #[inline]
    pub fn reset_to_translation(&mut self, x: f64, y: f64) {
        *self = Matrix2D([1.0, 0.0, 0.0, 1.0, x, y])
    }

    #[inline]
    pub fn reset_to_translation_point<P: Point>(&mut self, p: &P) {
        let p = p.into_f64();
        self.reset_to_translation(p[0], p[1]);
    }

    #[inline]
    pub fn reset_to_scaling(&mut self, x: f64, y: f64) {
        *self = Matrix2D([x, 0.0, 0.0, y, 0.0, 0.0])
    }

    #[inline]
    pub fn reset_to_scaling_point<P: Point>(&mut self, p: &P) {
        let p = p.into_f64();
        self.reset_to_scaling(p[0], p[1]);
    }

    #[inline]
    pub fn reset_to_skewing(&mut self, x: f64, y: f64) {
        unsafe { ffi::blMatrix2DSetSkewing(self as *mut _ as *mut _, x, y) };
    }

    #[inline]
    pub fn reset_to_skewing_point<P: Point>(&mut self, p: &P) {
        let p = p.into_f64();
        self.reset_to_skewing(p[0], p[1]);
    }

    #[inline]
    pub fn reset_to_sin_cos(&mut self, sin: f64, cos: f64, tx: f64, ty: f64) {
        *self = Matrix2D([cos, sin, -sin, cos, tx, ty])
    }

    #[inline]
    pub fn reset_to_sin_cos_point<P: Point>(&mut self, sin: f64, cos: f64, t: &P) {
        let t = t.into_f64();
        self.reset_to_sin_cos(sin, cos, t[0], t[1]);
    }

    #[inline]
    pub fn reset_to_rotation(&mut self, angle: f64, x: f64, y: f64) {
        unsafe { ffi::blMatrix2DSetRotation(self as *mut _ as *mut _, angle, x, y) };
    }

    #[inline]
    pub fn reset_to_rotation_point<P: Point>(&mut self, angle: f64, p: &P) {
        let p = p.into_f64();
        self.reset_to_rotation(angle, p[0], p[1]);
    }
}

impl MatrixTransform for Matrix2D {
    #[doc(hidden)]
    fn apply_matrix_op(&mut self, op: Matrix2DOp, data: &[f64]) -> Result<()> {
        unsafe {
            errcode_to_result(ffi::blMatrix2DApplyOp(
                self as *mut _ as *mut _,
                op.into(),
                data.as_ptr() as *const _,
            ))
        }
    }
}

pub trait MatrixTransform {
    #[doc(hidden)]
    fn apply_matrix_op(&mut self, op: Matrix2DOp, data: &[f64]) -> Result<()>;

    #[inline]
    fn set_matrix(&mut self, m: &Matrix2D) -> Result<()> {
        self.apply_matrix_op(Matrix2DOp::Assign, &m.0)
    }

    #[inline]
    fn reset_matrix(&mut self) -> Result<()> {
        self.apply_matrix_op(Matrix2DOp::Reset, &[])
    }

    #[inline]
    fn translate(&mut self, x: f64, y: f64) -> Result<()> {
        self.apply_matrix_op(Matrix2DOp::Translate, &[x, y])
    }

    #[inline]
    fn translate_point<P: Point>(&mut self, p: &P) -> Result<()> {
        self.apply_matrix_op(Matrix2DOp::Translate, &p.into_f64())
    }

    #[inline]
    fn scale(&mut self, x: f64, y: f64) -> Result<()> {
        self.apply_matrix_op(Matrix2DOp::Scale, &[x, y])
    }

    #[inline]
    fn scale_point<P: Point>(&mut self, p: &P) -> Result<()> {
        self.apply_matrix_op(Matrix2DOp::Scale, &p.into_f64())
    }

    #[inline]
    fn skew(&mut self, x: f64, y: f64) -> Result<()> {
        self.apply_matrix_op(Matrix2DOp::Skew, &[x, y])
    }

    #[inline]
    fn skew_point<P: Point>(&mut self, p: &P) -> Result<()> {
        self.apply_matrix_op(Matrix2DOp::Skew, &p.into_f64())
    }

    #[inline]
    fn rotate(&mut self, angle: f64) -> Result<()> {
        self.apply_matrix_op(Matrix2DOp::Rotate, &[angle])
    }

    #[inline]
    fn rotate_around(&mut self, angle: f64, x: f64, y: f64) -> Result<()> {
        self.apply_matrix_op(Matrix2DOp::RotatePoint, &[angle, x, y])
    }

    #[inline]
    fn rotate_around_point<P: Point>(&mut self, angle: f64, p: &P) -> Result<()> {
        let arr = p.into_f64();
        self.apply_matrix_op(Matrix2DOp::RotatePoint, &[angle, arr[0], arr[1]])
    }

    #[inline]
    fn transform(&mut self, mat: &Matrix2D) -> Result<()> {
        self.apply_matrix_op(Matrix2DOp::Transform, &mat.0)
    }

    #[inline]
    fn post_translate(&mut self, x: f64, y: f64) -> Result<()> {
        self.apply_matrix_op(Matrix2DOp::PostTranslate, &[x, y])
    }

    #[inline]
    fn post_translate_point<P: Point>(&mut self, p: &P) -> Result<()> {
        self.apply_matrix_op(Matrix2DOp::PostTranslate, &p.into_f64())
    }

    #[inline]
    fn post_scale(&mut self, x: f64, y: f64) -> Result<()> {
        self.apply_matrix_op(Matrix2DOp::PostScale, &[x, y])
    }

    #[inline]
    fn post_scale_point<P: Point>(&mut self, p: &P) -> Result<()> {
        self.apply_matrix_op(Matrix2DOp::PostScale, &p.into_f64())
    }

    #[inline]
    fn post_skew(&mut self, x: f64, y: f64) -> Result<()> {
        self.apply_matrix_op(Matrix2DOp::PostSkew, &[x, y])
    }

    #[inline]
    fn post_skew_point<P: Point>(&mut self, p: &P) -> Result<()> {
        self.apply_matrix_op(Matrix2DOp::PostSkew, &p.into_f64())
    }

    #[inline]
    fn post_rotate(&mut self, angle: f64) -> Result<()> {
        self.apply_matrix_op(Matrix2DOp::PostRotate, &[angle])
    }

    #[inline]
    fn post_rotate_around(&mut self, angle: f64, x: f64, y: f64) -> Result<()> {
        self.apply_matrix_op(Matrix2DOp::PostRotatePoint, &[angle, x, y])
    }

    #[inline]
    fn post_rotate_around_point<P: Point>(&mut self, angle: f64, p: &P) -> Result<()> {
        let arr = p.into_f64();
        self.apply_matrix_op(Matrix2DOp::PostRotatePoint, &[angle, arr[0], arr[1]])
    }

    #[inline]
    fn post_transform(&mut self, mat: &Matrix2D) -> Result<()> {
        self.apply_matrix_op(Matrix2DOp::PostTransform, &mat.0)
    }
}
