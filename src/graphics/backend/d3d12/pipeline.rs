use windows::Win32::Graphics::Direct3D12::{
    D3D12_COMPARISON_FUNC_LESS_EQUAL, D3D12_FILTER_COMPARISON_MIN_MAG_LINEAR_MIP_POINT,
    D3D12_TEXTURE_ADDRESS_MODE_BORDER, D3D12_STATIC_BORDER_COLOR_OPAQUE_WHITE,
};
use windows::{
    Win32::Graphics::{
        Direct3D12::{
            D3D_ROOT_SIGNATURE_VERSION_1, D3D12_BLEND_DESC, D3D12_COLOR_WRITE_ENABLE_ALL,
            D3D12_COMPARISON_FUNC_ALWAYS, D3D12_COMPARISON_FUNC_LESS, D3D12_CULL_MODE_BACK,
            D3D12_DEPTH_STENCIL_DESC, D3D12_DEPTH_WRITE_MASK_ALL, D3D12_DESCRIPTOR_RANGE,
            D3D12_DESCRIPTOR_RANGE_TYPE_SRV, D3D12_FILL_MODE_SOLID,
            D3D12_FILTER_MIN_MAG_MIP_LINEAR, D3D12_GRAPHICS_PIPELINE_STATE_DESC,
            D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA, D3D12_INPUT_ELEMENT_DESC,
            D3D12_INPUT_LAYOUT_DESC, D3D12_PIPELINE_STATE_FLAG_NONE,
            D3D12_PRIMITIVE_TOPOLOGY_TYPE_TRIANGLE, D3D12_RASTERIZER_DESC,
            D3D12_RENDER_TARGET_BLEND_DESC, D3D12_ROOT_DESCRIPTOR, D3D12_ROOT_DESCRIPTOR_TABLE,
            D3D12_ROOT_PARAMETER, D3D12_ROOT_PARAMETER_0, D3D12_ROOT_PARAMETER_TYPE_CBV,
            D3D12_ROOT_PARAMETER_TYPE_DESCRIPTOR_TABLE, D3D12_ROOT_SIGNATURE_DESC,
            D3D12_ROOT_SIGNATURE_FLAG_ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT, D3D12_SHADER_BYTECODE,
            D3D12_SHADER_VISIBILITY_ALL, D3D12_SHADER_VISIBILITY_PIXEL,
            D3D12_STATIC_BORDER_COLOR_OPAQUE_BLACK, D3D12_STATIC_SAMPLER_DESC,
            D3D12_TEXTURE_ADDRESS_MODE_WRAP, D3D12SerializeRootSignature, ID3D12PipelineState,
            ID3D12RootSignature,
        },
        Dxgi::Common::{
            DXGI_FORMAT_D32_FLOAT, DXGI_FORMAT, DXGI_FORMAT_R16G16B16A16_FLOAT,
            DXGI_FORMAT_R32G32_FLOAT,
            DXGI_FORMAT_R32G32B32_FLOAT, DXGI_FORMAT_R32G32B32A32_FLOAT, DXGI_FORMAT_UNKNOWN,
            DXGI_SAMPLE_DESC,
        },
    },
    core::Result,
};

use super::{D3D12Device, D3D12Shader};

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
        Self::new_mesh_pipeline(device, vertex_shader, pixel_shader, false)
    }

    pub(crate) fn new_transparent(
        device: &D3D12Device,
        vertex_shader: &D3D12Shader,
        pixel_shader: &D3D12Shader,
    ) -> Result<Self> {
        Self::new_mesh_pipeline(device, vertex_shader, pixel_shader, true)
    }

    fn new_mesh_pipeline(
        device: &D3D12Device,
        vertex_shader: &D3D12Shader,
        pixel_shader: &D3D12Shader,
        transparent: bool,
    ) -> Result<Self> {
        let texture_range = [D3D12_DESCRIPTOR_RANGE {
            RangeType: D3D12_DESCRIPTOR_RANGE_TYPE_SRV,

            NumDescriptors: 2,

            BaseShaderRegister: 0,
            RegisterSpace: 0,

            OffsetInDescriptorsFromTableStart: 0,
        }];

        let environment_range = [D3D12_DESCRIPTOR_RANGE {
            RangeType: D3D12_DESCRIPTOR_RANGE_TYPE_SRV,
            NumDescriptors: 3,
            BaseShaderRegister: 2,
            RegisterSpace: 0,
            OffsetInDescriptorsFromTableStart: 0,
        }];

        let shadow_range = [D3D12_DESCRIPTOR_RANGE {
            RangeType: D3D12_DESCRIPTOR_RANGE_TYPE_SRV,
            NumDescriptors: 3,
            BaseShaderRegister: 5,
            ..Default::default()
        }];
        let root_parameters = [
            D3D12_ROOT_PARAMETER {
                ParameterType: D3D12_ROOT_PARAMETER_TYPE_CBV,

                Anonymous: D3D12_ROOT_PARAMETER_0 {
                    Descriptor: D3D12_ROOT_DESCRIPTOR {
                        ShaderRegister: 0,
                        RegisterSpace: 0,
                    },
                },

                // ObjectConstants contains both transform data used by the
                // vertex shader and material data used by the pixel shader.
                ShaderVisibility: D3D12_SHADER_VISIBILITY_ALL,
            },
            D3D12_ROOT_PARAMETER {
                ParameterType: D3D12_ROOT_PARAMETER_TYPE_CBV,

                Anonymous: D3D12_ROOT_PARAMETER_0 {
                    Descriptor: D3D12_ROOT_DESCRIPTOR {
                        ShaderRegister: 1,
                        RegisterSpace: 0,
                    },
                },

                ShaderVisibility: D3D12_SHADER_VISIBILITY_ALL,
            },
            D3D12_ROOT_PARAMETER {
                ParameterType: D3D12_ROOT_PARAMETER_TYPE_CBV,

                Anonymous: D3D12_ROOT_PARAMETER_0 {
                    Descriptor: D3D12_ROOT_DESCRIPTOR {
                        ShaderRegister: 2,
                        RegisterSpace: 0,
                    },
                },

                ShaderVisibility: D3D12_SHADER_VISIBILITY_PIXEL,
            },
            D3D12_ROOT_PARAMETER {
                ParameterType: D3D12_ROOT_PARAMETER_TYPE_DESCRIPTOR_TABLE,

                Anonymous: D3D12_ROOT_PARAMETER_0 {
                    DescriptorTable: D3D12_ROOT_DESCRIPTOR_TABLE {
                        NumDescriptorRanges: texture_range.len() as u32,

                        pDescriptorRanges: texture_range.as_ptr(),
                    },
                },

                ShaderVisibility: D3D12_SHADER_VISIBILITY_PIXEL,
            },
            D3D12_ROOT_PARAMETER {
                ParameterType: D3D12_ROOT_PARAMETER_TYPE_DESCRIPTOR_TABLE,
                Anonymous: D3D12_ROOT_PARAMETER_0 {
                    DescriptorTable: D3D12_ROOT_DESCRIPTOR_TABLE {
                        NumDescriptorRanges: environment_range.len() as u32,
                        pDescriptorRanges: environment_range.as_ptr(),
                    },
                },
                ShaderVisibility: D3D12_SHADER_VISIBILITY_PIXEL,
            },
            D3D12_ROOT_PARAMETER {
                ParameterType: D3D12_ROOT_PARAMETER_TYPE_CBV,
                Anonymous: D3D12_ROOT_PARAMETER_0 {
                    Descriptor: D3D12_ROOT_DESCRIPTOR { ShaderRegister: 3, RegisterSpace: 0 },
                },
                ShaderVisibility: D3D12_SHADER_VISIBILITY_PIXEL,
            },
            D3D12_ROOT_PARAMETER {
                ParameterType: D3D12_ROOT_PARAMETER_TYPE_DESCRIPTOR_TABLE,
                Anonymous: D3D12_ROOT_PARAMETER_0 {
                    DescriptorTable: D3D12_ROOT_DESCRIPTOR_TABLE {
                        NumDescriptorRanges: 1, pDescriptorRanges: shadow_range.as_ptr(),
                    },
                },
                ShaderVisibility: D3D12_SHADER_VISIBILITY_PIXEL,
            },
        ];

        let static_samplers = [D3D12_STATIC_SAMPLER_DESC {
            Filter: D3D12_FILTER_MIN_MAG_MIP_LINEAR,

            AddressU: D3D12_TEXTURE_ADDRESS_MODE_WRAP,

            AddressV: D3D12_TEXTURE_ADDRESS_MODE_WRAP,

            AddressW: D3D12_TEXTURE_ADDRESS_MODE_WRAP,

            MipLODBias: 0.0,
            MaxAnisotropy: 1,

            ComparisonFunc: D3D12_COMPARISON_FUNC_ALWAYS,

            BorderColor: D3D12_STATIC_BORDER_COLOR_OPAQUE_BLACK,

            MinLOD: 0.0,
            MaxLOD: f32::MAX,

            ShaderRegister: 0,
            RegisterSpace: 0,

            ShaderVisibility: D3D12_SHADER_VISIBILITY_PIXEL,
        },
        D3D12_STATIC_SAMPLER_DESC {
            Filter: D3D12_FILTER_COMPARISON_MIN_MAG_LINEAR_MIP_POINT,
            AddressU: D3D12_TEXTURE_ADDRESS_MODE_BORDER,
            AddressV: D3D12_TEXTURE_ADDRESS_MODE_BORDER,
            AddressW: D3D12_TEXTURE_ADDRESS_MODE_BORDER,
            MaxAnisotropy: 1,
            ComparisonFunc: D3D12_COMPARISON_FUNC_LESS_EQUAL,
            BorderColor: D3D12_STATIC_BORDER_COLOR_OPAQUE_WHITE,
            MaxLOD: f32::MAX,
            ShaderRegister: 1,
            ShaderVisibility: D3D12_SHADER_VISIBILITY_PIXEL,
            ..Default::default()
        }];

        let root_desc = D3D12_ROOT_SIGNATURE_DESC {
            NumParameters: root_parameters.len() as u32,
            pParameters: root_parameters.as_ptr(),
            NumStaticSamplers: static_samplers.len() as u32,

            pStaticSamplers: static_samplers.as_ptr(),
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

        let signature_blob = signature_blob
            .expect("root signature serialization succeeded without producing a blob");

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
            BlendEnable: transparent.into(),
            LogicOpEnable: false.into(),
            SrcBlend: windows::Win32::Graphics::Direct3D12::D3D12_BLEND_SRC_ALPHA,
            DestBlend: windows::Win32::Graphics::Direct3D12::D3D12_BLEND_INV_SRC_ALPHA,
            BlendOp: windows::Win32::Graphics::Direct3D12::D3D12_BLEND_OP_ADD,
            SrcBlendAlpha: windows::Win32::Graphics::Direct3D12::D3D12_BLEND_ONE,
            DestBlendAlpha: windows::Win32::Graphics::Direct3D12::D3D12_BLEND_INV_SRC_ALPHA,
            BlendOpAlpha: windows::Win32::Graphics::Direct3D12::D3D12_BLEND_OP_ADD,
            LogicOp: Default::default(),
            RenderTargetWriteMask: D3D12_COLOR_WRITE_ENABLE_ALL.0 as u8,
        };

        let blend_state = D3D12_BLEND_DESC {
            AlphaToCoverageEnable: false.into(),
            IndependentBlendEnable: false.into(),
            RenderTarget: [render_target_blend; 8],
        };

        let depth_stencil = D3D12_DEPTH_STENCIL_DESC {
            DepthEnable: true.into(),
            DepthWriteMask: if transparent {
                windows::Win32::Graphics::Direct3D12::D3D12_DEPTH_WRITE_MASK_ZERO
            } else {
                D3D12_DEPTH_WRITE_MASK_ALL
            },
            DepthFunc: D3D12_COMPARISON_FUNC_LESS,
            StencilEnable: false.into(),
            StencilReadMask: 0,
            StencilWriteMask: 0,
            FrontFace: Default::default(),
            BackFace: Default::default(),
        };

        let mut rtv_formats = [DXGI_FORMAT_UNKNOWN; 8];
        rtv_formats[0] = DXGI_FORMAT_R16G16B16A16_FLOAT;

        let position_semantic = b"POSITION\0";
        let normal_semantic = b"NORMAL\0";
        let tex_coord_semantic = b"TEXCOORD\0";
        let tangent_semantic = b"TANGENT\0";

        let input_elements = [
            D3D12_INPUT_ELEMENT_DESC {
                SemanticName: windows::core::PCSTR(position_semantic.as_ptr()),
                SemanticIndex: 0,
                Format: DXGI_FORMAT_R32G32B32_FLOAT,
                InputSlot: 0,
                AlignedByteOffset: 0,
                InputSlotClass: D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA,
                InstanceDataStepRate: 0,
            },
            D3D12_INPUT_ELEMENT_DESC {
                SemanticName: windows::core::PCSTR(normal_semantic.as_ptr()),
                SemanticIndex: 0,
                Format: DXGI_FORMAT_R32G32B32_FLOAT,
                InputSlot: 0,
                AlignedByteOffset: 12,
                InputSlotClass: D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA,
                InstanceDataStepRate: 0,
            },
            D3D12_INPUT_ELEMENT_DESC {
                SemanticName: windows::core::PCSTR(tex_coord_semantic.as_ptr()),
                SemanticIndex: 0,
                Format: DXGI_FORMAT_R32G32_FLOAT,
                InputSlot: 0,
                AlignedByteOffset: 24,
                InputSlotClass: D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA,
                InstanceDataStepRate: 0,
            },
            D3D12_INPUT_ELEMENT_DESC {
                SemanticName: windows::core::PCSTR(tangent_semantic.as_ptr()),
                SemanticIndex: 0,
                Format: DXGI_FORMAT_R32G32B32A32_FLOAT,
                InputSlot: 0,
                AlignedByteOffset: 32,
                InputSlotClass: D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA,
                InstanceDataStepRate: 0,
            },
        ];

        let mut pipeline_desc = D3D12_GRAPHICS_PIPELINE_STATE_DESC {
            pRootSignature: std::mem::ManuallyDrop::new(Some(root_signature.clone())),

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

            PrimitiveTopologyType: D3D12_PRIMITIVE_TOPOLOGY_TYPE_TRIANGLE,

            NumRenderTargets: 1,
            RTVFormats: rtv_formats,

            DSVFormat: DXGI_FORMAT_D32_FLOAT,

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
            std::mem::ManuallyDrop::drop(&mut pipeline_desc.pRootSignature);
        }

        Ok(Self {
            root_signature,
            pipeline_state,
        })
    }

    pub(crate) fn new_shadow(device: &D3D12Device, vertex_shader: &D3D12Shader) -> Result<Self> {
        Self::new_shadow_pipeline(device, vertex_shader, None)
    }

    pub(crate) fn new_transmission(
        device: &D3D12Device,
        vertex_shader: &D3D12Shader,
        pixel_shader: &D3D12Shader,
    ) -> Result<Self> {
        Self::new_shadow_pipeline(device, vertex_shader, Some(pixel_shader))
    }

    fn new_shadow_pipeline(
        device: &D3D12Device,
        vertex_shader: &D3D12Shader,
        pixel_shader: Option<&D3D12Shader>,
    ) -> Result<Self> {
        let range = [D3D12_DESCRIPTOR_RANGE {
            RangeType: D3D12_DESCRIPTOR_RANGE_TYPE_SRV,
            NumDescriptors: 1,
            ..Default::default()
        }];
        let params = [D3D12_ROOT_PARAMETER {
            ParameterType: D3D12_ROOT_PARAMETER_TYPE_CBV,
            Anonymous: D3D12_ROOT_PARAMETER_0 {
                Descriptor: D3D12_ROOT_DESCRIPTOR { ShaderRegister: 0, RegisterSpace: 0 },
            },
            ShaderVisibility: D3D12_SHADER_VISIBILITY_ALL,
        }, D3D12_ROOT_PARAMETER {
            ParameterType: D3D12_ROOT_PARAMETER_TYPE_DESCRIPTOR_TABLE,
            Anonymous: D3D12_ROOT_PARAMETER_0 {
                DescriptorTable: D3D12_ROOT_DESCRIPTOR_TABLE {
                    NumDescriptorRanges: 1, pDescriptorRanges: range.as_ptr(),
                },
            },
            ShaderVisibility: D3D12_SHADER_VISIBILITY_PIXEL,
        }];
        let desc = D3D12_ROOT_SIGNATURE_DESC {
            NumParameters: params.len() as u32, pParameters: params.as_ptr(),
            Flags: D3D12_ROOT_SIGNATURE_FLAG_ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT,
            ..Default::default()
        };
        let mut blob = None;
        unsafe { D3D12SerializeRootSignature(&desc, D3D_ROOT_SIGNATURE_VERSION_1, &mut blob, None)?; }
        let blob = blob.unwrap();
        let root_signature: ID3D12RootSignature = unsafe {
            device.device().CreateRootSignature(0, std::slice::from_raw_parts(
                blob.GetBufferPointer() as *const u8, blob.GetBufferSize(),
            ))?
        };
        let input = [D3D12_INPUT_ELEMENT_DESC {
            SemanticName: windows::core::PCSTR(b"POSITION\0".as_ptr()),
            Format: DXGI_FORMAT_R32G32B32_FLOAT,
            InputSlotClass: D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA,
            ..Default::default()
        }];
        let mut formats = [DXGI_FORMAT_UNKNOWN; 8];
        if pixel_shader.is_some() {
            formats[0] = windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_R16_FLOAT;
        }
        let mut desc = D3D12_GRAPHICS_PIPELINE_STATE_DESC {
            pRootSignature: std::mem::ManuallyDrop::new(Some(root_signature.clone())),
            VS: D3D12_SHADER_BYTECODE {
                pShaderBytecode: vertex_shader.bytecode().as_ptr() as *const _,
                BytecodeLength: vertex_shader.bytecode().len(),
            },
            PS: pixel_shader.map(|shader| D3D12_SHADER_BYTECODE {
                pShaderBytecode: shader.bytecode().as_ptr() as *const _,
                BytecodeLength: shader.bytecode().len(),
            }).unwrap_or_default(),
            BlendState: D3D12_BLEND_DESC {
                RenderTarget: [D3D12_RENDER_TARGET_BLEND_DESC {
                    RenderTargetWriteMask: D3D12_COLOR_WRITE_ENABLE_ALL.0 as u8,
                    ..Default::default()
                }; 8],
                ..Default::default()
            },
            SampleMask: u32::MAX,
            RasterizerState: D3D12_RASTERIZER_DESC {
                FillMode: D3D12_FILL_MODE_SOLID,
                CullMode: D3D12_CULL_MODE_BACK,
                DepthClipEnable: true.into(),
                ..Default::default()
            },
            DepthStencilState: D3D12_DEPTH_STENCIL_DESC {
                DepthEnable: true.into(), DepthWriteMask: D3D12_DEPTH_WRITE_MASK_ALL,
                DepthFunc: D3D12_COMPARISON_FUNC_LESS,
                ..Default::default()
            },
            InputLayout: D3D12_INPUT_LAYOUT_DESC { pInputElementDescs: input.as_ptr(), NumElements: 1 },
            PrimitiveTopologyType: D3D12_PRIMITIVE_TOPOLOGY_TYPE_TRIANGLE,
            NumRenderTargets: pixel_shader.is_some() as u32,
            RTVFormats: formats,
            DSVFormat: DXGI_FORMAT_D32_FLOAT,
            SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
            ..Default::default()
        };
        let result = unsafe { device.device().CreateGraphicsPipelineState(&desc) };
        unsafe { std::mem::ManuallyDrop::drop(&mut desc.pRootSignature); }
        Ok(Self { root_signature, pipeline_state: result? })
    }

    pub(crate) fn new_fullscreen(
        device: &D3D12Device,
        vertex_shader: &D3D12Shader,
        pixel_shader: &D3D12Shader,
        output_format: DXGI_FORMAT,
    ) -> Result<Self> {
        let range = [D3D12_DESCRIPTOR_RANGE {
            RangeType: D3D12_DESCRIPTOR_RANGE_TYPE_SRV,
            NumDescriptors: 1,
            BaseShaderRegister: 0,
            RegisterSpace: 0,
            OffsetInDescriptorsFromTableStart: 0,
        }];
        let params = [
            D3D12_ROOT_PARAMETER {
                ParameterType: D3D12_ROOT_PARAMETER_TYPE_CBV,
                Anonymous: D3D12_ROOT_PARAMETER_0 {
                    Descriptor: D3D12_ROOT_DESCRIPTOR {
                        ShaderRegister: 0,
                        RegisterSpace: 0,
                    },
                },
                ShaderVisibility: D3D12_SHADER_VISIBILITY_ALL,
            },
            D3D12_ROOT_PARAMETER {
                ParameterType: D3D12_ROOT_PARAMETER_TYPE_DESCRIPTOR_TABLE,
                Anonymous: D3D12_ROOT_PARAMETER_0 {
                    DescriptorTable: D3D12_ROOT_DESCRIPTOR_TABLE {
                        NumDescriptorRanges: 1,
                        pDescriptorRanges: range.as_ptr(),
                    },
                },
                ShaderVisibility: D3D12_SHADER_VISIBILITY_PIXEL,
            },
        ];
        let sampler = [D3D12_STATIC_SAMPLER_DESC {
            Filter: D3D12_FILTER_MIN_MAG_MIP_LINEAR,
            AddressU: D3D12_TEXTURE_ADDRESS_MODE_WRAP,
            AddressV: windows::Win32::Graphics::Direct3D12::D3D12_TEXTURE_ADDRESS_MODE_CLAMP,
            AddressW: D3D12_TEXTURE_ADDRESS_MODE_WRAP,
            MipLODBias: 0.0,
            MaxAnisotropy: 1,
            ComparisonFunc: D3D12_COMPARISON_FUNC_ALWAYS,
            BorderColor: D3D12_STATIC_BORDER_COLOR_OPAQUE_BLACK,
            MinLOD: 0.0,
            MaxLOD: f32::MAX,
            ShaderRegister: 0,
            RegisterSpace: 0,
            ShaderVisibility: D3D12_SHADER_VISIBILITY_PIXEL,
        }];
        let desc = D3D12_ROOT_SIGNATURE_DESC {
            NumParameters: params.len() as u32,
            pParameters: params.as_ptr(),
            NumStaticSamplers: 1,
            pStaticSamplers: sampler.as_ptr(),
            Flags: D3D12_ROOT_SIGNATURE_FLAG_ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT,
        };
        let mut blob = None;
        unsafe {
            D3D12SerializeRootSignature(&desc, D3D_ROOT_SIGNATURE_VERSION_1, &mut blob, None)?;
        }
        let blob = blob.unwrap();
        let root_signature: ID3D12RootSignature = unsafe {
            device.device().CreateRootSignature(
                0,
                std::slice::from_raw_parts(
                    blob.GetBufferPointer() as *const u8,
                    blob.GetBufferSize(),
                ),
            )?
        };
        let mut rtv = [DXGI_FORMAT_UNKNOWN; 8];
        rtv[0] = output_format;
        let mut pipeline_desc = D3D12_GRAPHICS_PIPELINE_STATE_DESC {
            pRootSignature: std::mem::ManuallyDrop::new(Some(root_signature.clone())),
            VS: D3D12_SHADER_BYTECODE {
                pShaderBytecode: vertex_shader.bytecode().as_ptr() as *const _,
                BytecodeLength: vertex_shader.bytecode().len(),
            },
            PS: D3D12_SHADER_BYTECODE {
                pShaderBytecode: pixel_shader.bytecode().as_ptr() as *const _,
                BytecodeLength: pixel_shader.bytecode().len(),
            },
            BlendState: D3D12_BLEND_DESC {
                AlphaToCoverageEnable: false.into(),
                IndependentBlendEnable: false.into(),
                RenderTarget: [D3D12_RENDER_TARGET_BLEND_DESC {
                    RenderTargetWriteMask: D3D12_COLOR_WRITE_ENABLE_ALL.0 as u8,
                    ..Default::default()
                }; 8],
            },
            SampleMask: u32::MAX,
            RasterizerState: D3D12_RASTERIZER_DESC {
                FillMode: D3D12_FILL_MODE_SOLID,
                CullMode: windows::Win32::Graphics::Direct3D12::D3D12_CULL_MODE_NONE,
                DepthClipEnable: true.into(),
                ..Default::default()
            },
            DepthStencilState: D3D12_DEPTH_STENCIL_DESC {
                DepthEnable: false.into(),
                StencilEnable: false.into(),
                ..Default::default()
            },
            PrimitiveTopologyType: D3D12_PRIMITIVE_TOPOLOGY_TYPE_TRIANGLE,
            NumRenderTargets: 1,
            RTVFormats: rtv,
            SampleDesc: DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            },
            ..Default::default()
        };
        let pipeline_state = unsafe {
            device
                .device()
                .CreateGraphicsPipelineState(&pipeline_desc)?
        };
        unsafe {
            std::mem::ManuallyDrop::drop(&mut pipeline_desc.pRootSignature);
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
