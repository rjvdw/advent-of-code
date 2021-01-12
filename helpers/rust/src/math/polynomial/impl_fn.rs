// TODO: Implement FnOnce, FnMut and Fn for Polynomial, so we can do `y(x)`, rather than `y.at(x)`.
// impl FnOnce(i64) for Polynomial {
//     type Output = i64;
//
//     fn call_once(self, x: i64) -> Self::Output {
//         self.at(x)
//     }
// }
//
// impl FnMut(i64) for Polynomial{
//     fn call_mut(&mut self, x: i64) -> Self::Output {
//         self.at(x)
//     }
// }
//
// impl Fn(i64) for Polynomial {
//     fn call(&self, x: i64) -> Self::Output {
//         self.at(x)
//     }
// }
