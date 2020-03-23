#[allow(dead_code)]
pub mod module1 {
    mod private_submodule1 {
        pub fn public_function_in_a_private_module() {
            println!("Some internal calculations.");
        }
    }

    pub mod public_submodule {
        fn private_function() {}
        pub fn public_function() {
            println!("Public function executed.");
        }
        pub fn get_some_internal_calculations() {
            super::private_submodule1::public_function_in_a_private_module()
        }
        pub fn public_function_with_too_common_or_weird_name() {
            println!("All is well!");
        }
    }
}

pub fn some_public_function() {
    crate::module1::public_submodule::public_function()
}

pub mod separatefile;

pub mod separatedir;

pub mod separatefileanddir;
