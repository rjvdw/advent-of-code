//! Three dimensional linear algebra

/// A three-dimensional vector
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Vector3d<T: Copy>(pub T, pub T, pub T);

impl<T: Copy + Default> Default for Vector3d<T> {
    fn default() -> Self {
        Vector3d(T::default(), T::default(), T::default())
    }
}

impl<Lhs, Rhs, O> std::ops::Add<Vector3d<Rhs>> for Vector3d<Lhs>
where
    Lhs: std::ops::Add<Rhs, Output = O> + Copy,
    Rhs: Copy,
    O: Copy,
{
    type Output = Vector3d<O>;

    fn add(self, rhs: Vector3d<Rhs>) -> Self::Output {
        Vector3d(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl<Lhs, Rhs> std::ops::AddAssign<Vector3d<Rhs>> for Vector3d<Lhs>
where
    Lhs: std::ops::Add<Rhs, Output = Lhs> + Copy,
    Rhs: Copy,
{
    fn add_assign(&mut self, rhs: Vector3d<Rhs>) {
        *self = *self + rhs
    }
}

impl<Lhs, Rhs, O> std::ops::Sub<Vector3d<Rhs>> for Vector3d<Lhs>
where
    Lhs: std::ops::Sub<Rhs, Output = O> + Copy,
    Rhs: Copy,
    O: Copy,
{
    type Output = Vector3d<O>;

    fn sub(self, rhs: Vector3d<Rhs>) -> Self::Output {
        Vector3d(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl<Lhs, Rhs> std::ops::SubAssign<Vector3d<Rhs>> for Vector3d<Lhs>
where
    Lhs: std::ops::Sub<Rhs, Output = Lhs> + Copy,
    Rhs: Copy,
{
    fn sub_assign(&mut self, rhs: Vector3d<Rhs>) {
        *self = *self - rhs
    }
}

impl<Lhs, Rhs, O> std::ops::Mul<Rhs> for Vector3d<Lhs>
where
    Lhs: std::ops::Mul<Rhs, Output = O> + Copy,
    Rhs: Copy,
    O: Copy,
{
    type Output = Vector3d<O>;

    fn mul(self, rhs: Rhs) -> Self::Output {
        Vector3d(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl<Lhs, Rhs> std::ops::MulAssign<Rhs> for Vector3d<Lhs>
where
    Lhs: std::ops::Mul<Rhs, Output = Lhs> + Copy,
    Rhs: Copy,
{
    fn mul_assign(&mut self, rhs: Rhs) {
        *self = *self * rhs
    }
}

// TODO: this implementation leads to an error:
//       error[E0275]: overflow evaluating the requirement `algebra3d::Vector<_>: Mul<algebra3d::Vector<_>>`
// impl<Lhs, Rhs, O> std::ops::Mul<Vector<Rhs>> for Lhs
//     where
//         Lhs: std::ops::Mul<Rhs, Output = O> + Copy,
//         Rhs: Copy,
//         O: Copy,
// {
//     type Output = Vector<O>;
//
//     fn mul(self, rhs: Vector<Rhs>) -> Self::Output {
//         rhs * self
//     }
// }

impl<Lhs, Rhs, O> std::ops::Div<Rhs> for Vector3d<Lhs>
where
    Lhs: std::ops::Div<Rhs, Output = O> + Copy,
    Rhs: Copy,
    O: Copy,
{
    type Output = Vector3d<O>;

    fn div(self, rhs: Rhs) -> Self::Output {
        Vector3d(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl<Lhs, Rhs> std::ops::DivAssign<Rhs> for Vector3d<Lhs>
where
    Lhs: std::ops::Div<Rhs, Output = Lhs> + Copy,
    Rhs: Copy,
{
    fn div_assign(&mut self, rhs: Rhs) {
        *self = *self / rhs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(
            Vector3d(10, 20, 30) + Vector3d(5, 6, 7),
            Vector3d(15, 26, 37)
        );
    }

    #[test]
    fn test_add_assign() {
        let mut v = Vector3d(10, 20, 30);
        v += Vector3d(5, 6, 7);
        assert_eq!(v, Vector3d(15, 26, 37));
    }

    #[test]
    fn test_sub() {
        assert_eq!(
            Vector3d(10, 20, 30) - Vector3d(5, 6, 7),
            Vector3d(5, 14, 23)
        );
    }

    #[test]
    fn test_sub_assign() {
        let mut v = Vector3d(10, 20, 30);
        v -= Vector3d(5, 6, 7);
        assert_eq!(v, Vector3d(5, 14, 23));
    }

    #[test]
    fn test_mul() {
        // assert_eq!(10 * Vector(1, 2, 3), Vector(10, 20, 30));
        assert_eq!(Vector3d(1, 2, 3) * 10, Vector3d(10, 20, 30));
    }

    #[test]
    fn test_mul_assign() {
        let mut v = Vector3d(1, 2, 3);
        v *= 10;
        assert_eq!(v, Vector3d(10, 20, 30));
    }

    #[test]
    fn test_div() {
        assert_eq!(Vector3d(10, 20, 30) / 10, Vector3d(1, 2, 3));
    }

    #[test]
    fn test_div_assign() {
        let mut v = Vector3d(10, 20, 30);
        v /= 10;
        assert_eq!(v, Vector3d(1, 2, 3));
    }
}
