use rppal::gpio::Gpio;

trait Puls {
    fn generate_puls(&self, target_pin: dyn Gpio, times: u32) -> void {
        let a = Gpio::new()?.get(1)?.into_output();
        Gpio
    }
}
