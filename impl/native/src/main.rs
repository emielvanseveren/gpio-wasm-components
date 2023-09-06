use anyhow::Result;


#[tracing::instrument(name = "native", skip(self))]
async fn main() -> anyhow::Result<()> {
    let mut chip = Chip::new("/dev/gpiochip0")?;
    let output = chip.get_line(17)?;

    // 0 is the default value it should have when it is configured as an output
    let output_handle = output.request(LineRequestFlags::OUTPUT, 0, "who-is-using-this")?;
    output_handle.set_value(1);

    Ok(())
}

