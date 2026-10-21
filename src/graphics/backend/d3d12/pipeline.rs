use windows::{
    core::Result,
    Win32::Graphics::{
        Direct3D12::{
            D3D12SerializeRootSignature,
            ID3D12PipelineState,
            ID3D12RootSignature,
            D3D12_BLEND_DESC,
            D3D12_COLOR_WRITE_ENABLE_ALL,
            D3D12_COMPARISON_FUNC_ALWAYS,
            D3D12_CULL_MODE_BACK,
            D3D12_DEPTH_STENCIL_DESC,
            D3D12_DEPTH_WRITE_MASK_ZERO,
            D3D12_FILL_MODE_SOLID,
            D3D12_GRAPHICS_PIPELINE_STATE_DESC,
            D3D12_INPUT_LAYOUT_DESC,
            D3D12_PIPELINE_STATE_FLAG_NONE,
            D3D12_PRIMITIVE_TOPOLOGY_TYPE_TRIANGLE,
            D3D12_RASTERIZER_DESC,
            D3D12_RENDER_TARGET_BLEND_DESC,
            D3D12_ROOT_SIGNATURE_DESC,
            D3D12_ROOT_SIGNATURE_FLAG_ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT,
            D3D_ROOT_SIGNATURE_VERSION_1,
            D3D12_SHADER_BYTECODE,
            D3D12_INPUT_ELEMENT_DESC,
            D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA,
        },
        Dxgi::Common::{
            DXGI_FORMAT_R8G8B8A8_UNORM,
            DXGI_FORMAT_UNKNOWN,
            DXGI_SAMPLE_DESC,
            DXGI_FORMAT_R32G32B32_FLOAT,
            DXGI_FORMAT_R32G32_FLOAT,
        },
    },
};

use super::{
    D3D12Device,
    D3D12Shader,
};

pub(crate) struct D3D12Pipeline {
    root_signature: ID3D12RootSignature,
    pipeline_state: ID3D12PipelineState,
}

impl D3D12Pipeline {
    pub(crate) fn new(
        device: &D3D12Device,
        vertex_shader: &D3D12Shader,
        pixel_shader: &D3D12Shader,
    ) -> Result<Self> {
        let root_desc = D3D12_ROOT_SIGNATURE_DESC {
            NumParameters: 0,
            pParameters: std::ptr::null(),
            NumStaticSamplers: 0,
            pStaticSamplers: std::ptr::null(),
            Flags: D3D12_ROOT_SIGNATURE_FLAG_ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT,
        };

        let mut signature_blob = None;
        let mut error_blob = None;

        unsafe {
            D3D12SerializeRootSignature(
                &root_desc,
                D3D_ROOT_SIGNATURE_VERSION_1,
                &mut signature_blob,
                Some(&mut error_blob),
            )?;
        }

        let signature_blob = signature_blob.expect(
            "root signature serialization succeeded without producing a blob"
        );

        let root_signature: ID3D12RootSignature = unsafe {
            device.device().CreateRootSignature(
                0,
                std::slice::from_raw_parts(
                    signature_blob.GetBufferPointer() as *const u8,
                    signature_blob.GetBufferSize(),
                ),
            )?
        };

        let vertex_bytecode = D3D12_SHADER_BYTECODE {
            pShaderBytecode: vertex_shader.bytecode().as_ptr() as *const _,
            BytecodeLength: vertex_shader.bytecode().len(),
        };

        let pixel_bytecode = D3D12_SHADER_BYTECODE {
            pShaderBytecode: pixel_shader.bytecode().as_ptr() as *const _,
            BytecodeLength: pixel_shader.bytecode().len(),
        };

        let rasterizer = D3D12_RASTERIZER_DESC {
            FillMode: D3D12_FILL_MODE_SOLID,
            CullMode: D3D12_CULL_MODE_BACK,
            FrontCounterClockwise: false.into(),
            DepthBias: 0,
            DepthBiasClamp: 0.0,
            SlopeScaledDepthBias: 0.0,
            DepthClipEnable: true.into(),
            MultisampleEnable: false.into(),
            AntialiasedLineEnable: false.into(),
            ForcedSampleCount: 0,
            ConservativeRaster: Default::default(),
        };

        let render_target_blend = D3D12_RENDER_TARGET_BLEND_DESC {
            BlendEnable: false.into(),
            LogicOpEnable: false.into(),
            SrcBlend: Default::default(),
            DestBlend: Default::default(),
            BlendOp: Default::default(),
            SrcBlendAlpha: Default::default(),
            DestBlendAlpha: Default::default(),
            BlendOpAlpha: Default::default(),
            LogicOp: Default::default(),
            RenderTargetWriteMask: D3D12_COLOR_WRITE_ENABLE_ALL.0 as u8,
        };

        let blend_state = D3D12_BLEND_DESC {
            AlphaToCoverageEnable: false.into(),
            IndependentBlendEnable: false.into(),
            RenderTarget: [render_target_blend; 8],
        };

        let depth_stencil = D3D12_DEPTH_STENCIL_DESC {
            DepthEnable: false.into(),
            DepthWriteMask: D3D12_DEPTH_WRITE_MASK_ZERO,
            DepthFunc: D3D12_COMPARISON_FUNC_ALWAYS,
            StencilEnable: false.into(),
            StencilReadMask: 0,
            StencilWriteMask: 0,
            FrontFace: Default::default(),
            BackFace: Default::default(),
        };

        let mut rtv_formats = [DXGI_FORMAT_UNKNOWN; 8];
        rtv_formats[0] = DXGI_FORMAT_R8G8B8A8_UNORM;

        let position_semantic = b"POSITION\0";
        let normal_semantic = b"NORMAL\0";
        let tex_coord_semantic = b"TEXCOORD\0";

        let input_elements = [
            D3D12_INPUT_ELEMENT_DESC {
                SemanticName: windows::core::PCSTR(
                    position_semantic.as_ptr()
                ),
                SemanticIndex: 0,
                Format: DXGI_FORMAT_R32G32B32_FLOAT,
                InputSlot: 0,
                AlignedByteOffset: 0,
                InputSlotClass:
                    D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA,
                InstanceDataStepRate: 0,
            },

            D3D12_INPUT_ELEMENT_DESC {
                SemanticName: windows::core::PCSTR(
                    normal_semantic.as_ptr()
                ),
                SemanticIndex: 0,
                Format: DXGI_FORMAT_R32G32B32_FLOAT,
                InputSlot: 0,
                AlignedByteOffset: 12,
                InputSlotClass:
                    D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA,
                InstanceDataStepRate: 0,
            },

            D3D12_INPUT_ELEMENT_DESC {
                SemanticName: windows::core::PCSTR(
                    tex_coord_semantic.as_ptr()
                ),
                SemanticIndex: 0,
                Format: DXGI_FORMAT_R32G32_FLOAT,
                InputSlot: 0,
                AlignedByteOffset: 24,
                InputSlotClass:
                    D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA,
                InstanceDataStepRate: 0,
            },
        ];

        let mut pipeline_desc = D3D12_GRAPHICS_PIPELINE_STATE_DESC {
            pRootSignature: std::mem::ManuallyDrop::new(
                Some(root_signature.clone())
            ),

            VS: vertex_bytecode,
            PS: pixel_bytecode,

            BlendState: blend_state,
            SampleMask: u32::MAX,
            RasterizerState: rasterizer,
            DepthStencilState: depth_stencil,

            InputLayout: D3D12_INPUT_LAYOUT_DESC {
                pInputElementDescs: input_elements.as_ptr(),
                NumElements: input_elements.len() as u32,
            },

            PrimitiveTopologyType:
                D3D12_PRIMITIVE_TOPOLOGY_TYPE_TRIANGLE,

            NumRenderTargets: 1,
            RTVFormats: rtv_formats,

            DSVFormat: DXGI_FORMAT_UNKNOWN,

            SampleDesc: DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            },

            NodeMask: 0,
            CachedPSO: Default::default(),
            Flags: D3D12_PIPELINE_STATE_FLAG_NONE,

            ..Default::default()
        };

        let pipeline_state: ID3D12PipelineState = unsafe {
            device
                .device()
                .CreateGraphicsPipelineState(&pipeline_desc)?
        };

        unsafe {
            std::mem::ManuallyDrop::drop(
                &mut pipeline_desc.pRootSignature
            );
        }

        Ok(Self {
            root_signature,
            pipeline_state,
        })
    }

    pub(crate) fn root_signature(&self) -> &ID3D12RootSignature {
        &self.root_signature
    }

    pub(crate) fn pipeline_state(&self) -> &ID3D12PipelineState {
        &self.pipeline_state
    }
}