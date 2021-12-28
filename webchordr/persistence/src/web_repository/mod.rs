pub use self::catalog_web_repository::CatalogWebRepository;
pub use self::setlist_web_repository::SetlistWebRepository;
pub use self::settings_web_repository::SettingsWebRepository;
pub use self::web_repository_trait::WebRepositoryTrait;

mod catalog_web_repository;
mod setlist_web_repository;
mod settings_web_repository;
mod web_repository_trait;

// #[cfg(test)]
// mod test {
//
//     trait X {
//         fn xs(&mut self);
//     }
//     struct A {}
//     impl X for A {
//         fn xs(&mut self) {
//             println!("Xs")
//         }
//     }
//     trait Y {
//         fn ys(&mut self);
//     }
//
//     struct B<'a, T: X> {
//         inner: &'a T,
//     }
//     impl<'a, T: X> Y for B<'a, T> {
//         fn ys(&mut self) {
//             self.inner.xs()
//         }
//     }
//     #[test]
//     fn test() {
//         let a = A {};
//         let mut b = B { inner: &a };
//
//         b.ys();
//     }
// }
