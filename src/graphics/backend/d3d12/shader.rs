pub(crate) struct D3D12Shader {
    bytecode: &'static [u8],
}

impl D3D12Shader {
    pub(crate) const fn from_bytes(
        bytecode: &'static [u8],
    ) -> Self {
        Self {
            bytecode,
        }
    }

    pub(crate) fn bytecode(
        &self,
    ) -> &[u8] {
        self.bytecode
    }
}