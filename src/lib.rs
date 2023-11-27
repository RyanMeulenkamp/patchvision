#![feature(iter_intersperse)]
#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_assoc_type)]

pub(crate) mod round;
pub(crate) mod balloon;
pub mod panel;
pub(crate) mod placeholder;
pub(crate) mod template;
pub mod theme;
pub(crate) mod image;
pub mod slot;

#[cfg(test)]
mod tests {
    use crate::round::Round;

    #[test]
    fn test_round() {
        assert_eq!(15, 13usize.up(3));
        assert_eq!(12, 13usize.down(3));
    }
}
