use anyhow::Result;

pub trait Fragment {
    fn process_interaction(&self) -> Result<()>;
}
