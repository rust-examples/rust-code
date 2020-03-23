extern crate myapps;

use myapps::module1::public_submodule::public_function_with_too_common_or_weird_name as papperlapapp;
#[allow(unused_imports)]
use myapps::separatefile;
#[allow(unused_imports)]
use myapps::separatedir;
#[allow(unused_imports)]
use myapps::separatefileanddir;

fn main() {
    println!("Hello modules!");
    myapps::module1::public_submodule::public_function();

    use myapps::module1::public_submodule;
    public_submodule::get_some_internal_calculations();

    papperlapapp();
}
