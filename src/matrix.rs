//! 2DMatrix and transforms.
use crate::error::expect_mem_err;
use crate::geometry::Point;

pub(in crate) use self::private::Matrix2DOp;
mod private {
    use ffi::BLMatrix2DOp::*;    
    bl_enum! {
        #[doc(hidden)]
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
}

/// A Row-Major 2d matrix.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Matrix2D([f64; ffi::BLMatrix2DValue::BL_MATRIX2D_VALUE_COUNT as usize]);

impl Matrix2D {
    /// Creates a new matrix.
    #[inline]
    pub fn new(m00: f64, m01: f64, m10: f64, m11: f64, m20: f64, m21: f64) -> Self {
        Matrix2D([m00, m01, m10, m11, m20, m21])
    }

    /// Creates an identity matrix.
    #[inline]
    pub fn identity() -> Matrix2D {
        Matrix2D([1.0, 0.0, 0.0, 1.0, 0.0, 0.0])
    }

    /// Creates a translation matrix.
    #[inline]
    pub fn translation(x: f64, y: f64) -> Matrix2D {
        Matrix2D([1.0, 0.0, 0.0, 1.0, x, y])
    }

    /// Creates a translation matrix.
    #[inline]
    pub fn translation_point<P: Point>(p: &P) -> Matrix2D {
        let p = p.into_f64();
        Matrix2D([1.0, 0.0, 0.0, 1.0, p[0], p[1]])
    }

    /// Creates a scaling matrix.
    #[inline]
    pub fn scaling(x: f64, y: f64) -> Matrix2D {
        Matrix2D([x, 0.0, 0.0, y, 0.0, 0.0])
    }

    /// Creates a scaling matrix.
    #[inline]
    pub fn scaling_point<P: Point>(p: &P) -> Matrix2D {
        let p = p.into_f64();
        Matrix2D([p[0], 0.0, 0.0, p[1], 0.0, 0.0])
    }

    /// Creates a rotation matrix.
    #[inline]
    pub fn rotation(angle: f64, x: f64, y: f64) -> Matrix2D {
        let mut this = Matrix2D::identity();
        this.reset_to_rotation(angle, x, y);
        this
    }

    /// Creates a rotation matrix.
    pub fn rotation_point<P: Point>(angle: f64, p: &P) -> Matrix2D {
        let p = p.into_f64();
        let mut this = Matrix2D::identity();
        this.reset_to_rotation(angle, p[0], p[1]);
        this
    }

    /// Creates a skewing matrix.
    #[inline]
    pub fn skewing(x: f64, y: f64) -> Matrix2D {
        let mut this = Matrix2D::identity();
        this.reset_to_skewing(x, y);
        this
    }

    /// Creates a skewing matrix.
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

    /// Resets the matrix to the identity matrix.
    #[inline]
    pub fn reset(&mut self) {
        *self = Self::identity();
    }

    /// Resets the matrix to a translation matrix.
    #[inline]
    pub fn reset_to_translation(&mut self, x: f64, y: f64) {
        *self = Matrix2D([1.0, 0.0, 0.0, 1.0, x, y])
    }

    /// Resets the matrix to a translation matrix.
    #[inline]
    pub fn reset_to_translation_point<P: Point>(&mut self, p: &P) {
        let p = p.into_f64();
        self.reset_to_translation(p[0], p[1]);
    }

    /// Resets the matrix to a scaling matrix.
    #[inline]
    pub fn reset_to_scaling(&mut self, x: f64, y: f64) {
        *self = Matrix2D([x, 0.0, 0.0, y, 0.0, 0.0])
    }

    /// Resets the matrix to a scaling matrix.
    #[inline]
    pub fn reset_to_scaling_point<P: Point>(&mut self, p: &P) {
        let p = p.into_f64();
        self.reset_to_scaling(p[0], p[1]);
    }

    /// Resets the matrix to a skewing matrix.
    #[inline]
    pub fn reset_to_skewing(&mut self, x: f64, y: f64) {
        unsafe { ffi::blMatrix2DSetSkewing(self as *mut _ as *mut _, x, y) };
    }

    /// Resets the matrix to a skewing matrix.
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

    /// Resets the matrix to a rotation matrix.
    #[inline]
    pub fn reset_to_rotation(&mut self, angle: f64, x: f64, y: f64) {
        unsafe { ffi::blMatrix2DSetRotation(self as *mut _ as *mut _, angle, x, y) };
    }

    /// Resets the matrix to a rotation matrix.
    #[inline]
    pub fn reset_to_rotation_point<P: Point>(&mut self, angle: f64, p: &P) {
        let p = p.into_f64();
        self.reset_to_rotation(angle, p[0], p[1]);
    }
    #[inline]
    // Inverted src is writtinen into dst
    pub fn invert(dst: &mut Matrix2D, src: &Matrix2D) {               
        let src_p = src.0.as_ptr() as *const ffi::BLMatrix2D;
        let dst_p = dst.0.as_ptr() as *mut ffi::BLMatrix2D;
        unsafe{ ffi::blMatrix2DInvert(dst_p, src_p)};
    }
}

impl MatrixTransform for Matrix2D {
    #[inline]
    #[doc(hidden)]
    fn apply_matrix_op(&mut self, op: Matrix2DOp, data: &[f64]) {
        unsafe {
            expect_mem_err(ffi::blMatrix2DApplyOp(
                self as *mut _ as *mut _,
                op as u32,
                data.as_ptr() as *const _,
            ))
        };
    }
}

/// A trait for doing matrix transformations on the type.
pub trait MatrixTransform {
    #[doc(hidden)]
    fn apply_matrix_op(&mut self, op: Matrix2DOp, data: &[f64]);

    /// Set the transformation matrix of this type to m.
    #[inline]
    fn set_matrix(&mut self, m: &Matrix2D) {
        self.apply_matrix_op(Matrix2DOp::Assign, &m.0);
    }

    /// Reset the transformation matrix.
    #[inline]
    fn reset_matrix(&mut self) {
        self.apply_matrix_op(Matrix2DOp::Reset, &[]);
    }

    /// Translate the transformation matrix.
    #[inline]
    fn translate(&mut self, x: f64, y: f64) {
        self.apply_matrix_op(Matrix2DOp::Translate, &[x, y]);
    }

    /// Translate the transformation matrix.
    #[inline]
    fn translate_point<P: Point>(&mut self, p: &P) {
        self.apply_matrix_op(Matrix2DOp::Translate, &p.into_f64());
    }

    /// Scale the transformation matrix.
    #[inline]
    fn scale(&mut self, x: f64, y: f64) {
        self.apply_matrix_op(Matrix2DOp::Scale, &[x, y]);
    }

    /// Scale the transformation matrix.
    #[inline]
    fn scale_point<P: Point>(&mut self, p: &P) {
        self.apply_matrix_op(Matrix2DOp::Scale, &p.into_f64());
    }

    /// Skew the transformation matrix.
    #[inline]
    fn skew(&mut self, x: f64, y: f64) {
        self.apply_matrix_op(Matrix2DOp::Skew, &[x, y]);
    }

    /// Skew the transformation matrix.
    #[inline]
    fn skew_point<P: Point>(&mut self, p: &P) {
        self.apply_matrix_op(Matrix2DOp::Skew, &p.into_f64());
    }

    /// Rotate the transformation matrix.
    #[inline]
    fn rotate(&mut self, angle: f64) {
        self.apply_matrix_op(Matrix2DOp::Rotate, &[angle]);
    }

    /// Rotate the transformation matrix around a point.
    #[inline]
    fn rotate_around(&mut self, angle: f64, x: f64, y: f64) {
        self.apply_matrix_op(Matrix2DOp::RotatePoint, &[angle, x, y]);
    }

    /// Rotate the transformation matrix around a point.
    #[inline]
    fn rotate_around_point<P: Point>(&mut self, angle: f64, p: &P) {
        let arr = p.into_f64();
        self.apply_matrix_op(Matrix2DOp::RotatePoint, &[angle, arr[0], arr[1]]);
    }

    /// Transform the transformation matrix.
    #[inline]
    fn transform(&mut self, mat: &Matrix2D) {
        self.apply_matrix_op(Matrix2DOp::Transform, &mat.0);
    }

    /// Post-translate the transformation matrix.
    #[inline]
    fn post_translate(&mut self, x: f64, y: f64) {
        self.apply_matrix_op(Matrix2DOp::PostTranslate, &[x, y]);
    }

    /// Post-translate the transformation matrix.
    #[inline]
    fn post_translate_point<P: Point>(&mut self, p: &P) {
        self.apply_matrix_op(Matrix2DOp::PostTranslate, &p.into_f64());
    }

    /// Post-scale the transformation matrix.
    #[inline]
    fn post_scale(&mut self, x: f64, y: f64) {
        self.apply_matrix_op(Matrix2DOp::PostScale, &[x, y]);
    }

    /// Post-scale the transformation matrix.
    #[inline]
    fn post_scale_point<P: Point>(&mut self, p: &P) {
        self.apply_matrix_op(Matrix2DOp::PostScale, &p.into_f64());
    }

    /// Post-skew the transformation matrix.
    #[inline]
    fn post_skew(&mut self, x: f64, y: f64) {
        self.apply_matrix_op(Matrix2DOp::PostSkew, &[x, y]);
    }

    /// Post-skew the transformation matrix.
    #[inline]
    fn post_skew_point<P: Point>(&mut self, p: &P) {
        self.apply_matrix_op(Matrix2DOp::PostSkew, &p.into_f64());
    }

    /// Post-rotate the transformation matrix.
    #[inline]
    fn post_rotate(&mut self, angle: f64) {
        self.apply_matrix_op(Matrix2DOp::PostRotate, &[angle]);
    }

    /// Post-rotate the transformation matrix around a point.
    #[inline]
    fn post_rotate_around(&mut self, angle: f64, x: f64, y: f64) {
        self.apply_matrix_op(Matrix2DOp::PostRotatePoint, &[angle, x, y]);
    }

    /// Post-rotate the transformation matrix around a point.
    #[inline]
    fn post_rotate_around_point<P: Point>(&mut self, angle: f64, p: &P) {
        let arr = p.into_f64();
        self.apply_matrix_op(Matrix2DOp::PostRotatePoint, &[angle, arr[0], arr[1]]);
    }

    /// Post-transform the transformation matrix.
    #[inline]
    fn post_transform(&mut self, mat: &Matrix2D) {
        self.apply_matrix_op(Matrix2DOp::PostTransform, &mat.0);
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        let mut m = Matrix2D::scaling(3., 1.);
        let mut m2 = Matrix2D::identity();
        Matrix2D::invert(&mut m2, &m);
        m.transform(&m2);
        assert_eq!(m, Matrix2D::identity());
    }
}