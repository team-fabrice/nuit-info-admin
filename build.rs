use ructe::Ructe;

fn main() -> ructe::Result<()> {
    Ructe::from_env()?.compile_templates("templates")
}
