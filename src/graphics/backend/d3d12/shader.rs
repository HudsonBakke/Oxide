use std::{
    fs,
    io,
    path::Path,
};

pub(crate) struct D3D12Shader {
    bytecode: Vec<u8>,
}

impl D3D12Shader {
    pub(crate) fn load(path: impl AsRef<Path>) -> io::Result<Self> {
        let bytecode = fs::read(path)?;

        Ok(Self { bytecode })
    }

    pub(crate) fn bytecode(&self) -> &[u8] {
        &self.bytecode
    }
}