use windows::{
    core::Result,
    Win32::Graphics::{
        Direct3D12::{
            D3D12_HEAP_PROPERTIES,
            D3D12_HEAP_TYPE_DEFAULT,
            D3D12_HEAP_TYPE_UPLOAD,
            D3D12_PLACED_SUBRESOURCE_FOOTPRINT,
            D3D12_RESOURCE_BARRIER,
            D3D12_RESOURCE_BARRIER_0,
            D3D12_RESOURCE_BARRIER_ALL_SUBRESOURCES,
            D3D12_RESOURCE_BARRIER_FLAG_NONE,
            D3D12_RESOURCE_BARRIER_TYPE_TRANSITION,
            D3D12_RESOURCE_DESC,
            D3D12_RESOURCE_DIMENSION_BUFFER,
            D3D12_RESOURCE_DIMENSION_TEXTURE2D,
            D3D12_RESOURCE_FLAG_NONE,
            D3D12_RESOURCE_STATE_COPY_DEST,
            D3D12_RESOURCE_STATE_GENERIC_READ,
            D3D12_RESOURCE_STATE_PIXEL_SHADER_RESOURCE,
            D3D12_RESOURCE_TRANSITION_BARRIER,
            D3D12_TEXTURE_COPY_LOCATION,
            D3D12_TEXTURE_COPY_LOCATION_0,
            D3D12_TEXTURE_COPY_TYPE_PLACED_FOOTPRINT,
            D3D12_TEXTURE_COPY_TYPE_SUBRESOURCE_INDEX,
            D3D12_TEXTURE_LAYOUT_ROW_MAJOR,
            D3D12_TEXTURE_LAYOUT_UNKNOWN,
            ID3D12GraphicsCommandList,
            ID3D12Resource,
        },
        Dxgi::Common::{
            DXGI_FORMAT_R8G8B8A8_UNORM,
            DXGI_FORMAT_UNKNOWN,
            DXGI_SAMPLE_DESC,
        },
    },
};

use crate::graphics::Texture;

use super::D3D12Device;

pub(crate) struct D3D12Texture {
    resource: ID3D12Resource,
    _upload_resource: ID3D12Resource,

    descriptor_index: u32,
}

impl D3D12Texture {
    pub(crate) fn resource(&self) -> &ID3D12Resource {
        &self.resource
    }
}

impl D3D12Texture {
    pub(crate) fn new(
        device: &D3D12Device,
        command_list: &ID3D12GraphicsCommandList,
        texture: &Texture,
        descriptor_index: u32,
    ) -> Result<Self> {
        let texture_desc = D3D12_RESOURCE_DESC {
            Dimension: D3D12_RESOURCE_DIMENSION_TEXTURE2D,
            Alignment: 0,

            Width: texture.width() as u64,
            Height: texture.height(),

            DepthOrArraySize: 1,
            MipLevels: 1,

            Format: DXGI_FORMAT_R8G8B8A8_UNORM,

            SampleDesc: DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            },

            Layout: D3D12_TEXTURE_LAYOUT_UNKNOWN,
            Flags: D3D12_RESOURCE_FLAG_NONE,
        };

        let default_heap = D3D12_HEAP_PROPERTIES {
            Type: D3D12_HEAP_TYPE_DEFAULT,
            ..Default::default()
        };

        let mut resource: Option<ID3D12Resource> = None;

        unsafe {
            device.device().CreateCommittedResource(
                &default_heap,
                Default::default(),
                &texture_desc,
                D3D12_RESOURCE_STATE_COPY_DEST,
                None,
                &mut resource,
            )?;
        }

        let resource = resource.expect(
            "CreateCommittedResource succeeded but returned no texture"
        );

        let mut footprint =
            D3D12_PLACED_SUBRESOURCE_FOOTPRINT::default();

        let mut row_count = 0u32;
        let mut row_size = 0u64;
        let mut upload_size = 0u64;

        unsafe {
            device.device().GetCopyableFootprints(
                &texture_desc,
                0,
                1,
                0,
                Some(&mut footprint),
                Some(&mut row_count),
                Some(&mut row_size),
                Some(&mut upload_size),
            );
        }

        let upload_desc = D3D12_RESOURCE_DESC {
            Dimension: D3D12_RESOURCE_DIMENSION_BUFFER,
            Alignment: 0,

            Width: upload_size,
            Height: 1,

            DepthOrArraySize: 1,
            MipLevels: 1,

            Format: DXGI_FORMAT_UNKNOWN,

            SampleDesc: DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            },

            Layout: D3D12_TEXTURE_LAYOUT_ROW_MAJOR,
            Flags: D3D12_RESOURCE_FLAG_NONE,
        };

        let upload_heap = D3D12_HEAP_PROPERTIES {
            Type: D3D12_HEAP_TYPE_UPLOAD,
            ..Default::default()
        };

        let mut upload_resource:
            Option<ID3D12Resource> = None;

        unsafe {
            device.device().CreateCommittedResource(
                &upload_heap,
                Default::default(),
                &upload_desc,
                D3D12_RESOURCE_STATE_GENERIC_READ,
                None,
                &mut upload_resource,
            )?;
        }

        let upload_resource =
            upload_resource.expect(
                "CreateCommittedResource succeeded but returned no texture upload buffer"
            );

        let mut mapped =
            std::ptr::null_mut();

        unsafe {
            upload_resource.Map(
                0,
                None,
                Some(&mut mapped),
            )?;
        }

        let mapped = mapped as *mut u8;

        let source_row_pitch =
            texture.width() as usize * 4;

        let destination_row_pitch =
            footprint.Footprint.RowPitch as usize;

        let pixels = texture.pixels();

        for row in 0..texture.height() as usize {
            let source_offset =
                row * source_row_pitch;

            let destination_offset =
                footprint.Offset as usize
                + row * destination_row_pitch;

            unsafe {
                std::ptr::copy_nonoverlapping(
                    pixels
                        .as_ptr()
                        .add(source_offset),

                    mapped.add(
                        destination_offset
                    ),

                    source_row_pitch,
                );
            }
        }

        unsafe {
            upload_resource.Unmap(
                0,
                None,
            );
        }

        let source = D3D12_TEXTURE_COPY_LOCATION {
            pResource: std::mem::ManuallyDrop::new(
                Some(upload_resource.clone())
            ),

            Type:
                D3D12_TEXTURE_COPY_TYPE_PLACED_FOOTPRINT,

            Anonymous:
                D3D12_TEXTURE_COPY_LOCATION_0 {
                    PlacedFootprint: footprint,
                },
        };

        let destination =
            D3D12_TEXTURE_COPY_LOCATION {
                pResource:
                    std::mem::ManuallyDrop::new(
                        Some(resource.clone())
                    ),

                Type:
                    D3D12_TEXTURE_COPY_TYPE_SUBRESOURCE_INDEX,

                Anonymous:
                    D3D12_TEXTURE_COPY_LOCATION_0 {
                        SubresourceIndex: 0,
                    },
            };

        unsafe {
            command_list.CopyTextureRegion(
                &destination,
                0,
                0,
                0,
                &source,
                None,
            );
        }

        let barrier =
            D3D12_RESOURCE_BARRIER {
                Type:
                    D3D12_RESOURCE_BARRIER_TYPE_TRANSITION,

                Flags:
                    D3D12_RESOURCE_BARRIER_FLAG_NONE,

                Anonymous:
                    D3D12_RESOURCE_BARRIER_0 {
                        Transition:
                            std::mem::ManuallyDrop::new(
                                D3D12_RESOURCE_TRANSITION_BARRIER {
                                    pResource:
                                        std::mem::ManuallyDrop::new(
                                            Some(resource.clone())
                                        ),

                                    Subresource:
                                        D3D12_RESOURCE_BARRIER_ALL_SUBRESOURCES,

                                    StateBefore:
                                        D3D12_RESOURCE_STATE_COPY_DEST,

                                    StateAfter:
                                        D3D12_RESOURCE_STATE_PIXEL_SHADER_RESOURCE,
                                }
                            ),
                    },
            };

        unsafe {
            command_list.ResourceBarrier(
                &[barrier]
            );
        }

        Ok(Self {
            resource,
            _upload_resource: upload_resource,
            descriptor_index,
        })
    }

    pub(crate) fn descriptor_index(&self) -> u32 {
        self.descriptor_index
    }
}