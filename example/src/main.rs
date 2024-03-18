#[derive(Debug)]
enum ZeusError {
    DeviceNotFound,
    NotSupported,
    InsufficientPermission,
}

impl std::fmt::Display for ZeusError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ZeusError: {}", self)
    }
}

impl std::error::Error for ZeusError {}

trait MeasurePower2 {
    // User should implement this.
    fn measure_power(&self) -> Result<f64, ZeusError>;

    // User gets this for free.
    fn approximate_energy(&self, latency: f64) -> Result<f64, ZeusError> {
        match self.measure_power() {
            Ok(power) => {
                return Ok(power * latency);
            },
            Err(e) => {
                return Err(e);
            }
        }
    }
}

trait MeasurePower {
    // User should implement this.
    fn measure_power(&self) -> Result<f64, ZeusError>;

    // User gets this for free.
    fn approximate_energy(&self, latency: f64) -> Result<f64, ZeusError> {
        self.measure_power().map(|power| power * latency)
    }
}

struct CPU;

impl MeasurePower for CPU {
    fn measure_power(&self) -> Result<f64, ZeusError> {
        Ok(rapl::read_power() as f64)
    }
}

struct GPU;

impl MeasurePower for GPU {
    fn measure_power(&self) -> Result<f64, ZeusError> {
        Ok(nvml::deviceGetPowerUsage() as f64)
    }
}

fn main() {
    let cpu = CPU {};
    cpu.approximate_energy(0.1);

    let gpu = GPU {};
    gpu.approximate_energy(0.1);
}

fn print_energy<T>(device: T)
where
    T: MeasurePower,
{
    println!("Energy: {}", device.approximate_energy(0.1));
}
