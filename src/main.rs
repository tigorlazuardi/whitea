use color_eyre::{eyre::WrapErr, Result};

fn main() -> Result<()> {
    color_eyre::install()?;
    let cfg = configuration::get_configuration().wrap_err("failed to parse config")?;
    backend::start_server(cfg)
}
