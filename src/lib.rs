// This includes the generated structs to make them available to the code
pub mod test_svc_merchandise {
    include!(concat!(env!("OUT_DIR"), "/test_svc.merchandise.rs"));
} // This module (and the generated structs) are defined here to be part of the lib's public API

#[cfg(test)]
mod tests {
    use crate::test_svc_merchandise;

    #[test]
    fn compile_test_proto() {
        let shirt1 = test_svc_merchandise::Shirt {
            ..Default::default()
        };

        assert_eq!(shirt1.graphics, None);
    }
}
