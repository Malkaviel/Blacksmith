use toml;
use serde_derive;

use std::io::BufReader;
use std::io::Read;

use core::engine_support_systems::error_handling::error::{GameResult, GameError};
use core::engine_support_systems::system_management::systems::filesystems::VFilesystem;
use core::engine_support_systems::system_management::systems::filesystems::RootDir;
//TODO: EngineConfig need a default impl with default mapping of options.

//Here what the file should look like :

/*
[graphic]
key = value

[physic]
...

[debug]
flush = true/false

[input]
"move_up = "Z"
...
*/



#[derive(Deserialize, Serialize, Debug)]
pub struct EngineConfig {
    debug_options: DebugOptions,
}
impl EngineConfig {
    pub fn new(debug_options: DebugOptions) -> Self {
        EngineConfig {
            debug_options,
        }
    }

    //Deserialize a toml string to an EngineConfig rust structure.
    fn load_config(filesystem: &Box<VFilesystem>) -> GameResult<Self> {
        let mut bufreader = BufReader::new(filesystem.open(RootDir::UserEngineConfigurationRoot, "engine_configuration.toml")?);
        let mut toml_string = String::new();
        bufreader.read_to_string(&mut toml_string)?;
        Ok(toml::from_str(toml_string.as_str())?)

    }

    //Serialize a EngineConfig to TOML string, to be saved in a toml file.
    fn save_config(&self, filesystem: &Box<VFilesystem>) -> GameResult<()> {
        let toml_string = toml::to_string(&self)?;

        //override the existing engine_configuration.toml
        filesystem.create(RootDir::UserEngineConfigurationRoot, "engine_configuration.toml")?.write_all(toml_string.as_bytes())?;
        Ok(())
    }
}




#[derive(Deserialize, Serialize, Debug)]
pub struct DebugOptions {
    flush: bool,
}
impl DebugOptions {
    fn new(flush: bool) -> Self {
        DebugOptions {
            flush,
        }
    }
}

//TODO: EngineConfig tests
#[cfg(test)]
mod engine_config_test {
    use super::*;
    use systems::platforms::linux::filesystem::Filesystem;
    use app_dirs;

    #[test]
    fn engine_config_serialization_deserialization() {
        let engine_configuration = EngineConfig::new(DebugOptions::new(false));
        assert!(!engine_configuration.debug_options.flush);
        //Serialize the EngineConfig rust structure to a TOML string.
        let toml = toml::to_string(&engine_configuration).unwrap();

        //Deserialize the TOML string to an EngineConfig rust structure.
        let new_engine_configuration: EngineConfig = toml::from_str(toml.as_str()).unwrap();
        assert!(!new_engine_configuration.debug_options.flush)

    }

    #[test]
    fn engine_config_load_save_config() {
        let filesystem = Box::new(Filesystem::new(app_dirs::AppInfo{name: "test_engine_config_blacksmith", author: "Malkaviel"}).unwrap()) as Box<VFilesystem>;
        let engine_configuration = EngineConfig::new(DebugOptions::new(false));
        engine_configuration.save_config(&filesystem);

        let new_engine_configuration = EngineConfig::load_config(&filesystem).unwrap();
        assert!(!new_engine_configuration.debug_options.flush);

        //cleanup.
        filesystem.rmrf(RootDir::UserDataRoot, "").unwrap();
        filesystem.rmrf(RootDir::UserConfigRoot, "").unwrap();
    }
}