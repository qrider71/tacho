extern crate yaml_rust;
use yaml_rust::{YamlLoader, YamlEmitter};

use std::fs;

pub struct TachoYaml {
    sets: TachoYamlSet[]
}

pub struct TachoYamlSet {
    command: String,
    args: String[]
}

/*

    Read yaml file into a string

    tacho_suite:
        tacho_sets:
            - tacho_set:
                command: myCommand
                tag: myTag
                args:
                    - arg1
                    - arg2
                    - arg3
            - tacho_set:

            - tacho_set:

        
    });
*/
pub fn read_config(filename:String) -> Result<String, String> {
    let yaml_raw = fs::read_to_string("address.txt")?
    let docs = YamlLoader::load_from_str(yaml_raw).unwrap();
}