use wdk_build::ConfigError;

fn main() -> Result<(), ConfigError> {
    let config = wdk_build::Config::from_env_auto()?;
    config.configure_binary_build();
    Ok(())
}
