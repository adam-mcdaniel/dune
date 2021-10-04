// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Direct3D capabilities include file
use ctypes::c_float;
use shared::d3d9types::D3DDEVTYPE;
use shared::guiddef::GUID;
use shared::minwindef::{DWORD, INT, UINT};
use um::winnt::ULONGLONG;
STRUCT!{struct D3DVSHADERCAPS2_0 {
    Caps: DWORD,
    DynamicFlowControlDepth: INT,
    NumTemps: INT,
    StaticFlowControlDepth: INT,
}}
pub const D3DVS20CAPS_PREDICATION: DWORD = 1 << 0;
pub const D3DVS20_MAX_DYNAMICFLOWCONTROLDEPTH: DWORD = 24;
pub const D3DVS20_MIN_DYNAMICFLOWCONTROLDEPTH: DWORD = 0;
pub const D3DVS20_MAX_NUMTEMPS: DWORD = 32;
pub const D3DVS20_MIN_NUMTEMPS: DWORD = 12;
pub const D3DVS20_MAX_STATICFLOWCONTROLDEPTH: DWORD = 4;
pub const D3DVS20_MIN_STATICFLOWCONTROLDEPTH: DWORD = 1;
STRUCT!{struct D3DPSHADERCAPS2_0 {
    Caps: DWORD,
    DynamicFlowControlDepth: INT,
    NumTemps: INT,
    StaticFlowControlDepth: INT,
    NumInstructionSlots: INT,
}}
pub const D3DPS20CAPS_ARBITRARYSWIZZLE: DWORD = 1 << 0;
pub const D3DPS20CAPS_GRADIENTINSTRUCTIONS: DWORD = 1 << 1;
pub const D3DPS20CAPS_PREDICATION: DWORD = 1 << 2;
pub const D3DPS20CAPS_NODEPENDENTREADLIMIT: DWORD = 1 << 3;
pub const D3DPS20CAPS_NOTEXINSTRUCTIONLIMIT: DWORD = 1 << 4;
pub const D3DPS20_MAX_DYNAMICFLOWCONTROLDEPTH: DWORD = 24;
pub const D3DPS20_MIN_DYNAMICFLOWCONTROLDEPTH: DWORD = 0;
pub const D3DPS20_MAX_NUMTEMPS: DWORD = 32;
pub const D3DPS20_MIN_NUMTEMPS: DWORD = 12;
pub const D3DPS20_MAX_STATICFLOWCONTROLDEPTH: DWORD = 4;
pub const D3DPS20_MIN_STATICFLOWCONTROLDEPTH: DWORD = 0;
pub const D3DPS20_MAX_NUMINSTRUCTIONSLOTS: DWORD = 512;
pub const D3DPS20_MIN_NUMINSTRUCTIONSLOTS: DWORD = 96;
pub const D3DMIN30SHADERINSTRUCTIONS: DWORD = 512;
pub const D3DMAX30SHADERINSTRUCTIONS: DWORD = 32768;
STRUCT!{struct D3DOVERLAYCAPS {
    Caps: UINT,
    MaxOverlayDisplayWidth: UINT,
    MaxOverlayDisplayHeight: UINT,
}}
pub const D3DOVERLAYCAPS_FULLRANGERGB: DWORD = 0x00000001;
pub const D3DOVERLAYCAPS_LIMITEDRANGERGB: DWORD = 0x00000002;
pub const D3DOVERLAYCAPS_YCbCr_BT601: DWORD = 0x00000004;
pub const D3DOVERLAYCAPS_YCbCr_BT709: DWORD = 0x00000008;
pub const D3DOVERLAYCAPS_YCbCr_BT601_xvYCC: DWORD = 0x00000010;
pub const D3DOVERLAYCAPS_YCbCr_BT709_xvYCC: DWORD = 0x00000020;
pub const D3DOVERLAYCAPS_STRETCHX: DWORD = 0x00000040;
pub const D3DOVERLAYCAPS_STRETCHY: DWORD = 0x00000080;
// FIXME packed(4)
STRUCT!{#[cfg_attr(target_arch = "x86", repr(packed))] struct D3DCONTENTPROTECTIONCAPS {
    Caps: DWORD,
    KeyExchangeType: GUID,
    BufferAlignmentStart: UINT,
    BlockAlignmentSize: UINT,
    ProtectedMemorySize: ULONGLONG,
}}
pub const D3DCPCAPS_SOFTWARE: DWORD = 0x00000001;
pub const D3DCPCAPS_HARDWARE: DWORD = 0x00000002;
pub const D3DCPCAPS_PROTECTIONALWAYSON: DWORD = 0x00000004;
pub const D3DCPCAPS_PARTIALDECRYPTION: DWORD = 0x00000008;
pub const D3DCPCAPS_CONTENTKEY: DWORD = 0x00000010;
pub const D3DCPCAPS_FRESHENSESSIONKEY: DWORD = 0x00000020;
pub const D3DCPCAPS_ENCRYPTEDREADBACK: DWORD = 0x00000040;
pub const D3DCPCAPS_ENCRYPTEDREADBACKKEY: DWORD = 0x00000080;
pub const D3DCPCAPS_SEQUENTIAL_CTR_IV: DWORD = 0x00000100;
pub const D3DCPCAPS_ENCRYPTSLICEDATAONLY: DWORD = 0x00000200;
DEFINE_GUID!{D3DCRYPTOTYPE_AES128_CTR,
    0x9b6bd711, 0x4f74, 0x41c9, 0x9e, 0x7b, 0x0b, 0xe2, 0xd7, 0xd9, 0x3b, 0x4f}
DEFINE_GUID!{D3DCRYPTOTYPE_PROPRIETARY,
    0xab4e9afd, 0x1d1c, 0x46e6, 0xa7, 0x2f, 0x08, 0x69, 0x91, 0x7b, 0x0d, 0xe8}
DEFINE_GUID!{D3DKEYEXCHANGE_RSAES_OAEP,
    0xc1949895, 0xd72a, 0x4a1d, 0x8e, 0x5d, 0xed, 0x85, 0x7d, 0x17, 0x15, 0x20}
DEFINE_GUID!{D3DKEYEXCHANGE_DXVA,
    0x43d3775c, 0x38e5, 0x4924, 0x8d, 0x86, 0xd3, 0xfc, 0xcf, 0x15, 0x3e, 0x9b}
STRUCT!{struct D3DCAPS9 {
    DeviceType: D3DDEVTYPE,
    AdapterOrdinal: UINT,
    Caps: DWORD,
    Caps2: DWORD,
    Caps3: DWORD,
    PresentationIntervals: DWORD,
    CursorCaps: DWORD,
    DevCaps: DWORD,
    PrimitiveMiscCaps: DWORD,
    RasterCaps: DWORD,
    ZCmpCaps: DWORD,
    SrcBlendCaps: DWORD,
    DestBlendCaps: DWORD,
    AlphaCmpCaps: DWORD,
    ShadeCaps: DWORD,
    TextureCaps: DWORD,
    TextureFilterCaps: DWORD,
    CubeTextureFilterCaps: DWORD,
    VolumeTextureFilterCaps: DWORD,
    TextureAddressCaps: DWORD,
    VolumeTextureAddressCaps: DWORD,
    LineCaps: DWORD,
    MaxTextureWidth: DWORD,
    MaxTextureHeight: DWORD,
    MaxVolumeExtent: DWORD,
    MaxTextureRepeat: DWORD,
    MaxTextureAspectRatio: DWORD,
    MaxAnisotropy: DWORD,
    MaxVertexW: c_float,
    GuardBandLeft: c_float,
    GuardBandTop: c_float,
    GuardBandRight: c_float,
    GuardBandBottom: c_float,
    ExtentsAdjust: c_float,
    StencilCaps: DWORD,
    FVFCaps: DWORD,
    TextureOpCaps: DWORD,
    MaxTextureBlendStages: DWORD,
    MaxSimultaneousTextures: DWORD,
    VertexProcessingCaps: DWORD,
    MaxActiveLights: DWORD,
    MaxUserClipPlanes: DWORD,
    MaxVertexBlendMatrices: DWORD,
    MaxVertexBlendMatrixIndex: DWORD,
    MaxPointSize: c_float,
    MaxPrimitiveCount: DWORD,
    MaxVertexIndex: DWORD,
    MaxStreams: DWORD,
    MaxStreamStride: DWORD,
    VertexShaderVersion: DWORD,
    MaxVertexShaderConst: DWORD,
    PixelShaderVersion: DWORD,
    PixelShader1xMaxValue: c_float,
    DevCaps2: DWORD,
    MaxNpatchTessellationLevel: c_float,
    Reserved5: DWORD,
    MasterAdapterOrdinal: UINT,
    AdapterOrdinalInGroup: UINT,
    NumberOfAdaptersInGroup: UINT,
    DeclTypes: DWORD,
    NumSimultaneousRTs: DWORD,
    StretchRectFilterCaps: DWORD,
    VS20Caps: D3DVSHADERCAPS2_0,
    PS20Caps: D3DPSHADERCAPS2_0,
    VertexTextureFilterCaps: DWORD,
    MaxVShaderInstructionsExecuted: DWORD,
    MaxPShaderInstructionsExecuted: DWORD,
    MaxVertexShader30InstructionSlots: DWORD,
    MaxPixelShader30InstructionSlots: DWORD,
}}
pub const D3DCAPS_OVERLAY: DWORD = 0x00000800;
pub const D3DCAPS_READ_SCANLINE: DWORD = 0x00020000;
pub const D3DCAPS2_FULLSCREENGAMMA: DWORD = 0x00020000;
pub const D3DCAPS2_CANCALIBRATEGAMMA: DWORD = 0x00100000;
pub const D3DCAPS2_RESERVED: DWORD = 0x02000000;
pub const D3DCAPS2_CANMANAGERESOURCE: DWORD = 0x10000000;
pub const D3DCAPS2_DYNAMICTEXTURES: DWORD = 0x20000000;
pub const D3DCAPS2_CANAUTOGENMIPMAP: DWORD = 0x40000000;
pub const D3DCAPS2_CANSHARERESOURCE: DWORD = 0x80000000;
pub const D3DCAPS3_RESERVED: DWORD = 0x8000001f;
pub const D3DCAPS3_ALPHA_FULLSCREEN_FLIP_OR_DISCARD: DWORD = 0x00000020;
pub const D3DCAPS3_LINEAR_TO_SRGB_PRESENTATION: DWORD = 0x00000080;
pub const D3DCAPS3_COPY_TO_VIDMEM: DWORD = 0x00000100;
pub const D3DCAPS3_COPY_TO_SYSTEMMEM: DWORD = 0x00000200;
pub const D3DCAPS3_DXVAHD: DWORD = 0x00000400;
pub const D3DCAPS3_DXVAHD_LIMITED: DWORD = 0x00000800;
pub const D3DPRESENT_INTERVAL_DEFAULT: DWORD = 0x00000000;
pub const D3DPRESENT_INTERVAL_ONE: DWORD = 0x00000001;
pub const D3DPRESENT_INTERVAL_TWO: DWORD = 0x00000002;
pub const D3DPRESENT_INTERVAL_THREE: DWORD = 0x00000004;
pub const D3DPRESENT_INTERVAL_FOUR: DWORD = 0x00000008;
pub const D3DPRESENT_INTERVAL_IMMEDIATE: DWORD = 0x80000000;
pub const D3DCURSORCAPS_COLOR: DWORD = 0x00000001;
pub const D3DCURSORCAPS_LOWRES: DWORD = 0x00000002;
pub const D3DDEVCAPS_EXECUTESYSTEMMEMORY: DWORD = 0x00000010;
pub const D3DDEVCAPS_EXECUTEVIDEOMEMORY: DWORD = 0x00000020;
pub const D3DDEVCAPS_TLVERTEXSYSTEMMEMORY: DWORD = 0x00000040;
pub const D3DDEVCAPS_TLVERTEXVIDEOMEMORY: DWORD = 0x00000080;
pub const D3DDEVCAPS_TEXTURESYSTEMMEMORY: DWORD = 0x00000100;
pub const D3DDEVCAPS_TEXTUREVIDEOMEMORY: DWORD = 0x00000200;
pub const D3DDEVCAPS_DRAWPRIMTLVERTEX: DWORD = 0x00000400;
pub const D3DDEVCAPS_CANRENDERAFTERFLIP: DWORD = 0x00000800;
pub const D3DDEVCAPS_TEXTURENONLOCALVIDMEM: DWORD = 0x00001000;
pub const D3DDEVCAPS_DRAWPRIMITIVES2: DWORD = 0x00002000;
pub const D3DDEVCAPS_SEPARATETEXTUREMEMORIES: DWORD = 0x00004000;
pub const D3DDEVCAPS_DRAWPRIMITIVES2EX: DWORD = 0x00008000;
pub const D3DDEVCAPS_HWTRANSFORMANDLIGHT: DWORD = 0x00010000;
pub const D3DDEVCAPS_CANBLTSYSTONONLOCAL: DWORD = 0x00020000;
pub const D3DDEVCAPS_HWRASTERIZATION: DWORD = 0x00080000;
pub const D3DDEVCAPS_PUREDEVICE: DWORD = 0x00100000;
pub const D3DDEVCAPS_QUINTICRTPATCHES: DWORD = 0x00200000;
pub const D3DDEVCAPS_RTPATCHES: DWORD = 0x00400000;
pub const D3DDEVCAPS_RTPATCHHANDLEZERO: DWORD = 0x00800000;
pub const D3DDEVCAPS_NPATCHES: DWORD = 0x01000000;
pub const D3DPMISCCAPS_MASKZ: DWORD = 0x00000002;
pub const D3DPMISCCAPS_CULLNONE: DWORD = 0x00000010;
pub const D3DPMISCCAPS_CULLCW: DWORD = 0x00000020;
pub const D3DPMISCCAPS_CULLCCW: DWORD = 0x00000040;
pub const D3DPMISCCAPS_COLORWRITEENABLE: DWORD = 0x00000080;
pub const D3DPMISCCAPS_CLIPPLANESCALEDPOINTS: DWORD = 0x00000100;
pub const D3DPMISCCAPS_CLIPTLVERTS: DWORD = 0x00000200;
pub const D3DPMISCCAPS_TSSARGTEMP: DWORD = 0x00000400;
pub const D3DPMISCCAPS_BLENDOP: DWORD = 0x00000800;
pub const D3DPMISCCAPS_NULLREFERENCE: DWORD = 0x00001000;
pub const D3DPMISCCAPS_INDEPENDENTWRITEMASKS: DWORD = 0x00004000;
pub const D3DPMISCCAPS_PERSTAGECONSTANT: DWORD = 0x00008000;
pub const D3DPMISCCAPS_FOGANDSPECULARALPHA: DWORD = 0x00010000;
pub const D3DPMISCCAPS_SEPARATEALPHABLEND: DWORD = 0x00020000;
pub const D3DPMISCCAPS_MRTINDEPENDENTBITDEPTHS: DWORD = 0x00040000;
pub const D3DPMISCCAPS_MRTPOSTPIXELSHADERBLENDING: DWORD = 0x00080000;
pub const D3DPMISCCAPS_FOGVERTEXCLAMPED: DWORD = 0x00100000;
pub const D3DPMISCCAPS_POSTBLENDSRGBCONVERT: DWORD = 0x00200000;
pub const D3DLINECAPS_TEXTURE: DWORD = 0x00000001;
pub const D3DLINECAPS_ZTEST: DWORD = 0x00000002;
pub const D3DLINECAPS_BLEND: DWORD = 0x00000004;
pub const D3DLINECAPS_ALPHACMP: DWORD = 0x00000008;
pub const D3DLINECAPS_FOG: DWORD = 0x00000010;
pub const D3DLINECAPS_ANTIALIAS: DWORD = 0x00000020;
pub const D3DPRASTERCAPS_DITHER: DWORD = 0x00000001;
pub const D3DPRASTERCAPS_ZTEST: DWORD = 0x00000010;
pub const D3DPRASTERCAPS_FOGVERTEX: DWORD = 0x00000080;
pub const D3DPRASTERCAPS_FOGTABLE: DWORD = 0x00000100;
pub const D3DPRASTERCAPS_MIPMAPLODBIAS: DWORD = 0x00002000;
pub const D3DPRASTERCAPS_ZBUFFERLESSHSR: DWORD = 0x00008000;
pub const D3DPRASTERCAPS_FOGRANGE: DWORD = 0x00010000;
pub const D3DPRASTERCAPS_ANISOTROPY: DWORD = 0x00020000;
pub const D3DPRASTERCAPS_WBUFFER: DWORD = 0x00040000;
pub const D3DPRASTERCAPS_WFOG: DWORD = 0x00100000;
pub const D3DPRASTERCAPS_ZFOG: DWORD = 0x00200000;
pub const D3DPRASTERCAPS_COLORPERSPECTIVE: DWORD = 0x00400000;
pub const D3DPRASTERCAPS_SCISSORTEST: DWORD = 0x01000000;
pub const D3DPRASTERCAPS_SLOPESCALEDEPTHBIAS: DWORD = 0x02000000;
pub const D3DPRASTERCAPS_DEPTHBIAS: DWORD = 0x04000000;
pub const D3DPRASTERCAPS_MULTISAMPLE_TOGGLE: DWORD = 0x08000000;
pub const D3DPCMPCAPS_NEVER: DWORD = 0x00000001;
pub const D3DPCMPCAPS_LESS: DWORD = 0x00000002;
pub const D3DPCMPCAPS_EQUAL: DWORD = 0x00000004;
pub const D3DPCMPCAPS_LESSEQUAL: DWORD = 0x00000008;
pub const D3DPCMPCAPS_GREATER: DWORD = 0x00000010;
pub const D3DPCMPCAPS_NOTEQUAL: DWORD = 0x00000020;
pub const D3DPCMPCAPS_GREATEREQUAL: DWORD = 0x00000040;
pub const D3DPCMPCAPS_ALWAYS: DWORD = 0x00000080;
pub const D3DPBLENDCAPS_ZERO: DWORD = 0x00000001;
pub const D3DPBLENDCAPS_ONE: DWORD = 0x00000002;
pub const D3DPBLENDCAPS_SRCCOLOR: DWORD = 0x00000004;
pub const D3DPBLENDCAPS_INVSRCCOLOR: DWORD = 0x00000008;
pub const D3DPBLENDCAPS_SRCALPHA: DWORD = 0x00000010;
pub const D3DPBLENDCAPS_INVSRCALPHA: DWORD = 0x00000020;
pub const D3DPBLENDCAPS_DESTALPHA: DWORD = 0x00000040;
pub const D3DPBLENDCAPS_INVDESTALPHA: DWORD = 0x00000080;
pub const D3DPBLENDCAPS_DESTCOLOR: DWORD = 0x00000100;
pub const D3DPBLENDCAPS_INVDESTCOLOR: DWORD = 0x00000200;
pub const D3DPBLENDCAPS_SRCALPHASAT: DWORD = 0x00000400;
pub const D3DPBLENDCAPS_BOTHSRCALPHA: DWORD = 0x00000800;
pub const D3DPBLENDCAPS_BOTHINVSRCALPHA: DWORD = 0x00001000;
pub const D3DPBLENDCAPS_BLENDFACTOR: DWORD = 0x00002000;
pub const D3DPBLENDCAPS_SRCCOLOR2: DWORD = 0x00004000;
pub const D3DPBLENDCAPS_INVSRCCOLOR2: DWORD = 0x00008000;
pub const D3DPSHADECAPS_COLORGOURAUDRGB: DWORD = 0x00000008;
pub const D3DPSHADECAPS_SPECULARGOURAUDRGB: DWORD = 0x00000200;
pub const D3DPSHADECAPS_ALPHAGOURAUDBLEND: DWORD = 0x00004000;
pub const D3DPSHADECAPS_FOGGOURAUD: DWORD = 0x00080000;
pub const D3DPTEXTURECAPS_PERSPECTIVE: DWORD = 0x00000001;
pub const D3DPTEXTURECAPS_POW2: DWORD = 0x00000002;
pub const D3DPTEXTURECAPS_ALPHA: DWORD = 0x00000004;
pub const D3DPTEXTURECAPS_SQUAREONLY: DWORD = 0x00000020;
pub const D3DPTEXTURECAPS_TEXREPEATNOTSCALEDBYSIZE: DWORD = 0x00000040;
pub const D3DPTEXTURECAPS_ALPHAPALETTE: DWORD = 0x00000080;
pub const D3DPTEXTURECAPS_NONPOW2CONDITIONAL: DWORD = 0x00000100;
pub const D3DPTEXTURECAPS_PROJECTED: DWORD = 0x00000400;
pub const D3DPTEXTURECAPS_CUBEMAP: DWORD = 0x00000800;
pub const D3DPTEXTURECAPS_VOLUMEMAP: DWORD = 0x00002000;
pub const D3DPTEXTURECAPS_MIPMAP: DWORD = 0x00004000;
pub const D3DPTEXTURECAPS_MIPVOLUMEMAP: DWORD = 0x00008000;
pub const D3DPTEXTURECAPS_MIPCUBEMAP: DWORD = 0x00010000;
pub const D3DPTEXTURECAPS_CUBEMAP_POW2: DWORD = 0x00020000;
pub const D3DPTEXTURECAPS_VOLUMEMAP_POW2: DWORD = 0x00040000;
pub const D3DPTEXTURECAPS_NOPROJECTEDBUMPENV: DWORD = 0x00200000;
pub const D3DPTFILTERCAPS_MINFPOINT: DWORD = 0x00000100;
pub const D3DPTFILTERCAPS_MINFLINEAR: DWORD = 0x00000200;
pub const D3DPTFILTERCAPS_MINFANISOTROPIC: DWORD = 0x00000400;
pub const D3DPTFILTERCAPS_MINFPYRAMIDALQUAD: DWORD = 0x00000800;
pub const D3DPTFILTERCAPS_MINFGAUSSIANQUAD: DWORD = 0x00001000;
pub const D3DPTFILTERCAPS_MIPFPOINT: DWORD = 0x00010000;
pub const D3DPTFILTERCAPS_MIPFLINEAR: DWORD = 0x00020000;
pub const D3DPTFILTERCAPS_CONVOLUTIONMONO: DWORD = 0x00040000;
pub const D3DPTFILTERCAPS_MAGFPOINT: DWORD = 0x01000000;
pub const D3DPTFILTERCAPS_MAGFLINEAR: DWORD = 0x02000000;
pub const D3DPTFILTERCAPS_MAGFANISOTROPIC: DWORD = 0x04000000;
pub const D3DPTFILTERCAPS_MAGFPYRAMIDALQUAD: DWORD = 0x08000000;
pub const D3DPTFILTERCAPS_MAGFGAUSSIANQUAD: DWORD = 0x10000000;
pub const D3DPTADDRESSCAPS_WRAP: DWORD = 0x00000001;
pub const D3DPTADDRESSCAPS_MIRROR: DWORD = 0x00000002;
pub const D3DPTADDRESSCAPS_CLAMP: DWORD = 0x00000004;
pub const D3DPTADDRESSCAPS_BORDER: DWORD = 0x00000008;
pub const D3DPTADDRESSCAPS_INDEPENDENTUV: DWORD = 0x00000010;
pub const D3DPTADDRESSCAPS_MIRRORONCE: DWORD = 0x00000020;
pub const D3DSTENCILCAPS_KEEP: DWORD = 0x00000001;
pub const D3DSTENCILCAPS_ZERO: DWORD = 0x00000002;
pub const D3DSTENCILCAPS_REPLACE: DWORD = 0x00000004;
pub const D3DSTENCILCAPS_INCRSAT: DWORD = 0x00000008;
pub const D3DSTENCILCAPS_DECRSAT: DWORD = 0x00000010;
pub const D3DSTENCILCAPS_INVERT: DWORD = 0x00000020;
pub const D3DSTENCILCAPS_INCR: DWORD = 0x00000040;
pub const D3DSTENCILCAPS_DECR: DWORD = 0x00000080;
pub const D3DSTENCILCAPS_TWOSIDED: DWORD = 0x00000100;
pub const D3DTEXOPCAPS_DISABLE: DWORD = 0x00000001;
pub const D3DTEXOPCAPS_SELECTARG1: DWORD = 0x00000002;
pub const D3DTEXOPCAPS_SELECTARG2: DWORD = 0x00000004;
pub const D3DTEXOPCAPS_MODULATE: DWORD = 0x00000008;
pub const D3DTEXOPCAPS_MODULATE2X: DWORD = 0x00000010;
pub const D3DTEXOPCAPS_MODULATE4X: DWORD = 0x00000020;
pub const D3DTEXOPCAPS_ADD: DWORD = 0x00000040;
pub const D3DTEXOPCAPS_ADDSIGNED: DWORD = 0x00000080;
pub const D3DTEXOPCAPS_ADDSIGNED2X: DWORD = 0x00000100;
pub const D3DTEXOPCAPS_SUBTRACT: DWORD = 0x00000200;
pub const D3DTEXOPCAPS_ADDSMOOTH: DWORD = 0x00000400;
pub const D3DTEXOPCAPS_BLENDDIFFUSEALPHA: DWORD = 0x00000800;
pub const D3DTEXOPCAPS_BLENDTEXTUREALPHA: DWORD = 0x00001000;
pub const D3DTEXOPCAPS_BLENDFACTORALPHA: DWORD = 0x00002000;
pub const D3DTEXOPCAPS_BLENDTEXTUREALPHAPM: DWORD = 0x00004000;
pub const D3DTEXOPCAPS_BLENDCURRENTALPHA: DWORD = 0x00008000;
pub const D3DTEXOPCAPS_PREMODULATE: DWORD = 0x00010000;
pub const D3DTEXOPCAPS_MODULATEALPHA_ADDCOLOR: DWORD = 0x00020000;
pub const D3DTEXOPCAPS_MODULATECOLOR_ADDALPHA: DWORD = 0x00040000;
pub const D3DTEXOPCAPS_MODULATEINVALPHA_ADDCOLOR: DWORD = 0x00080000;
pub const D3DTEXOPCAPS_MODULATEINVCOLOR_ADDALPHA: DWORD = 0x00100000;
pub const D3DTEXOPCAPS_BUMPENVMAP: DWORD = 0x00200000;
pub const D3DTEXOPCAPS_BUMPENVMAPLUMINANCE: DWORD = 0x00400000;
pub const D3DTEXOPCAPS_DOTPRODUCT3: DWORD = 0x00800000;
pub const D3DTEXOPCAPS_MULTIPLYADD: DWORD = 0x01000000;
pub const D3DTEXOPCAPS_LERP: DWORD = 0x02000000;
pub const D3DFVFCAPS_TEXCOORDCOUNTMASK: DWORD = 0x0000ffff;
pub const D3DFVFCAPS_DONOTSTRIPELEMENTS: DWORD = 0x00080000;
pub const D3DFVFCAPS_PSIZE: DWORD = 0x00100000;
pub const D3DVTXPCAPS_TEXGEN: DWORD = 0x00000001;
pub const D3DVTXPCAPS_MATERIALSOURCE7: DWORD = 0x00000002;
pub const D3DVTXPCAPS_DIRECTIONALLIGHTS: DWORD = 0x00000008;
pub const D3DVTXPCAPS_POSITIONALLIGHTS: DWORD = 0x00000010;
pub const D3DVTXPCAPS_LOCALVIEWER: DWORD = 0x00000020;
pub const D3DVTXPCAPS_TWEENING: DWORD = 0x00000040;
pub const D3DVTXPCAPS_TEXGEN_SPHEREMAP: DWORD = 0x00000100;
pub const D3DVTXPCAPS_NO_TEXGEN_NONLOCALVIEWER: DWORD = 0x00000200;
pub const D3DDEVCAPS2_STREAMOFFSET: DWORD = 0x00000001;
pub const D3DDEVCAPS2_DMAPNPATCH: DWORD = 0x00000002;
pub const D3DDEVCAPS2_ADAPTIVETESSRTPATCH: DWORD = 0x00000004;
pub const D3DDEVCAPS2_ADAPTIVETESSNPATCH: DWORD = 0x00000008;
pub const D3DDEVCAPS2_CAN_STRETCHRECT_FROM_TEXTURES: DWORD = 0x00000010;
pub const D3DDEVCAPS2_PRESAMPLEDDMAPNPATCH: DWORD = 0x00000020;
pub const D3DDEVCAPS2_VERTEXELEMENTSCANSHARESTREAMOFFSET: DWORD = 0x00000040;
pub const D3DDTCAPS_UBYTE4: DWORD = 0x00000001;
pub const D3DDTCAPS_UBYTE4N: DWORD = 0x00000002;
pub const D3DDTCAPS_SHORT2N: DWORD = 0x00000004;
pub const D3DDTCAPS_SHORT4N: DWORD = 0x00000008;
pub const D3DDTCAPS_USHORT2N: DWORD = 0x00000010;
pub const D3DDTCAPS_USHORT4N: DWORD = 0x00000020;
pub const D3DDTCAPS_UDEC3: DWORD = 0x00000040;
pub const D3DDTCAPS_DEC3N: DWORD = 0x00000080;
pub const D3DDTCAPS_FLOAT16_2: DWORD = 0x00000100;
pub const D3DDTCAPS_FLOAT16_4: DWORD = 0x00000200;
