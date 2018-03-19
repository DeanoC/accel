#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

use vector_types::*;

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum cudaError_t {
    Success = 0,
    MissingConfiguration = 1,
    MemoryAllocation = 2,
    InitializationError = 3,
    LaunchFailure = 4,
    PriorLaunchFailure = 5,
    LaunchTimeout = 6,
    LaunchOutOfResources = 7,
    InvalidDeviceFunction = 8,
    InvalidConfiguration = 9,
    InvalidDevice = 10,
    InvalidValue = 11,
    InvalidPitchValue = 12,
    InvalidSymbol = 13,
    MapBufferObjectFailed = 14,
    UnmapBufferObjectFailed = 15,
    InvalidHostPointer = 16,
    InvalidDevicePointer = 17,
    InvalidTexture = 18,
    InvalidTextureBinding = 19,
    InvalidChannelDescriptor = 20,
    InvalidMemcpyDirection = 21,
    AddressOfConstant = 22,
    TextureFetchFailed = 23,
    TextureNotBound = 24,
    SynchronizationError = 25,
    InvalidFilterSetting = 26,
    InvalidNormSetting = 27,
    MixedDeviceExecution = 28,
    CudartUnloading = 29,
    Unknown = 30,
    NotYetImplemented = 31,
    MemoryValueTooLarge = 32,
    InvalidResourceHandle = 33,
    NotReady = 34,
    InsufficientDriver = 35,
    SetOnActiveProcess = 36,
    InvalidSurface = 37,
    NoDevice = 38,
    ECCUncorrectable = 39,
    SharedObjectSymbolNotFound = 40,
    SharedObjectInitFailed = 41,
    UnsupportedLimit = 42,
    DuplicateVariableName = 43,
    DuplicateTextureName = 44,
    DuplicateSurfaceName = 45,
    DevicesUnavailable = 46,
    InvalidKernelImage = 47,
    NoKernelImageForDevice = 48,
    IncompatibleDriverContext = 49,
    PeerAccessAlreadyEnabled = 50,
    PeerAccessNotEnabled = 51,
    DeviceAlreadyInUse = 54,
    ProfilerDisabled = 55,
    ProfilerNotInitialized = 56,
    ProfilerAlreadyStarted = 57,
    ProfilerAlreadyStopped = 58,
    Assert = 59,
    TooManyPeers = 60,
    HostMemoryAlreadyRegistered = 61,
    HostMemoryNotRegistered = 62,
    OperatingSystem = 63,
    PeerAccessUnsupported = 64,
    LaunchMaxDepthExceeded = 65,
    LaunchFileScopedTex = 66,
    LaunchFileScopedSurf = 67,
    SyncDepthExceeded = 68,
    LaunchPendingCountExceeded = 69,
    NotPermitted = 70,
    NotSupported = 71,
    HardwareStackError = 72,
    IllegalInstruction = 73,
    MisalignedAddress = 74,
    InvalidAddressSpace = 75,
    InvalidPc = 76,
    IllegalAddress = 77,
    InvalidPtx = 78,
    InvalidGraphicsContext = 79,
    NvlinkUncorrectable = 80,
    JitCompilerNotFound = 81,
    CooperativeLaunchTooLarge = 82,
    StartupFailure = 127,
    ApiFailureBase = 10000,
}

pub const cudaHostAllocDefault: ::std::os::raw::c_uint = 0;
pub const cudaHostAllocPortable: ::std::os::raw::c_uint = 1;
pub const cudaHostAllocMapped: ::std::os::raw::c_uint = 2;
pub const cudaHostAllocWriteCombined: ::std::os::raw::c_uint = 4;
pub const cudaHostRegisterDefault: ::std::os::raw::c_uint = 0;
pub const cudaHostRegisterPortable: ::std::os::raw::c_uint = 1;
pub const cudaHostRegisterMapped: ::std::os::raw::c_uint = 2;
pub const cudaHostRegisterIoMemory: ::std::os::raw::c_uint = 4;
pub const cudaPeerAccessDefault: ::std::os::raw::c_uint = 0;
pub const cudaStreamDefault: ::std::os::raw::c_uint = 0;
pub const cudaStreamNonBlocking: ::std::os::raw::c_uint = 1;
pub const cudaEventDefault: ::std::os::raw::c_uint = 0;
pub const cudaEventBlockingSync: ::std::os::raw::c_uint = 1;
pub const cudaEventDisableTiming: ::std::os::raw::c_uint = 2;
pub const cudaEventInterprocess: ::std::os::raw::c_uint = 4;
pub const cudaDeviceScheduleAuto: ::std::os::raw::c_uint = 0;
pub const cudaDeviceScheduleSpin: ::std::os::raw::c_uint = 1;
pub const cudaDeviceScheduleYield: ::std::os::raw::c_uint = 2;
pub const cudaDeviceScheduleBlockingSync: ::std::os::raw::c_uint = 4;
pub const cudaDeviceBlockingSync: ::std::os::raw::c_uint = 4;
pub const cudaDeviceScheduleMask: ::std::os::raw::c_uint = 7;
pub const cudaDeviceMapHost: ::std::os::raw::c_uint = 8;
pub const cudaDeviceLmemResizeToMax: ::std::os::raw::c_uint = 16;
pub const cudaDeviceMask: ::std::os::raw::c_uint = 31;
pub const cudaArrayDefault: ::std::os::raw::c_uint = 0;
pub const cudaArrayLayered: ::std::os::raw::c_uint = 1;
pub const cudaArraySurfaceLoadStore: ::std::os::raw::c_uint = 2;
pub const cudaArrayCubemap: ::std::os::raw::c_uint = 4;
pub const cudaArrayTextureGather: ::std::os::raw::c_uint = 8;
pub const cudaIpcMemLazyEnablePeerAccess: ::std::os::raw::c_uint = 1;
pub const cudaMemAttachGlobal: ::std::os::raw::c_uint = 1;
pub const cudaMemAttachHost: ::std::os::raw::c_uint = 2;
pub const cudaMemAttachSingle: ::std::os::raw::c_uint = 4;
pub const cudaOccupancyDefault: ::std::os::raw::c_uint = 0;
pub const cudaOccupancyDisableCachingOverride: ::std::os::raw::c_uint = 1;
pub const cudaCooperativeLaunchMultiDeviceNoPreSync: ::std::os::raw::c_uint = 1;
pub const cudaCooperativeLaunchMultiDeviceNoPostSync: ::std::os::raw::c_uint = 2;
pub const CUDA_IPC_HANDLE_SIZE: ::std::os::raw::c_uint = 64;
pub const cudaSurfaceType1D: ::std::os::raw::c_uint = 1;
pub const cudaSurfaceType2D: ::std::os::raw::c_uint = 2;
pub const cudaSurfaceType3D: ::std::os::raw::c_uint = 3;
pub const cudaSurfaceTypeCubemap: ::std::os::raw::c_uint = 12;
pub const cudaSurfaceType1DLayered: ::std::os::raw::c_uint = 241;
pub const cudaSurfaceType2DLayered: ::std::os::raw::c_uint = 242;
pub const cudaSurfaceTypeCubemapLayered: ::std::os::raw::c_uint = 252;
pub const cudaTextureType1D: ::std::os::raw::c_uint = 1;
pub const cudaTextureType2D: ::std::os::raw::c_uint = 2;
pub const cudaTextureType3D: ::std::os::raw::c_uint = 3;
pub const cudaTextureTypeCubemap: ::std::os::raw::c_uint = 12;
pub const cudaTextureType1DLayered: ::std::os::raw::c_uint = 241;
pub const cudaTextureType2DLayered: ::std::os::raw::c_uint = 242;
pub const cudaTextureTypeCubemapLayered: ::std::os::raw::c_uint = 252;
pub const CUDART_VERSION: ::std::os::raw::c_uint = 9000;
pub const cudaRoundMode_cudaRoundNearest: cudaRoundMode = 0;
pub const cudaRoundMode_cudaRoundZero: cudaRoundMode = 1;
pub const cudaRoundMode_cudaRoundPosInf: cudaRoundMode = 2;
pub const cudaRoundMode_cudaRoundMinInf: cudaRoundMode = 3;
pub type cudaRoundMode = ::std::os::raw::c_uint;
pub const cudaChannelFormatKind_cudaChannelFormatKindSigned: cudaChannelFormatKind = 0;
pub const cudaChannelFormatKind_cudaChannelFormatKindUnsigned: cudaChannelFormatKind = 1;
pub const cudaChannelFormatKind_cudaChannelFormatKindFloat: cudaChannelFormatKind = 2;
pub const cudaChannelFormatKind_cudaChannelFormatKindNone: cudaChannelFormatKind = 3;
pub type cudaChannelFormatKind = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudaChannelFormatDesc {
    pub x: ::std::os::raw::c_int,
    pub y: ::std::os::raw::c_int,
    pub z: ::std::os::raw::c_int,
    pub w: ::std::os::raw::c_int,
    pub f: cudaChannelFormatKind,
}
#[test]
fn bindgen_test_layout_cudaChannelFormatDesc() {
    assert_eq!(
        ::std::mem::size_of::<cudaChannelFormatDesc>(),
        20usize,
        concat!("Size of: ", stringify!(cudaChannelFormatDesc))
    );
    assert_eq!(
        ::std::mem::align_of::<cudaChannelFormatDesc>(),
        4usize,
        concat!("Alignment of ", stringify!(cudaChannelFormatDesc))
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaChannelFormatDesc)).x as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaChannelFormatDesc),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaChannelFormatDesc)).y as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaChannelFormatDesc),
            "::",
            stringify!(y)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaChannelFormatDesc)).z as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaChannelFormatDesc),
            "::",
            stringify!(z)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaChannelFormatDesc)).w as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaChannelFormatDesc),
            "::",
            stringify!(w)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaChannelFormatDesc)).f as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaChannelFormatDesc),
            "::",
            stringify!(f)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudaArray {
    _unused: [u8; 0],
}
pub type cudaArray_t = *mut cudaArray;
pub type cudaArray_const_t = *const cudaArray;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudaMipmappedArray {
    _unused: [u8; 0],
}
pub type cudaMipmappedArray_t = *mut cudaMipmappedArray;
pub type cudaMipmappedArray_const_t = *const cudaMipmappedArray;
pub const cudaMemoryType_cudaMemoryTypeHost: cudaMemoryType = 1;
pub const cudaMemoryType_cudaMemoryTypeDevice: cudaMemoryType = 2;
pub type cudaMemoryType = ::std::os::raw::c_uint;
pub const cudaMemcpyKind_cudaMemcpyHostToHost: cudaMemcpyKind = 0;
pub const cudaMemcpyKind_cudaMemcpyHostToDevice: cudaMemcpyKind = 1;
pub const cudaMemcpyKind_cudaMemcpyDeviceToHost: cudaMemcpyKind = 2;
pub const cudaMemcpyKind_cudaMemcpyDeviceToDevice: cudaMemcpyKind = 3;
pub const cudaMemcpyKind_cudaMemcpyDefault: cudaMemcpyKind = 4;
pub type cudaMemcpyKind = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudaPitchedPtr {
    pub ptr: *mut ::std::os::raw::c_void,
    pub pitch: usize,
    pub xsize: usize,
    pub ysize: usize,
}
#[test]
fn bindgen_test_layout_cudaPitchedPtr() {
    assert_eq!(
        ::std::mem::size_of::<cudaPitchedPtr>(),
        32usize,
        concat!("Size of: ", stringify!(cudaPitchedPtr))
    );
    assert_eq!(
        ::std::mem::align_of::<cudaPitchedPtr>(),
        8usize,
        concat!("Alignment of ", stringify!(cudaPitchedPtr))
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaPitchedPtr)).ptr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaPitchedPtr),
            "::",
            stringify!(ptr)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaPitchedPtr)).pitch as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaPitchedPtr),
            "::",
            stringify!(pitch)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaPitchedPtr)).xsize as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaPitchedPtr),
            "::",
            stringify!(xsize)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaPitchedPtr)).ysize as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaPitchedPtr),
            "::",
            stringify!(ysize)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudaExtent {
    pub width: usize,
    pub height: usize,
    pub depth: usize,
}
#[test]
fn bindgen_test_layout_cudaExtent() {
    assert_eq!(
        ::std::mem::size_of::<cudaExtent>(),
        24usize,
        concat!("Size of: ", stringify!(cudaExtent))
    );
    assert_eq!(
        ::std::mem::align_of::<cudaExtent>(),
        8usize,
        concat!("Alignment of ", stringify!(cudaExtent))
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaExtent)).width as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaExtent),
            "::",
            stringify!(width)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaExtent)).height as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaExtent),
            "::",
            stringify!(height)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaExtent)).depth as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaExtent),
            "::",
            stringify!(depth)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudaPos {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}
#[test]
fn bindgen_test_layout_cudaPos() {
    assert_eq!(
        ::std::mem::size_of::<cudaPos>(),
        24usize,
        concat!("Size of: ", stringify!(cudaPos))
    );
    assert_eq!(
        ::std::mem::align_of::<cudaPos>(),
        8usize,
        concat!("Alignment of ", stringify!(cudaPos))
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaPos)).x as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaPos),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaPos)).y as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaPos),
            "::",
            stringify!(y)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaPos)).z as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaPos),
            "::",
            stringify!(z)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudaMemcpy3DParms {
    pub srcArray: cudaArray_t,
    pub srcPos: cudaPos,
    pub srcPtr: cudaPitchedPtr,
    pub dstArray: cudaArray_t,
    pub dstPos: cudaPos,
    pub dstPtr: cudaPitchedPtr,
    pub extent: cudaExtent,
    pub kind: cudaMemcpyKind,
}
#[test]
fn bindgen_test_layout_cudaMemcpy3DParms() {
    assert_eq!(
        ::std::mem::size_of::<cudaMemcpy3DParms>(),
        160usize,
        concat!("Size of: ", stringify!(cudaMemcpy3DParms))
    );
    assert_eq!(
        ::std::mem::align_of::<cudaMemcpy3DParms>(),
        8usize,
        concat!("Alignment of ", stringify!(cudaMemcpy3DParms))
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaMemcpy3DParms)).srcArray as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaMemcpy3DParms),
            "::",
            stringify!(srcArray)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaMemcpy3DParms)).srcPos as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaMemcpy3DParms),
            "::",
            stringify!(srcPos)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaMemcpy3DParms)).srcPtr as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaMemcpy3DParms),
            "::",
            stringify!(srcPtr)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaMemcpy3DParms)).dstArray as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaMemcpy3DParms),
            "::",
            stringify!(dstArray)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaMemcpy3DParms)).dstPos as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaMemcpy3DParms),
            "::",
            stringify!(dstPos)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaMemcpy3DParms)).dstPtr as *const _ as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaMemcpy3DParms),
            "::",
            stringify!(dstPtr)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaMemcpy3DParms)).extent as *const _ as usize },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaMemcpy3DParms),
            "::",
            stringify!(extent)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaMemcpy3DParms)).kind as *const _ as usize },
        152usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaMemcpy3DParms),
            "::",
            stringify!(kind)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudaMemcpy3DPeerParms {
    pub srcArray: cudaArray_t,
    pub srcPos: cudaPos,
    pub srcPtr: cudaPitchedPtr,
    pub srcDevice: ::std::os::raw::c_int,
    pub dstArray: cudaArray_t,
    pub dstPos: cudaPos,
    pub dstPtr: cudaPitchedPtr,
    pub dstDevice: ::std::os::raw::c_int,
    pub extent: cudaExtent,
}
#[test]
fn bindgen_test_layout_cudaMemcpy3DPeerParms() {
    assert_eq!(
        ::std::mem::size_of::<cudaMemcpy3DPeerParms>(),
        168usize,
        concat!("Size of: ", stringify!(cudaMemcpy3DPeerParms))
    );
    assert_eq!(
        ::std::mem::align_of::<cudaMemcpy3DPeerParms>(),
        8usize,
        concat!("Alignment of ", stringify!(cudaMemcpy3DPeerParms))
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaMemcpy3DPeerParms)).srcArray as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaMemcpy3DPeerParms),
            "::",
            stringify!(srcArray)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaMemcpy3DPeerParms)).srcPos as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaMemcpy3DPeerParms),
            "::",
            stringify!(srcPos)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaMemcpy3DPeerParms)).srcPtr as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaMemcpy3DPeerParms),
            "::",
            stringify!(srcPtr)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaMemcpy3DPeerParms)).srcDevice as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaMemcpy3DPeerParms),
            "::",
            stringify!(srcDevice)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaMemcpy3DPeerParms)).dstArray as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaMemcpy3DPeerParms),
            "::",
            stringify!(dstArray)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaMemcpy3DPeerParms)).dstPos as *const _ as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaMemcpy3DPeerParms),
            "::",
            stringify!(dstPos)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaMemcpy3DPeerParms)).dstPtr as *const _ as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaMemcpy3DPeerParms),
            "::",
            stringify!(dstPtr)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaMemcpy3DPeerParms)).dstDevice as *const _ as usize },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaMemcpy3DPeerParms),
            "::",
            stringify!(dstDevice)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaMemcpy3DPeerParms)).extent as *const _ as usize },
        144usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaMemcpy3DPeerParms),
            "::",
            stringify!(extent)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudaGraphicsResource {
    _unused: [u8; 0],
}
pub const cudaGraphicsRegisterFlags_cudaGraphicsRegisterFlagsNone: cudaGraphicsRegisterFlags = 0;
pub const cudaGraphicsRegisterFlags_cudaGraphicsRegisterFlagsReadOnly: cudaGraphicsRegisterFlags = 1;
pub const cudaGraphicsRegisterFlags_cudaGraphicsRegisterFlagsWriteDiscard: cudaGraphicsRegisterFlags = 2;
pub const cudaGraphicsRegisterFlags_cudaGraphicsRegisterFlagsSurfaceLoadStore: cudaGraphicsRegisterFlags = 4;
pub const cudaGraphicsRegisterFlags_cudaGraphicsRegisterFlagsTextureGather: cudaGraphicsRegisterFlags = 8;
pub type cudaGraphicsRegisterFlags = ::std::os::raw::c_uint;
pub const cudaGraphicsMapFlags_cudaGraphicsMapFlagsNone: cudaGraphicsMapFlags = 0;
pub const cudaGraphicsMapFlags_cudaGraphicsMapFlagsReadOnly: cudaGraphicsMapFlags = 1;
pub const cudaGraphicsMapFlags_cudaGraphicsMapFlagsWriteDiscard: cudaGraphicsMapFlags = 2;
pub type cudaGraphicsMapFlags = ::std::os::raw::c_uint;
pub const cudaGraphicsCubeFace_cudaGraphicsCubeFacePositiveX: cudaGraphicsCubeFace = 0;
pub const cudaGraphicsCubeFace_cudaGraphicsCubeFaceNegativeX: cudaGraphicsCubeFace = 1;
pub const cudaGraphicsCubeFace_cudaGraphicsCubeFacePositiveY: cudaGraphicsCubeFace = 2;
pub const cudaGraphicsCubeFace_cudaGraphicsCubeFaceNegativeY: cudaGraphicsCubeFace = 3;
pub const cudaGraphicsCubeFace_cudaGraphicsCubeFacePositiveZ: cudaGraphicsCubeFace = 4;
pub const cudaGraphicsCubeFace_cudaGraphicsCubeFaceNegativeZ: cudaGraphicsCubeFace = 5;
pub type cudaGraphicsCubeFace = ::std::os::raw::c_uint;
pub const cudaResourceType_cudaResourceTypeArray: cudaResourceType = 0;
pub const cudaResourceType_cudaResourceTypeMipmappedArray: cudaResourceType = 1;
pub const cudaResourceType_cudaResourceTypeLinear: cudaResourceType = 2;
pub const cudaResourceType_cudaResourceTypePitch2D: cudaResourceType = 3;
pub type cudaResourceType = ::std::os::raw::c_uint;
pub const cudaResourceViewFormat_cudaResViewFormatNone: cudaResourceViewFormat = 0;
pub const cudaResourceViewFormat_cudaResViewFormatUnsignedChar1: cudaResourceViewFormat = 1;
pub const cudaResourceViewFormat_cudaResViewFormatUnsignedChar2: cudaResourceViewFormat = 2;
pub const cudaResourceViewFormat_cudaResViewFormatUnsignedChar4: cudaResourceViewFormat = 3;
pub const cudaResourceViewFormat_cudaResViewFormatSignedChar1: cudaResourceViewFormat = 4;
pub const cudaResourceViewFormat_cudaResViewFormatSignedChar2: cudaResourceViewFormat = 5;
pub const cudaResourceViewFormat_cudaResViewFormatSignedChar4: cudaResourceViewFormat = 6;
pub const cudaResourceViewFormat_cudaResViewFormatUnsignedShort1: cudaResourceViewFormat = 7;
pub const cudaResourceViewFormat_cudaResViewFormatUnsignedShort2: cudaResourceViewFormat = 8;
pub const cudaResourceViewFormat_cudaResViewFormatUnsignedShort4: cudaResourceViewFormat = 9;
pub const cudaResourceViewFormat_cudaResViewFormatSignedShort1: cudaResourceViewFormat = 10;
pub const cudaResourceViewFormat_cudaResViewFormatSignedShort2: cudaResourceViewFormat = 11;
pub const cudaResourceViewFormat_cudaResViewFormatSignedShort4: cudaResourceViewFormat = 12;
pub const cudaResourceViewFormat_cudaResViewFormatUnsignedInt1: cudaResourceViewFormat = 13;
pub const cudaResourceViewFormat_cudaResViewFormatUnsignedInt2: cudaResourceViewFormat = 14;
pub const cudaResourceViewFormat_cudaResViewFormatUnsignedInt4: cudaResourceViewFormat = 15;
pub const cudaResourceViewFormat_cudaResViewFormatSignedInt1: cudaResourceViewFormat = 16;
pub const cudaResourceViewFormat_cudaResViewFormatSignedInt2: cudaResourceViewFormat = 17;
pub const cudaResourceViewFormat_cudaResViewFormatSignedInt4: cudaResourceViewFormat = 18;
pub const cudaResourceViewFormat_cudaResViewFormatHalf1: cudaResourceViewFormat = 19;
pub const cudaResourceViewFormat_cudaResViewFormatHalf2: cudaResourceViewFormat = 20;
pub const cudaResourceViewFormat_cudaResViewFormatHalf4: cudaResourceViewFormat = 21;
pub const cudaResourceViewFormat_cudaResViewFormatFloat1: cudaResourceViewFormat = 22;
pub const cudaResourceViewFormat_cudaResViewFormatFloat2: cudaResourceViewFormat = 23;
pub const cudaResourceViewFormat_cudaResViewFormatFloat4: cudaResourceViewFormat = 24;
pub const cudaResourceViewFormat_cudaResViewFormatUnsignedBlockCompressed1: cudaResourceViewFormat = 25;
pub const cudaResourceViewFormat_cudaResViewFormatUnsignedBlockCompressed2: cudaResourceViewFormat = 26;
pub const cudaResourceViewFormat_cudaResViewFormatUnsignedBlockCompressed3: cudaResourceViewFormat = 27;
pub const cudaResourceViewFormat_cudaResViewFormatUnsignedBlockCompressed4: cudaResourceViewFormat = 28;
pub const cudaResourceViewFormat_cudaResViewFormatSignedBlockCompressed4: cudaResourceViewFormat = 29;
pub const cudaResourceViewFormat_cudaResViewFormatUnsignedBlockCompressed5: cudaResourceViewFormat = 30;
pub const cudaResourceViewFormat_cudaResViewFormatSignedBlockCompressed5: cudaResourceViewFormat = 31;
pub const cudaResourceViewFormat_cudaResViewFormatUnsignedBlockCompressed6H: cudaResourceViewFormat = 32;
pub const cudaResourceViewFormat_cudaResViewFormatSignedBlockCompressed6H: cudaResourceViewFormat = 33;
pub const cudaResourceViewFormat_cudaResViewFormatUnsignedBlockCompressed7: cudaResourceViewFormat = 34;
pub type cudaResourceViewFormat = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cudaResourceDesc {
    pub resType: cudaResourceType,
    pub res: cudaResourceDesc__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union cudaResourceDesc__bindgen_ty_1 {
    array: cudaResourceDesc__bindgen_ty_1__bindgen_ty_1,
    mipmap: cudaResourceDesc__bindgen_ty_1__bindgen_ty_2,
    linear: cudaResourceDesc__bindgen_ty_1__bindgen_ty_3,
    pitch2D: cudaResourceDesc__bindgen_ty_1__bindgen_ty_4,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudaResourceDesc__bindgen_ty_1__bindgen_ty_1 {
    pub array: cudaArray_t,
}
#[test]
fn bindgen_test_layout_cudaResourceDesc__bindgen_ty_1__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<cudaResourceDesc__bindgen_ty_1__bindgen_ty_1>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(cudaResourceDesc__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<cudaResourceDesc__bindgen_ty_1__bindgen_ty_1>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(cudaResourceDesc__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaResourceDesc__bindgen_ty_1__bindgen_ty_1)).array as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaResourceDesc__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(array)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudaResourceDesc__bindgen_ty_1__bindgen_ty_2 {
    pub mipmap: cudaMipmappedArray_t,
}
#[test]
fn bindgen_test_layout_cudaResourceDesc__bindgen_ty_1__bindgen_ty_2() {
    assert_eq!(
        ::std::mem::size_of::<cudaResourceDesc__bindgen_ty_1__bindgen_ty_2>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(cudaResourceDesc__bindgen_ty_1__bindgen_ty_2)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<cudaResourceDesc__bindgen_ty_1__bindgen_ty_2>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(cudaResourceDesc__bindgen_ty_1__bindgen_ty_2)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaResourceDesc__bindgen_ty_1__bindgen_ty_2)).mipmap as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaResourceDesc__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(mipmap)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudaResourceDesc__bindgen_ty_1__bindgen_ty_3 {
    pub devPtr: *mut ::std::os::raw::c_void,
    pub desc: cudaChannelFormatDesc,
    pub sizeInBytes: usize,
}
#[test]
fn bindgen_test_layout_cudaResourceDesc__bindgen_ty_1__bindgen_ty_3() {
    assert_eq!(
        ::std::mem::size_of::<cudaResourceDesc__bindgen_ty_1__bindgen_ty_3>(),
        40usize,
        concat!(
            "Size of: ",
            stringify!(cudaResourceDesc__bindgen_ty_1__bindgen_ty_3)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<cudaResourceDesc__bindgen_ty_1__bindgen_ty_3>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(cudaResourceDesc__bindgen_ty_1__bindgen_ty_3)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaResourceDesc__bindgen_ty_1__bindgen_ty_3)).devPtr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaResourceDesc__bindgen_ty_1__bindgen_ty_3),
            "::",
            stringify!(devPtr)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaResourceDesc__bindgen_ty_1__bindgen_ty_3)).desc as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaResourceDesc__bindgen_ty_1__bindgen_ty_3),
            "::",
            stringify!(desc)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaResourceDesc__bindgen_ty_1__bindgen_ty_3)).sizeInBytes as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaResourceDesc__bindgen_ty_1__bindgen_ty_3),
            "::",
            stringify!(sizeInBytes)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudaResourceDesc__bindgen_ty_1__bindgen_ty_4 {
    pub devPtr: *mut ::std::os::raw::c_void,
    pub desc: cudaChannelFormatDesc,
    pub width: usize,
    pub height: usize,
    pub pitchInBytes: usize,
}
#[test]
fn bindgen_test_layout_cudaResourceDesc__bindgen_ty_1__bindgen_ty_4() {
    assert_eq!(
        ::std::mem::size_of::<cudaResourceDesc__bindgen_ty_1__bindgen_ty_4>(),
        56usize,
        concat!(
            "Size of: ",
            stringify!(cudaResourceDesc__bindgen_ty_1__bindgen_ty_4)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<cudaResourceDesc__bindgen_ty_1__bindgen_ty_4>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(cudaResourceDesc__bindgen_ty_1__bindgen_ty_4)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaResourceDesc__bindgen_ty_1__bindgen_ty_4)).devPtr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaResourceDesc__bindgen_ty_1__bindgen_ty_4),
            "::",
            stringify!(devPtr)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaResourceDesc__bindgen_ty_1__bindgen_ty_4)).desc as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaResourceDesc__bindgen_ty_1__bindgen_ty_4),
            "::",
            stringify!(desc)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaResourceDesc__bindgen_ty_1__bindgen_ty_4)).width as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaResourceDesc__bindgen_ty_1__bindgen_ty_4),
            "::",
            stringify!(width)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaResourceDesc__bindgen_ty_1__bindgen_ty_4)).height as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaResourceDesc__bindgen_ty_1__bindgen_ty_4),
            "::",
            stringify!(height)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaResourceDesc__bindgen_ty_1__bindgen_ty_4)).pitchInBytes as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaResourceDesc__bindgen_ty_1__bindgen_ty_4),
            "::",
            stringify!(pitchInBytes)
        )
    );
}
#[test]
fn bindgen_test_layout_cudaResourceDesc__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<cudaResourceDesc__bindgen_ty_1>(),
        56usize,
        concat!("Size of: ", stringify!(cudaResourceDesc__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<cudaResourceDesc__bindgen_ty_1>(),
        8usize,
        concat!("Alignment of ", stringify!(cudaResourceDesc__bindgen_ty_1))
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaResourceDesc__bindgen_ty_1)).array as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaResourceDesc__bindgen_ty_1),
            "::",
            stringify!(array)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaResourceDesc__bindgen_ty_1)).mipmap as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaResourceDesc__bindgen_ty_1),
            "::",
            stringify!(mipmap)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaResourceDesc__bindgen_ty_1)).linear as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaResourceDesc__bindgen_ty_1),
            "::",
            stringify!(linear)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaResourceDesc__bindgen_ty_1)).pitch2D as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaResourceDesc__bindgen_ty_1),
            "::",
            stringify!(pitch2D)
        )
    );
}
#[test]
fn bindgen_test_layout_cudaResourceDesc() {
    assert_eq!(
        ::std::mem::size_of::<cudaResourceDesc>(),
        64usize,
        concat!("Size of: ", stringify!(cudaResourceDesc))
    );
    assert_eq!(
        ::std::mem::align_of::<cudaResourceDesc>(),
        8usize,
        concat!("Alignment of ", stringify!(cudaResourceDesc))
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaResourceDesc)).resType as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaResourceDesc),
            "::",
            stringify!(resType)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaResourceDesc)).res as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaResourceDesc),
            "::",
            stringify!(res)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudaResourceViewDesc {
    pub format: cudaResourceViewFormat,
    pub width: usize,
    pub height: usize,
    pub depth: usize,
    pub firstMipmapLevel: ::std::os::raw::c_uint,
    pub lastMipmapLevel: ::std::os::raw::c_uint,
    pub firstLayer: ::std::os::raw::c_uint,
    pub lastLayer: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_cudaResourceViewDesc() {
    assert_eq!(
        ::std::mem::size_of::<cudaResourceViewDesc>(),
        48usize,
        concat!("Size of: ", stringify!(cudaResourceViewDesc))
    );
    assert_eq!(
        ::std::mem::align_of::<cudaResourceViewDesc>(),
        8usize,
        concat!("Alignment of ", stringify!(cudaResourceViewDesc))
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaResourceViewDesc)).format as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaResourceViewDesc),
            "::",
            stringify!(format)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaResourceViewDesc)).width as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaResourceViewDesc),
            "::",
            stringify!(width)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaResourceViewDesc)).height as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaResourceViewDesc),
            "::",
            stringify!(height)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaResourceViewDesc)).depth as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaResourceViewDesc),
            "::",
            stringify!(depth)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaResourceViewDesc)).firstMipmapLevel as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaResourceViewDesc),
            "::",
            stringify!(firstMipmapLevel)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaResourceViewDesc)).lastMipmapLevel as *const _ as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaResourceViewDesc),
            "::",
            stringify!(lastMipmapLevel)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaResourceViewDesc)).firstLayer as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaResourceViewDesc),
            "::",
            stringify!(firstLayer)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaResourceViewDesc)).lastLayer as *const _ as usize },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaResourceViewDesc),
            "::",
            stringify!(lastLayer)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudaPointerAttributes {
    pub memoryType: cudaMemoryType,
    pub device: ::std::os::raw::c_int,
    pub devicePointer: *mut ::std::os::raw::c_void,
    pub hostPointer: *mut ::std::os::raw::c_void,
    pub isManaged: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_cudaPointerAttributes() {
    assert_eq!(
        ::std::mem::size_of::<cudaPointerAttributes>(),
        32usize,
        concat!("Size of: ", stringify!(cudaPointerAttributes))
    );
    assert_eq!(
        ::std::mem::align_of::<cudaPointerAttributes>(),
        8usize,
        concat!("Alignment of ", stringify!(cudaPointerAttributes))
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaPointerAttributes)).memoryType as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaPointerAttributes),
            "::",
            stringify!(memoryType)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaPointerAttributes)).device as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaPointerAttributes),
            "::",
            stringify!(device)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaPointerAttributes)).devicePointer as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaPointerAttributes),
            "::",
            stringify!(devicePointer)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaPointerAttributes)).hostPointer as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaPointerAttributes),
            "::",
            stringify!(hostPointer)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaPointerAttributes)).isManaged as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaPointerAttributes),
            "::",
            stringify!(isManaged)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudaFuncAttributes {
    pub sharedSizeBytes: usize,
    pub constSizeBytes: usize,
    pub localSizeBytes: usize,
    pub maxThreadsPerBlock: ::std::os::raw::c_int,
    pub numRegs: ::std::os::raw::c_int,
    pub ptxVersion: ::std::os::raw::c_int,
    pub binaryVersion: ::std::os::raw::c_int,
    pub cacheModeCA: ::std::os::raw::c_int,
    pub maxDynamicSharedSizeBytes: ::std::os::raw::c_int,
    pub preferredShmemCarveout: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_cudaFuncAttributes() {
    assert_eq!(
        ::std::mem::size_of::<cudaFuncAttributes>(),
        56usize,
        concat!("Size of: ", stringify!(cudaFuncAttributes))
    );
    assert_eq!(
        ::std::mem::align_of::<cudaFuncAttributes>(),
        8usize,
        concat!("Alignment of ", stringify!(cudaFuncAttributes))
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaFuncAttributes)).sharedSizeBytes as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaFuncAttributes),
            "::",
            stringify!(sharedSizeBytes)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaFuncAttributes)).constSizeBytes as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaFuncAttributes),
            "::",
            stringify!(constSizeBytes)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaFuncAttributes)).localSizeBytes as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaFuncAttributes),
            "::",
            stringify!(localSizeBytes)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaFuncAttributes)).maxThreadsPerBlock as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaFuncAttributes),
            "::",
            stringify!(maxThreadsPerBlock)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaFuncAttributes)).numRegs as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaFuncAttributes),
            "::",
            stringify!(numRegs)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaFuncAttributes)).ptxVersion as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaFuncAttributes),
            "::",
            stringify!(ptxVersion)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaFuncAttributes)).binaryVersion as *const _ as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaFuncAttributes),
            "::",
            stringify!(binaryVersion)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaFuncAttributes)).cacheModeCA as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaFuncAttributes),
            "::",
            stringify!(cacheModeCA)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaFuncAttributes)).maxDynamicSharedSizeBytes as *const _ as usize },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaFuncAttributes),
            "::",
            stringify!(maxDynamicSharedSizeBytes)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaFuncAttributes)).preferredShmemCarveout as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaFuncAttributes),
            "::",
            stringify!(preferredShmemCarveout)
        )
    );
}
pub const cudaFuncAttribute_cudaFuncAttributeMaxDynamicSharedMemorySize: cudaFuncAttribute = 8;
pub const cudaFuncAttribute_cudaFuncAttributePreferredSharedMemoryCarveout: cudaFuncAttribute = 9;
pub const cudaFuncAttribute_cudaFuncAttributeMax: cudaFuncAttribute = 10;
pub type cudaFuncAttribute = ::std::os::raw::c_uint;
pub const cudaFuncCache_cudaFuncCachePreferNone: cudaFuncCache = 0;
pub const cudaFuncCache_cudaFuncCachePreferShared: cudaFuncCache = 1;
pub const cudaFuncCache_cudaFuncCachePreferL1: cudaFuncCache = 2;
pub const cudaFuncCache_cudaFuncCachePreferEqual: cudaFuncCache = 3;
pub type cudaFuncCache = ::std::os::raw::c_uint;
pub const cudaSharedMemConfig_cudaSharedMemBankSizeDefault: cudaSharedMemConfig = 0;
pub const cudaSharedMemConfig_cudaSharedMemBankSizeFourByte: cudaSharedMemConfig = 1;
pub const cudaSharedMemConfig_cudaSharedMemBankSizeEightByte: cudaSharedMemConfig = 2;
pub type cudaSharedMemConfig = ::std::os::raw::c_uint;
pub const cudaSharedCarveout_cudaSharedmemCarveoutDefault: cudaSharedCarveout = -1;
pub const cudaSharedCarveout_cudaSharedmemCarveoutMaxShared: cudaSharedCarveout = 100;
pub const cudaSharedCarveout_cudaSharedmemCarveoutMaxL1: cudaSharedCarveout = 0;
pub type cudaSharedCarveout = ::std::os::raw::c_int;
pub const cudaComputeMode_cudaComputeModeDefault: cudaComputeMode = 0;
pub const cudaComputeMode_cudaComputeModeExclusive: cudaComputeMode = 1;
pub const cudaComputeMode_cudaComputeModeProhibited: cudaComputeMode = 2;
pub const cudaComputeMode_cudaComputeModeExclusiveProcess: cudaComputeMode = 3;
pub type cudaComputeMode = ::std::os::raw::c_uint;
pub const cudaLimit_cudaLimitStackSize: cudaLimit = 0;
pub const cudaLimit_cudaLimitPrintfFifoSize: cudaLimit = 1;
pub const cudaLimit_cudaLimitMallocHeapSize: cudaLimit = 2;
pub const cudaLimit_cudaLimitDevRuntimeSyncDepth: cudaLimit = 3;
pub const cudaLimit_cudaLimitDevRuntimePendingLaunchCount: cudaLimit = 4;
pub type cudaLimit = ::std::os::raw::c_uint;
pub const cudaMemoryAdvise_cudaMemAdviseSetReadMostly: cudaMemoryAdvise = 1;
pub const cudaMemoryAdvise_cudaMemAdviseUnsetReadMostly: cudaMemoryAdvise = 2;
pub const cudaMemoryAdvise_cudaMemAdviseSetPreferredLocation: cudaMemoryAdvise = 3;
pub const cudaMemoryAdvise_cudaMemAdviseUnsetPreferredLocation: cudaMemoryAdvise = 4;
pub const cudaMemoryAdvise_cudaMemAdviseSetAccessedBy: cudaMemoryAdvise = 5;
pub const cudaMemoryAdvise_cudaMemAdviseUnsetAccessedBy: cudaMemoryAdvise = 6;
pub type cudaMemoryAdvise = ::std::os::raw::c_uint;
pub const cudaMemRangeAttribute_cudaMemRangeAttributeReadMostly: cudaMemRangeAttribute = 1;
pub const cudaMemRangeAttribute_cudaMemRangeAttributePreferredLocation: cudaMemRangeAttribute = 2;
pub const cudaMemRangeAttribute_cudaMemRangeAttributeAccessedBy: cudaMemRangeAttribute = 3;
pub const cudaMemRangeAttribute_cudaMemRangeAttributeLastPrefetchLocation: cudaMemRangeAttribute = 4;
pub type cudaMemRangeAttribute = ::std::os::raw::c_uint;
pub const cudaOutputMode_cudaKeyValuePair: cudaOutputMode = 0;
pub const cudaOutputMode_cudaCSV: cudaOutputMode = 1;
pub type cudaOutputMode = ::std::os::raw::c_uint;
pub const cudaDeviceAttr_cudaDevAttrMaxThreadsPerBlock: cudaDeviceAttr = 1;
pub const cudaDeviceAttr_cudaDevAttrMaxBlockDimX: cudaDeviceAttr = 2;
pub const cudaDeviceAttr_cudaDevAttrMaxBlockDimY: cudaDeviceAttr = 3;
pub const cudaDeviceAttr_cudaDevAttrMaxBlockDimZ: cudaDeviceAttr = 4;
pub const cudaDeviceAttr_cudaDevAttrMaxGridDimX: cudaDeviceAttr = 5;
pub const cudaDeviceAttr_cudaDevAttrMaxGridDimY: cudaDeviceAttr = 6;
pub const cudaDeviceAttr_cudaDevAttrMaxGridDimZ: cudaDeviceAttr = 7;
pub const cudaDeviceAttr_cudaDevAttrMaxSharedMemoryPerBlock: cudaDeviceAttr = 8;
pub const cudaDeviceAttr_cudaDevAttrTotalConstantMemory: cudaDeviceAttr = 9;
pub const cudaDeviceAttr_cudaDevAttrWarpSize: cudaDeviceAttr = 10;
pub const cudaDeviceAttr_cudaDevAttrMaxPitch: cudaDeviceAttr = 11;
pub const cudaDeviceAttr_cudaDevAttrMaxRegistersPerBlock: cudaDeviceAttr = 12;
pub const cudaDeviceAttr_cudaDevAttrClockRate: cudaDeviceAttr = 13;
pub const cudaDeviceAttr_cudaDevAttrTextureAlignment: cudaDeviceAttr = 14;
pub const cudaDeviceAttr_cudaDevAttrGpuOverlap: cudaDeviceAttr = 15;
pub const cudaDeviceAttr_cudaDevAttrMultiProcessorCount: cudaDeviceAttr = 16;
pub const cudaDeviceAttr_cudaDevAttrKernelExecTimeout: cudaDeviceAttr = 17;
pub const cudaDeviceAttr_cudaDevAttrIntegrated: cudaDeviceAttr = 18;
pub const cudaDeviceAttr_cudaDevAttrCanMapHostMemory: cudaDeviceAttr = 19;
pub const cudaDeviceAttr_cudaDevAttrComputeMode: cudaDeviceAttr = 20;
pub const cudaDeviceAttr_cudaDevAttrMaxTexture1DWidth: cudaDeviceAttr = 21;
pub const cudaDeviceAttr_cudaDevAttrMaxTexture2DWidth: cudaDeviceAttr = 22;
pub const cudaDeviceAttr_cudaDevAttrMaxTexture2DHeight: cudaDeviceAttr = 23;
pub const cudaDeviceAttr_cudaDevAttrMaxTexture3DWidth: cudaDeviceAttr = 24;
pub const cudaDeviceAttr_cudaDevAttrMaxTexture3DHeight: cudaDeviceAttr = 25;
pub const cudaDeviceAttr_cudaDevAttrMaxTexture3DDepth: cudaDeviceAttr = 26;
pub const cudaDeviceAttr_cudaDevAttrMaxTexture2DLayeredWidth: cudaDeviceAttr = 27;
pub const cudaDeviceAttr_cudaDevAttrMaxTexture2DLayeredHeight: cudaDeviceAttr = 28;
pub const cudaDeviceAttr_cudaDevAttrMaxTexture2DLayeredLayers: cudaDeviceAttr = 29;
pub const cudaDeviceAttr_cudaDevAttrSurfaceAlignment: cudaDeviceAttr = 30;
pub const cudaDeviceAttr_cudaDevAttrConcurrentKernels: cudaDeviceAttr = 31;
pub const cudaDeviceAttr_cudaDevAttrEccEnabled: cudaDeviceAttr = 32;
pub const cudaDeviceAttr_cudaDevAttrPciBusId: cudaDeviceAttr = 33;
pub const cudaDeviceAttr_cudaDevAttrPciDeviceId: cudaDeviceAttr = 34;
pub const cudaDeviceAttr_cudaDevAttrTccDriver: cudaDeviceAttr = 35;
pub const cudaDeviceAttr_cudaDevAttrMemoryClockRate: cudaDeviceAttr = 36;
pub const cudaDeviceAttr_cudaDevAttrGlobalMemoryBusWidth: cudaDeviceAttr = 37;
pub const cudaDeviceAttr_cudaDevAttrL2CacheSize: cudaDeviceAttr = 38;
pub const cudaDeviceAttr_cudaDevAttrMaxThreadsPerMultiProcessor: cudaDeviceAttr = 39;
pub const cudaDeviceAttr_cudaDevAttrAsyncEngineCount: cudaDeviceAttr = 40;
pub const cudaDeviceAttr_cudaDevAttrUnifiedAddressing: cudaDeviceAttr = 41;
pub const cudaDeviceAttr_cudaDevAttrMaxTexture1DLayeredWidth: cudaDeviceAttr = 42;
pub const cudaDeviceAttr_cudaDevAttrMaxTexture1DLayeredLayers: cudaDeviceAttr = 43;
pub const cudaDeviceAttr_cudaDevAttrMaxTexture2DGatherWidth: cudaDeviceAttr = 45;
pub const cudaDeviceAttr_cudaDevAttrMaxTexture2DGatherHeight: cudaDeviceAttr = 46;
pub const cudaDeviceAttr_cudaDevAttrMaxTexture3DWidthAlt: cudaDeviceAttr = 47;
pub const cudaDeviceAttr_cudaDevAttrMaxTexture3DHeightAlt: cudaDeviceAttr = 48;
pub const cudaDeviceAttr_cudaDevAttrMaxTexture3DDepthAlt: cudaDeviceAttr = 49;
pub const cudaDeviceAttr_cudaDevAttrPciDomainId: cudaDeviceAttr = 50;
pub const cudaDeviceAttr_cudaDevAttrTexturePitchAlignment: cudaDeviceAttr = 51;
pub const cudaDeviceAttr_cudaDevAttrMaxTextureCubemapWidth: cudaDeviceAttr = 52;
pub const cudaDeviceAttr_cudaDevAttrMaxTextureCubemapLayeredWidth: cudaDeviceAttr = 53;
pub const cudaDeviceAttr_cudaDevAttrMaxTextureCubemapLayeredLayers: cudaDeviceAttr = 54;
pub const cudaDeviceAttr_cudaDevAttrMaxSurface1DWidth: cudaDeviceAttr = 55;
pub const cudaDeviceAttr_cudaDevAttrMaxSurface2DWidth: cudaDeviceAttr = 56;
pub const cudaDeviceAttr_cudaDevAttrMaxSurface2DHeight: cudaDeviceAttr = 57;
pub const cudaDeviceAttr_cudaDevAttrMaxSurface3DWidth: cudaDeviceAttr = 58;
pub const cudaDeviceAttr_cudaDevAttrMaxSurface3DHeight: cudaDeviceAttr = 59;
pub const cudaDeviceAttr_cudaDevAttrMaxSurface3DDepth: cudaDeviceAttr = 60;
pub const cudaDeviceAttr_cudaDevAttrMaxSurface1DLayeredWidth: cudaDeviceAttr = 61;
pub const cudaDeviceAttr_cudaDevAttrMaxSurface1DLayeredLayers: cudaDeviceAttr = 62;
pub const cudaDeviceAttr_cudaDevAttrMaxSurface2DLayeredWidth: cudaDeviceAttr = 63;
pub const cudaDeviceAttr_cudaDevAttrMaxSurface2DLayeredHeight: cudaDeviceAttr = 64;
pub const cudaDeviceAttr_cudaDevAttrMaxSurface2DLayeredLayers: cudaDeviceAttr = 65;
pub const cudaDeviceAttr_cudaDevAttrMaxSurfaceCubemapWidth: cudaDeviceAttr = 66;
pub const cudaDeviceAttr_cudaDevAttrMaxSurfaceCubemapLayeredWidth: cudaDeviceAttr = 67;
pub const cudaDeviceAttr_cudaDevAttrMaxSurfaceCubemapLayeredLayers: cudaDeviceAttr = 68;
pub const cudaDeviceAttr_cudaDevAttrMaxTexture1DLinearWidth: cudaDeviceAttr = 69;
pub const cudaDeviceAttr_cudaDevAttrMaxTexture2DLinearWidth: cudaDeviceAttr = 70;
pub const cudaDeviceAttr_cudaDevAttrMaxTexture2DLinearHeight: cudaDeviceAttr = 71;
pub const cudaDeviceAttr_cudaDevAttrMaxTexture2DLinearPitch: cudaDeviceAttr = 72;
pub const cudaDeviceAttr_cudaDevAttrMaxTexture2DMipmappedWidth: cudaDeviceAttr = 73;
pub const cudaDeviceAttr_cudaDevAttrMaxTexture2DMipmappedHeight: cudaDeviceAttr = 74;
pub const cudaDeviceAttr_cudaDevAttrComputeCapabilityMajor: cudaDeviceAttr = 75;
pub const cudaDeviceAttr_cudaDevAttrComputeCapabilityMinor: cudaDeviceAttr = 76;
pub const cudaDeviceAttr_cudaDevAttrMaxTexture1DMipmappedWidth: cudaDeviceAttr = 77;
pub const cudaDeviceAttr_cudaDevAttrStreamPrioritiesSupported: cudaDeviceAttr = 78;
pub const cudaDeviceAttr_cudaDevAttrGlobalL1CacheSupported: cudaDeviceAttr = 79;
pub const cudaDeviceAttr_cudaDevAttrLocalL1CacheSupported: cudaDeviceAttr = 80;
pub const cudaDeviceAttr_cudaDevAttrMaxSharedMemoryPerMultiprocessor: cudaDeviceAttr = 81;
pub const cudaDeviceAttr_cudaDevAttrMaxRegistersPerMultiprocessor: cudaDeviceAttr = 82;
pub const cudaDeviceAttr_cudaDevAttrManagedMemory: cudaDeviceAttr = 83;
pub const cudaDeviceAttr_cudaDevAttrIsMultiGpuBoard: cudaDeviceAttr = 84;
pub const cudaDeviceAttr_cudaDevAttrMultiGpuBoardGroupID: cudaDeviceAttr = 85;
pub const cudaDeviceAttr_cudaDevAttrHostNativeAtomicSupported: cudaDeviceAttr = 86;
pub const cudaDeviceAttr_cudaDevAttrSingleToDoublePrecisionPerfRatio: cudaDeviceAttr = 87;
pub const cudaDeviceAttr_cudaDevAttrPageableMemoryAccess: cudaDeviceAttr = 88;
pub const cudaDeviceAttr_cudaDevAttrConcurrentManagedAccess: cudaDeviceAttr = 89;
pub const cudaDeviceAttr_cudaDevAttrComputePreemptionSupported: cudaDeviceAttr = 90;
pub const cudaDeviceAttr_cudaDevAttrCanUseHostPointerForRegisteredMem: cudaDeviceAttr = 91;
pub const cudaDeviceAttr_cudaDevAttrReserved92: cudaDeviceAttr = 92;
pub const cudaDeviceAttr_cudaDevAttrReserved93: cudaDeviceAttr = 93;
pub const cudaDeviceAttr_cudaDevAttrReserved94: cudaDeviceAttr = 94;
pub const cudaDeviceAttr_cudaDevAttrCooperativeLaunch: cudaDeviceAttr = 95;
pub const cudaDeviceAttr_cudaDevAttrCooperativeMultiDeviceLaunch: cudaDeviceAttr = 96;
pub const cudaDeviceAttr_cudaDevAttrMaxSharedMemoryPerBlockOptin: cudaDeviceAttr = 97;
pub type cudaDeviceAttr = ::std::os::raw::c_uint;
pub const cudaDeviceP2PAttr_cudaDevP2PAttrPerformanceRank: cudaDeviceP2PAttr = 1;
pub const cudaDeviceP2PAttr_cudaDevP2PAttrAccessSupported: cudaDeviceP2PAttr = 2;
pub const cudaDeviceP2PAttr_cudaDevP2PAttrNativeAtomicSupported: cudaDeviceP2PAttr = 3;
pub type cudaDeviceP2PAttr = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cudaDeviceProp {
    pub name: [::std::os::raw::c_char; 256usize],
    pub totalGlobalMem: usize,
    pub sharedMemPerBlock: usize,
    pub regsPerBlock: ::std::os::raw::c_int,
    pub warpSize: ::std::os::raw::c_int,
    pub memPitch: usize,
    pub maxThreadsPerBlock: ::std::os::raw::c_int,
    pub maxThreadsDim: [::std::os::raw::c_int; 3usize],
    pub maxGridSize: [::std::os::raw::c_int; 3usize],
    pub clockRate: ::std::os::raw::c_int,
    pub totalConstMem: usize,
    pub major: ::std::os::raw::c_int,
    pub minor: ::std::os::raw::c_int,
    pub textureAlignment: usize,
    pub texturePitchAlignment: usize,
    pub deviceOverlap: ::std::os::raw::c_int,
    pub multiProcessorCount: ::std::os::raw::c_int,
    pub kernelExecTimeoutEnabled: ::std::os::raw::c_int,
    pub integrated: ::std::os::raw::c_int,
    pub canMapHostMemory: ::std::os::raw::c_int,
    pub computeMode: ::std::os::raw::c_int,
    pub maxTexture1D: ::std::os::raw::c_int,
    pub maxTexture1DMipmap: ::std::os::raw::c_int,
    pub maxTexture1DLinear: ::std::os::raw::c_int,
    pub maxTexture2D: [::std::os::raw::c_int; 2usize],
    pub maxTexture2DMipmap: [::std::os::raw::c_int; 2usize],
    pub maxTexture2DLinear: [::std::os::raw::c_int; 3usize],
    pub maxTexture2DGather: [::std::os::raw::c_int; 2usize],
    pub maxTexture3D: [::std::os::raw::c_int; 3usize],
    pub maxTexture3DAlt: [::std::os::raw::c_int; 3usize],
    pub maxTextureCubemap: ::std::os::raw::c_int,
    pub maxTexture1DLayered: [::std::os::raw::c_int; 2usize],
    pub maxTexture2DLayered: [::std::os::raw::c_int; 3usize],
    pub maxTextureCubemapLayered: [::std::os::raw::c_int; 2usize],
    pub maxSurface1D: ::std::os::raw::c_int,
    pub maxSurface2D: [::std::os::raw::c_int; 2usize],
    pub maxSurface3D: [::std::os::raw::c_int; 3usize],
    pub maxSurface1DLayered: [::std::os::raw::c_int; 2usize],
    pub maxSurface2DLayered: [::std::os::raw::c_int; 3usize],
    pub maxSurfaceCubemap: ::std::os::raw::c_int,
    pub maxSurfaceCubemapLayered: [::std::os::raw::c_int; 2usize],
    pub surfaceAlignment: usize,
    pub concurrentKernels: ::std::os::raw::c_int,
    pub ECCEnabled: ::std::os::raw::c_int,
    pub pciBusID: ::std::os::raw::c_int,
    pub pciDeviceID: ::std::os::raw::c_int,
    pub pciDomainID: ::std::os::raw::c_int,
    pub tccDriver: ::std::os::raw::c_int,
    pub asyncEngineCount: ::std::os::raw::c_int,
    pub unifiedAddressing: ::std::os::raw::c_int,
    pub memoryClockRate: ::std::os::raw::c_int,
    pub memoryBusWidth: ::std::os::raw::c_int,
    pub l2CacheSize: ::std::os::raw::c_int,
    pub maxThreadsPerMultiProcessor: ::std::os::raw::c_int,
    pub streamPrioritiesSupported: ::std::os::raw::c_int,
    pub globalL1CacheSupported: ::std::os::raw::c_int,
    pub localL1CacheSupported: ::std::os::raw::c_int,
    pub sharedMemPerMultiprocessor: usize,
    pub regsPerMultiprocessor: ::std::os::raw::c_int,
    pub managedMemory: ::std::os::raw::c_int,
    pub isMultiGpuBoard: ::std::os::raw::c_int,
    pub multiGpuBoardGroupID: ::std::os::raw::c_int,
    pub hostNativeAtomicSupported: ::std::os::raw::c_int,
    pub singleToDoublePrecisionPerfRatio: ::std::os::raw::c_int,
    pub pageableMemoryAccess: ::std::os::raw::c_int,
    pub concurrentManagedAccess: ::std::os::raw::c_int,
    pub computePreemptionSupported: ::std::os::raw::c_int,
    pub canUseHostPointerForRegisteredMem: ::std::os::raw::c_int,
    pub cooperativeLaunch: ::std::os::raw::c_int,
    pub cooperativeMultiDeviceLaunch: ::std::os::raw::c_int,
    pub sharedMemPerBlockOptin: usize,
}
#[test]
fn bindgen_test_layout_cudaDeviceProp() {
    assert_eq!(
        ::std::mem::size_of::<cudaDeviceProp>(),
        672usize,
        concat!("Size of: ", stringify!(cudaDeviceProp))
    );
    assert_eq!(
        ::std::mem::align_of::<cudaDeviceProp>(),
        8usize,
        concat!("Alignment of ", stringify!(cudaDeviceProp))
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).name as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).totalGlobalMem as *const _ as usize },
        256usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(totalGlobalMem)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).sharedMemPerBlock as *const _ as usize },
        264usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(sharedMemPerBlock)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).regsPerBlock as *const _ as usize },
        272usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(regsPerBlock)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).warpSize as *const _ as usize },
        276usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(warpSize)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).memPitch as *const _ as usize },
        280usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(memPitch)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).maxThreadsPerBlock as *const _ as usize },
        288usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(maxThreadsPerBlock)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).maxThreadsDim as *const _ as usize },
        292usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(maxThreadsDim)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).maxGridSize as *const _ as usize },
        304usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(maxGridSize)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).clockRate as *const _ as usize },
        316usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(clockRate)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).totalConstMem as *const _ as usize },
        320usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(totalConstMem)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).major as *const _ as usize },
        328usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(major)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).minor as *const _ as usize },
        332usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(minor)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).textureAlignment as *const _ as usize },
        336usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(textureAlignment)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).texturePitchAlignment as *const _ as usize },
        344usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(texturePitchAlignment)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).deviceOverlap as *const _ as usize },
        352usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(deviceOverlap)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).multiProcessorCount as *const _ as usize },
        356usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(multiProcessorCount)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).kernelExecTimeoutEnabled as *const _ as usize },
        360usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(kernelExecTimeoutEnabled)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).integrated as *const _ as usize },
        364usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(integrated)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).canMapHostMemory as *const _ as usize },
        368usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(canMapHostMemory)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).computeMode as *const _ as usize },
        372usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(computeMode)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).maxTexture1D as *const _ as usize },
        376usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(maxTexture1D)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).maxTexture1DMipmap as *const _ as usize },
        380usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(maxTexture1DMipmap)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).maxTexture1DLinear as *const _ as usize },
        384usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(maxTexture1DLinear)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).maxTexture2D as *const _ as usize },
        388usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(maxTexture2D)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).maxTexture2DMipmap as *const _ as usize },
        396usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(maxTexture2DMipmap)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).maxTexture2DLinear as *const _ as usize },
        404usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(maxTexture2DLinear)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).maxTexture2DGather as *const _ as usize },
        416usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(maxTexture2DGather)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).maxTexture3D as *const _ as usize },
        424usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(maxTexture3D)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).maxTexture3DAlt as *const _ as usize },
        436usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(maxTexture3DAlt)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).maxTextureCubemap as *const _ as usize },
        448usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(maxTextureCubemap)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).maxTexture1DLayered as *const _ as usize },
        452usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(maxTexture1DLayered)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).maxTexture2DLayered as *const _ as usize },
        460usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(maxTexture2DLayered)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).maxTextureCubemapLayered as *const _ as usize },
        472usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(maxTextureCubemapLayered)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).maxSurface1D as *const _ as usize },
        480usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(maxSurface1D)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).maxSurface2D as *const _ as usize },
        484usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(maxSurface2D)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).maxSurface3D as *const _ as usize },
        492usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(maxSurface3D)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).maxSurface1DLayered as *const _ as usize },
        504usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(maxSurface1DLayered)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).maxSurface2DLayered as *const _ as usize },
        512usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(maxSurface2DLayered)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).maxSurfaceCubemap as *const _ as usize },
        524usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(maxSurfaceCubemap)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).maxSurfaceCubemapLayered as *const _ as usize },
        528usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(maxSurfaceCubemapLayered)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).surfaceAlignment as *const _ as usize },
        536usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(surfaceAlignment)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).concurrentKernels as *const _ as usize },
        544usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(concurrentKernels)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).ECCEnabled as *const _ as usize },
        548usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(ECCEnabled)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).pciBusID as *const _ as usize },
        552usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(pciBusID)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).pciDeviceID as *const _ as usize },
        556usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(pciDeviceID)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).pciDomainID as *const _ as usize },
        560usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(pciDomainID)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).tccDriver as *const _ as usize },
        564usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(tccDriver)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).asyncEngineCount as *const _ as usize },
        568usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(asyncEngineCount)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).unifiedAddressing as *const _ as usize },
        572usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(unifiedAddressing)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).memoryClockRate as *const _ as usize },
        576usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(memoryClockRate)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).memoryBusWidth as *const _ as usize },
        580usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(memoryBusWidth)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).l2CacheSize as *const _ as usize },
        584usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(l2CacheSize)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).maxThreadsPerMultiProcessor as *const _ as usize },
        588usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(maxThreadsPerMultiProcessor)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).streamPrioritiesSupported as *const _ as usize },
        592usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(streamPrioritiesSupported)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).globalL1CacheSupported as *const _ as usize },
        596usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(globalL1CacheSupported)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).localL1CacheSupported as *const _ as usize },
        600usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(localL1CacheSupported)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).sharedMemPerMultiprocessor as *const _ as usize },
        608usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(sharedMemPerMultiprocessor)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).regsPerMultiprocessor as *const _ as usize },
        616usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(regsPerMultiprocessor)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).managedMemory as *const _ as usize },
        620usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(managedMemory)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).isMultiGpuBoard as *const _ as usize },
        624usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(isMultiGpuBoard)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).multiGpuBoardGroupID as *const _ as usize },
        628usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(multiGpuBoardGroupID)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).hostNativeAtomicSupported as *const _ as usize },
        632usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(hostNativeAtomicSupported)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).singleToDoublePrecisionPerfRatio as *const _ as usize },
        636usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(singleToDoublePrecisionPerfRatio)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).pageableMemoryAccess as *const _ as usize },
        640usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(pageableMemoryAccess)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).concurrentManagedAccess as *const _ as usize },
        644usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(concurrentManagedAccess)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).computePreemptionSupported as *const _ as usize },
        648usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(computePreemptionSupported)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).canUseHostPointerForRegisteredMem as *const _ as usize },
        652usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(canUseHostPointerForRegisteredMem)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).cooperativeLaunch as *const _ as usize },
        656usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(cooperativeLaunch)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).cooperativeMultiDeviceLaunch as *const _ as usize },
        660usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(cooperativeMultiDeviceLaunch)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaDeviceProp)).sharedMemPerBlockOptin as *const _ as usize },
        664usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaDeviceProp),
            "::",
            stringify!(sharedMemPerBlockOptin)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cudaIpcEventHandle_st {
    pub reserved: [::std::os::raw::c_char; 64usize],
}
#[test]
fn bindgen_test_layout_cudaIpcEventHandle_st() {
    assert_eq!(
        ::std::mem::size_of::<cudaIpcEventHandle_st>(),
        64usize,
        concat!("Size of: ", stringify!(cudaIpcEventHandle_st))
    );
    assert_eq!(
        ::std::mem::align_of::<cudaIpcEventHandle_st>(),
        1usize,
        concat!("Alignment of ", stringify!(cudaIpcEventHandle_st))
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaIpcEventHandle_st)).reserved as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaIpcEventHandle_st),
            "::",
            stringify!(reserved)
        )
    );
}
pub type cudaIpcEventHandle_t = cudaIpcEventHandle_st;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cudaIpcMemHandle_st {
    pub reserved: [::std::os::raw::c_char; 64usize],
}
#[test]
fn bindgen_test_layout_cudaIpcMemHandle_st() {
    assert_eq!(
        ::std::mem::size_of::<cudaIpcMemHandle_st>(),
        64usize,
        concat!("Size of: ", stringify!(cudaIpcMemHandle_st))
    );
    assert_eq!(
        ::std::mem::align_of::<cudaIpcMemHandle_st>(),
        1usize,
        concat!("Alignment of ", stringify!(cudaIpcMemHandle_st))
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaIpcMemHandle_st)).reserved as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaIpcMemHandle_st),
            "::",
            stringify!(reserved)
        )
    );
}
pub type cudaIpcMemHandle_t = cudaIpcMemHandle_st;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUstream_st {
    _unused: [u8; 0],
}
pub type cudaStream_t = *mut CUstream_st;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUevent_st {
    _unused: [u8; 0],
}
pub type cudaEvent_t = *mut CUevent_st;
pub type cudaGraphicsResource_t = *mut cudaGraphicsResource;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUuuid_st {
    _unused: [u8; 0],
}
pub type cudaUUID_t = CUuuid_st;
pub use self::cudaOutputMode as cudaOutputMode_t;
pub const cudaCGScope_cudaCGScopeInvalid: cudaCGScope = 0;
pub const cudaCGScope_cudaCGScopeGrid: cudaCGScope = 1;
pub const cudaCGScope_cudaCGScopeMultiGrid: cudaCGScope = 2;
pub type cudaCGScope = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudaLaunchParams {
    pub func: *mut ::std::os::raw::c_void,
    pub gridDim: dim3,
    pub blockDim: dim3,
    pub args: *mut *mut ::std::os::raw::c_void,
    pub sharedMem: usize,
    pub stream: cudaStream_t,
}
#[test]
fn bindgen_test_layout_cudaLaunchParams() {
    assert_eq!(
        ::std::mem::size_of::<cudaLaunchParams>(),
        56usize,
        concat!("Size of: ", stringify!(cudaLaunchParams))
    );
    assert_eq!(
        ::std::mem::align_of::<cudaLaunchParams>(),
        8usize,
        concat!("Alignment of ", stringify!(cudaLaunchParams))
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaLaunchParams)).func as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaLaunchParams),
            "::",
            stringify!(func)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaLaunchParams)).gridDim as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaLaunchParams),
            "::",
            stringify!(gridDim)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaLaunchParams)).blockDim as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaLaunchParams),
            "::",
            stringify!(blockDim)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaLaunchParams)).args as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaLaunchParams),
            "::",
            stringify!(args)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaLaunchParams)).sharedMem as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaLaunchParams),
            "::",
            stringify!(sharedMem)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaLaunchParams)).stream as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaLaunchParams),
            "::",
            stringify!(stream)
        )
    );
}
pub const cudaSurfaceBoundaryMode_cudaBoundaryModeZero: cudaSurfaceBoundaryMode = 0;
pub const cudaSurfaceBoundaryMode_cudaBoundaryModeClamp: cudaSurfaceBoundaryMode = 1;
pub const cudaSurfaceBoundaryMode_cudaBoundaryModeTrap: cudaSurfaceBoundaryMode = 2;
pub type cudaSurfaceBoundaryMode = ::std::os::raw::c_uint;
pub const cudaSurfaceFormatMode_cudaFormatModeForced: cudaSurfaceFormatMode = 0;
pub const cudaSurfaceFormatMode_cudaFormatModeAuto: cudaSurfaceFormatMode = 1;
pub type cudaSurfaceFormatMode = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct surfaceReference {
    pub channelDesc: cudaChannelFormatDesc,
}
#[test]
fn bindgen_test_layout_surfaceReference() {
    assert_eq!(
        ::std::mem::size_of::<surfaceReference>(),
        20usize,
        concat!("Size of: ", stringify!(surfaceReference))
    );
    assert_eq!(
        ::std::mem::align_of::<surfaceReference>(),
        4usize,
        concat!("Alignment of ", stringify!(surfaceReference))
    );
    assert_eq!(
        unsafe { &(*(0 as *const surfaceReference)).channelDesc as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(surfaceReference),
            "::",
            stringify!(channelDesc)
        )
    );
}
pub type cudaSurfaceObject_t = ::std::os::raw::c_ulonglong;
pub const cudaTextureAddressMode_cudaAddressModeWrap: cudaTextureAddressMode = 0;
pub const cudaTextureAddressMode_cudaAddressModeClamp: cudaTextureAddressMode = 1;
pub const cudaTextureAddressMode_cudaAddressModeMirror: cudaTextureAddressMode = 2;
pub const cudaTextureAddressMode_cudaAddressModeBorder: cudaTextureAddressMode = 3;
pub type cudaTextureAddressMode = ::std::os::raw::c_uint;
pub const cudaTextureFilterMode_cudaFilterModePoint: cudaTextureFilterMode = 0;
pub const cudaTextureFilterMode_cudaFilterModeLinear: cudaTextureFilterMode = 1;
pub type cudaTextureFilterMode = ::std::os::raw::c_uint;
pub const cudaTextureReadMode_cudaReadModeElementType: cudaTextureReadMode = 0;
pub const cudaTextureReadMode_cudaReadModeNormalizedFloat: cudaTextureReadMode = 1;
pub type cudaTextureReadMode = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct textureReference {
    pub normalized: ::std::os::raw::c_int,
    pub filterMode: cudaTextureFilterMode,
    pub addressMode: [cudaTextureAddressMode; 3usize],
    pub channelDesc: cudaChannelFormatDesc,
    pub sRGB: ::std::os::raw::c_int,
    pub maxAnisotropy: ::std::os::raw::c_uint,
    pub mipmapFilterMode: cudaTextureFilterMode,
    pub mipmapLevelBias: f32,
    pub minMipmapLevelClamp: f32,
    pub maxMipmapLevelClamp: f32,
    pub __cudaReserved: [::std::os::raw::c_int; 15usize],
}
#[test]
fn bindgen_test_layout_textureReference() {
    assert_eq!(
        ::std::mem::size_of::<textureReference>(),
        124usize,
        concat!("Size of: ", stringify!(textureReference))
    );
    assert_eq!(
        ::std::mem::align_of::<textureReference>(),
        4usize,
        concat!("Alignment of ", stringify!(textureReference))
    );
    assert_eq!(
        unsafe { &(*(0 as *const textureReference)).normalized as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(textureReference),
            "::",
            stringify!(normalized)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const textureReference)).filterMode as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(textureReference),
            "::",
            stringify!(filterMode)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const textureReference)).addressMode as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(textureReference),
            "::",
            stringify!(addressMode)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const textureReference)).channelDesc as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(textureReference),
            "::",
            stringify!(channelDesc)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const textureReference)).sRGB as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(textureReference),
            "::",
            stringify!(sRGB)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const textureReference)).maxAnisotropy as *const _ as usize },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(textureReference),
            "::",
            stringify!(maxAnisotropy)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const textureReference)).mipmapFilterMode as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(textureReference),
            "::",
            stringify!(mipmapFilterMode)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const textureReference)).mipmapLevelBias as *const _ as usize },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(textureReference),
            "::",
            stringify!(mipmapLevelBias)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const textureReference)).minMipmapLevelClamp as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(textureReference),
            "::",
            stringify!(minMipmapLevelClamp)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const textureReference)).maxMipmapLevelClamp as *const _ as usize },
        60usize,
        concat!(
            "Offset of field: ",
            stringify!(textureReference),
            "::",
            stringify!(maxMipmapLevelClamp)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const textureReference)).__cudaReserved as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(textureReference),
            "::",
            stringify!(__cudaReserved)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudaTextureDesc {
    pub addressMode: [cudaTextureAddressMode; 3usize],
    pub filterMode: cudaTextureFilterMode,
    pub readMode: cudaTextureReadMode,
    pub sRGB: ::std::os::raw::c_int,
    pub borderColor: [f32; 4usize],
    pub normalizedCoords: ::std::os::raw::c_int,
    pub maxAnisotropy: ::std::os::raw::c_uint,
    pub mipmapFilterMode: cudaTextureFilterMode,
    pub mipmapLevelBias: f32,
    pub minMipmapLevelClamp: f32,
    pub maxMipmapLevelClamp: f32,
}
#[test]
fn bindgen_test_layout_cudaTextureDesc() {
    assert_eq!(
        ::std::mem::size_of::<cudaTextureDesc>(),
        64usize,
        concat!("Size of: ", stringify!(cudaTextureDesc))
    );
    assert_eq!(
        ::std::mem::align_of::<cudaTextureDesc>(),
        4usize,
        concat!("Alignment of ", stringify!(cudaTextureDesc))
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaTextureDesc)).addressMode as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaTextureDesc),
            "::",
            stringify!(addressMode)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaTextureDesc)).filterMode as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaTextureDesc),
            "::",
            stringify!(filterMode)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaTextureDesc)).readMode as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaTextureDesc),
            "::",
            stringify!(readMode)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaTextureDesc)).sRGB as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaTextureDesc),
            "::",
            stringify!(sRGB)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaTextureDesc)).borderColor as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaTextureDesc),
            "::",
            stringify!(borderColor)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaTextureDesc)).normalizedCoords as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaTextureDesc),
            "::",
            stringify!(normalizedCoords)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaTextureDesc)).maxAnisotropy as *const _ as usize },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaTextureDesc),
            "::",
            stringify!(maxAnisotropy)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaTextureDesc)).mipmapFilterMode as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaTextureDesc),
            "::",
            stringify!(mipmapFilterMode)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaTextureDesc)).mipmapLevelBias as *const _ as usize },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaTextureDesc),
            "::",
            stringify!(mipmapLevelBias)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaTextureDesc)).minMipmapLevelClamp as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaTextureDesc),
            "::",
            stringify!(minMipmapLevelClamp)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const cudaTextureDesc)).maxMipmapLevelClamp as *const _ as usize },
        60usize,
        concat!(
            "Offset of field: ",
            stringify!(cudaTextureDesc),
            "::",
            stringify!(maxMipmapLevelClamp)
        )
    );
}
pub type cudaTextureObject_t = ::std::os::raw::c_ulonglong;
pub const cudaDataType_t_CUDA_R_16F: cudaDataType_t = 2;
pub const cudaDataType_t_CUDA_C_16F: cudaDataType_t = 6;
pub const cudaDataType_t_CUDA_R_32F: cudaDataType_t = 0;
pub const cudaDataType_t_CUDA_C_32F: cudaDataType_t = 4;
pub const cudaDataType_t_CUDA_R_64F: cudaDataType_t = 1;
pub const cudaDataType_t_CUDA_C_64F: cudaDataType_t = 5;
pub const cudaDataType_t_CUDA_R_8I: cudaDataType_t = 3;
pub const cudaDataType_t_CUDA_C_8I: cudaDataType_t = 7;
pub const cudaDataType_t_CUDA_R_8U: cudaDataType_t = 8;
pub const cudaDataType_t_CUDA_C_8U: cudaDataType_t = 9;
pub const cudaDataType_t_CUDA_R_32I: cudaDataType_t = 10;
pub const cudaDataType_t_CUDA_C_32I: cudaDataType_t = 11;
pub const cudaDataType_t_CUDA_R_32U: cudaDataType_t = 12;
pub const cudaDataType_t_CUDA_C_32U: cudaDataType_t = 13;
pub type cudaDataType_t = ::std::os::raw::c_uint;
pub use self::cudaDataType_t as cudaDataType;
pub const libraryPropertyType_t_MAJOR_VERSION: libraryPropertyType_t = 0;
pub const libraryPropertyType_t_MINOR_VERSION: libraryPropertyType_t = 1;
pub const libraryPropertyType_t_PATCH_LEVEL: libraryPropertyType_t = 2;
pub type libraryPropertyType_t = ::std::os::raw::c_uint;
pub use self::libraryPropertyType_t as libraryPropertyType;
extern "C" {
    pub fn cudaDeviceReset() -> cudaError_t;
}
extern "C" {
    pub fn cudaDeviceSynchronize() -> cudaError_t;
}
extern "C" {
    pub fn cudaDeviceSetLimit(limit: cudaLimit, value: usize) -> cudaError_t;
}
extern "C" {
    pub fn cudaDeviceGetLimit(pValue: *mut usize, limit: cudaLimit) -> cudaError_t;
}
extern "C" {
    pub fn cudaDeviceGetCacheConfig(pCacheConfig: *mut cudaFuncCache) -> cudaError_t;
}
extern "C" {
    pub fn cudaDeviceGetStreamPriorityRange(
        leastPriority: *mut ::std::os::raw::c_int,
        greatestPriority: *mut ::std::os::raw::c_int,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaDeviceSetCacheConfig(cacheConfig: cudaFuncCache) -> cudaError_t;
}
extern "C" {
    pub fn cudaDeviceGetSharedMemConfig(pConfig: *mut cudaSharedMemConfig) -> cudaError_t;
}
extern "C" {
    pub fn cudaDeviceSetSharedMemConfig(config: cudaSharedMemConfig) -> cudaError_t;
}
extern "C" {
    pub fn cudaDeviceGetByPCIBusId(
        device: *mut ::std::os::raw::c_int,
        pciBusId: *const ::std::os::raw::c_char,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaDeviceGetPCIBusId(
        pciBusId: *mut ::std::os::raw::c_char,
        len: ::std::os::raw::c_int,
        device: ::std::os::raw::c_int,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaIpcGetEventHandle(handle: *mut cudaIpcEventHandle_t, event: cudaEvent_t) -> cudaError_t;
}
extern "C" {
    pub fn cudaIpcOpenEventHandle(event: *mut cudaEvent_t, handle: cudaIpcEventHandle_t) -> cudaError_t;
}
extern "C" {
    pub fn cudaIpcGetMemHandle(handle: *mut cudaIpcMemHandle_t, devPtr: *mut ::std::os::raw::c_void) -> cudaError_t;
}
extern "C" {
    pub fn cudaIpcOpenMemHandle(
        devPtr: *mut *mut ::std::os::raw::c_void,
        handle: cudaIpcMemHandle_t,
        flags: ::std::os::raw::c_uint,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaIpcCloseMemHandle(devPtr: *mut ::std::os::raw::c_void) -> cudaError_t;
}
extern "C" {
    pub fn cudaThreadExit() -> cudaError_t;
}
extern "C" {
    pub fn cudaThreadSynchronize() -> cudaError_t;
}
extern "C" {
    pub fn cudaThreadSetLimit(limit: cudaLimit, value: usize) -> cudaError_t;
}
extern "C" {
    pub fn cudaThreadGetLimit(pValue: *mut usize, limit: cudaLimit) -> cudaError_t;
}
extern "C" {
    pub fn cudaThreadGetCacheConfig(pCacheConfig: *mut cudaFuncCache) -> cudaError_t;
}
extern "C" {
    pub fn cudaThreadSetCacheConfig(cacheConfig: cudaFuncCache) -> cudaError_t;
}
extern "C" {
    pub fn cudaGetLastError() -> cudaError_t;
}
extern "C" {
    pub fn cudaPeekAtLastError() -> cudaError_t;
}
extern "C" {
    pub fn cudaGetErrorName(error: cudaError_t) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn cudaGetErrorString(error: cudaError_t) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn cudaGetDeviceCount(count: *mut ::std::os::raw::c_int) -> cudaError_t;
}
extern "C" {
    pub fn cudaGetDeviceProperties(prop: *mut cudaDeviceProp, device: ::std::os::raw::c_int) -> cudaError_t;
}
extern "C" {
    pub fn cudaDeviceGetAttribute(
        value: *mut ::std::os::raw::c_int,
        attr: cudaDeviceAttr,
        device: ::std::os::raw::c_int,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaDeviceGetP2PAttribute(
        value: *mut ::std::os::raw::c_int,
        attr: cudaDeviceP2PAttr,
        srcDevice: ::std::os::raw::c_int,
        dstDevice: ::std::os::raw::c_int,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaChooseDevice(device: *mut ::std::os::raw::c_int, prop: *const cudaDeviceProp) -> cudaError_t;
}
extern "C" {
    pub fn cudaSetDevice(device: ::std::os::raw::c_int) -> cudaError_t;
}
extern "C" {
    pub fn cudaGetDevice(device: *mut ::std::os::raw::c_int) -> cudaError_t;
}
extern "C" {
    pub fn cudaSetValidDevices(device_arr: *mut ::std::os::raw::c_int, len: ::std::os::raw::c_int) -> cudaError_t;
}
extern "C" {
    pub fn cudaSetDeviceFlags(flags: ::std::os::raw::c_uint) -> cudaError_t;
}
extern "C" {
    pub fn cudaGetDeviceFlags(flags: *mut ::std::os::raw::c_uint) -> cudaError_t;
}
extern "C" {
    pub fn cudaStreamCreate(pStream: *mut cudaStream_t) -> cudaError_t;
}
extern "C" {
    pub fn cudaStreamCreateWithFlags(pStream: *mut cudaStream_t, flags: ::std::os::raw::c_uint) -> cudaError_t;
}
extern "C" {
    pub fn cudaStreamCreateWithPriority(
        pStream: *mut cudaStream_t,
        flags: ::std::os::raw::c_uint,
        priority: ::std::os::raw::c_int,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaStreamGetPriority(hStream: cudaStream_t, priority: *mut ::std::os::raw::c_int) -> cudaError_t;
}
extern "C" {
    pub fn cudaStreamGetFlags(hStream: cudaStream_t, flags: *mut ::std::os::raw::c_uint) -> cudaError_t;
}
extern "C" {
    pub fn cudaStreamDestroy(stream: cudaStream_t) -> cudaError_t;
}
extern "C" {
    pub fn cudaStreamWaitEvent(stream: cudaStream_t, event: cudaEvent_t, flags: ::std::os::raw::c_uint) -> cudaError_t;
}
pub type cudaStreamCallback_t = ::std::option::Option<
    unsafe extern "C" fn(stream: cudaStream_t, status: cudaError_t, userData: *mut ::std::os::raw::c_void),
>;
extern "C" {
    pub fn cudaStreamAddCallback(
        stream: cudaStream_t,
        callback: cudaStreamCallback_t,
        userData: *mut ::std::os::raw::c_void,
        flags: ::std::os::raw::c_uint,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaStreamSynchronize(stream: cudaStream_t) -> cudaError_t;
}
extern "C" {
    pub fn cudaStreamQuery(stream: cudaStream_t) -> cudaError_t;
}
extern "C" {
    pub fn cudaStreamAttachMemAsync(
        stream: cudaStream_t,
        devPtr: *mut ::std::os::raw::c_void,
        length: usize,
        flags: ::std::os::raw::c_uint,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaEventCreate(event: *mut cudaEvent_t) -> cudaError_t;
}
extern "C" {
    pub fn cudaEventCreateWithFlags(event: *mut cudaEvent_t, flags: ::std::os::raw::c_uint) -> cudaError_t;
}
extern "C" {
    pub fn cudaEventRecord(event: cudaEvent_t, stream: cudaStream_t) -> cudaError_t;
}
extern "C" {
    pub fn cudaEventQuery(event: cudaEvent_t) -> cudaError_t;
}
extern "C" {
    pub fn cudaEventSynchronize(event: cudaEvent_t) -> cudaError_t;
}
extern "C" {
    pub fn cudaEventDestroy(event: cudaEvent_t) -> cudaError_t;
}
extern "C" {
    pub fn cudaEventElapsedTime(ms: *mut f32, start: cudaEvent_t, end: cudaEvent_t) -> cudaError_t;
}
extern "C" {
    pub fn cudaLaunchKernel(
        func: *const ::std::os::raw::c_void,
        gridDim: dim3,
        blockDim: dim3,
        args: *mut *mut ::std::os::raw::c_void,
        sharedMem: usize,
        stream: cudaStream_t,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaLaunchCooperativeKernel(
        func: *const ::std::os::raw::c_void,
        gridDim: dim3,
        blockDim: dim3,
        args: *mut *mut ::std::os::raw::c_void,
        sharedMem: usize,
        stream: cudaStream_t,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaLaunchCooperativeKernelMultiDevice(
        launchParamsList: *mut cudaLaunchParams,
        numDevices: ::std::os::raw::c_uint,
        flags: ::std::os::raw::c_uint,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaFuncSetCacheConfig(func: *const ::std::os::raw::c_void, cacheConfig: cudaFuncCache) -> cudaError_t;
}
extern "C" {
    pub fn cudaFuncSetSharedMemConfig(func: *const ::std::os::raw::c_void, config: cudaSharedMemConfig) -> cudaError_t;
}
extern "C" {
    pub fn cudaFuncGetAttributes(attr: *mut cudaFuncAttributes, func: *const ::std::os::raw::c_void) -> cudaError_t;
}
extern "C" {
    pub fn cudaFuncSetAttribute(
        func: *const ::std::os::raw::c_void,
        attr: cudaFuncAttribute,
        value: ::std::os::raw::c_int,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaSetDoubleForDevice(d: *mut f64) -> cudaError_t;
}
extern "C" {
    pub fn cudaSetDoubleForHost(d: *mut f64) -> cudaError_t;
}
extern "C" {
    pub fn cudaOccupancyMaxActiveBlocksPerMultiprocessor(
        numBlocks: *mut ::std::os::raw::c_int,
        func: *const ::std::os::raw::c_void,
        blockSize: ::std::os::raw::c_int,
        dynamicSMemSize: usize,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaOccupancyMaxActiveBlocksPerMultiprocessorWithFlags(
        numBlocks: *mut ::std::os::raw::c_int,
        func: *const ::std::os::raw::c_void,
        blockSize: ::std::os::raw::c_int,
        dynamicSMemSize: usize,
        flags: ::std::os::raw::c_uint,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaConfigureCall(gridDim: dim3, blockDim: dim3, sharedMem: usize, stream: cudaStream_t) -> cudaError_t;
}
extern "C" {
    pub fn cudaSetupArgument(arg: *const ::std::os::raw::c_void, size: usize, offset: usize) -> cudaError_t;
}
extern "C" {
    pub fn cudaLaunch(func: *const ::std::os::raw::c_void) -> cudaError_t;
}
extern "C" {
    pub fn cudaMallocManaged(
        devPtr: *mut *mut ::std::os::raw::c_void,
        size: usize,
        flags: ::std::os::raw::c_uint,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaMalloc(devPtr: *mut *mut ::std::os::raw::c_void, size: usize) -> cudaError_t;
}
extern "C" {
    pub fn cudaMallocHost(ptr: *mut *mut ::std::os::raw::c_void, size: usize) -> cudaError_t;
}
extern "C" {
    pub fn cudaMallocPitch(
        devPtr: *mut *mut ::std::os::raw::c_void,
        pitch: *mut usize,
        width: usize,
        height: usize,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaMallocArray(
        array: *mut cudaArray_t,
        desc: *const cudaChannelFormatDesc,
        width: usize,
        height: usize,
        flags: ::std::os::raw::c_uint,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaFree(devPtr: *mut ::std::os::raw::c_void) -> cudaError_t;
}
extern "C" {
    pub fn cudaFreeHost(ptr: *mut ::std::os::raw::c_void) -> cudaError_t;
}
extern "C" {
    pub fn cudaFreeArray(array: cudaArray_t) -> cudaError_t;
}
extern "C" {
    pub fn cudaFreeMipmappedArray(mipmappedArray: cudaMipmappedArray_t) -> cudaError_t;
}
extern "C" {
    pub fn cudaHostAlloc(
        pHost: *mut *mut ::std::os::raw::c_void,
        size: usize,
        flags: ::std::os::raw::c_uint,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaHostRegister(
        ptr: *mut ::std::os::raw::c_void,
        size: usize,
        flags: ::std::os::raw::c_uint,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaHostUnregister(ptr: *mut ::std::os::raw::c_void) -> cudaError_t;
}
extern "C" {
    pub fn cudaHostGetDevicePointer(
        pDevice: *mut *mut ::std::os::raw::c_void,
        pHost: *mut ::std::os::raw::c_void,
        flags: ::std::os::raw::c_uint,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaHostGetFlags(pFlags: *mut ::std::os::raw::c_uint, pHost: *mut ::std::os::raw::c_void) -> cudaError_t;
}
extern "C" {
    pub fn cudaMalloc3D(pitchedDevPtr: *mut cudaPitchedPtr, extent: cudaExtent) -> cudaError_t;
}
extern "C" {
    pub fn cudaMalloc3DArray(
        array: *mut cudaArray_t,
        desc: *const cudaChannelFormatDesc,
        extent: cudaExtent,
        flags: ::std::os::raw::c_uint,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaMallocMipmappedArray(
        mipmappedArray: *mut cudaMipmappedArray_t,
        desc: *const cudaChannelFormatDesc,
        extent: cudaExtent,
        numLevels: ::std::os::raw::c_uint,
        flags: ::std::os::raw::c_uint,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaGetMipmappedArrayLevel(
        levelArray: *mut cudaArray_t,
        mipmappedArray: cudaMipmappedArray_const_t,
        level: ::std::os::raw::c_uint,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaMemcpy3D(p: *const cudaMemcpy3DParms) -> cudaError_t;
}
extern "C" {
    pub fn cudaMemcpy3DPeer(p: *const cudaMemcpy3DPeerParms) -> cudaError_t;
}
extern "C" {
    pub fn cudaMemcpy3DAsync(p: *const cudaMemcpy3DParms, stream: cudaStream_t) -> cudaError_t;
}
extern "C" {
    pub fn cudaMemcpy3DPeerAsync(p: *const cudaMemcpy3DPeerParms, stream: cudaStream_t) -> cudaError_t;
}
extern "C" {
    pub fn cudaMemGetInfo(free: *mut usize, total: *mut usize) -> cudaError_t;
}
extern "C" {
    pub fn cudaArrayGetInfo(
        desc: *mut cudaChannelFormatDesc,
        extent: *mut cudaExtent,
        flags: *mut ::std::os::raw::c_uint,
        array: cudaArray_t,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaMemcpy(
        dst: *mut ::std::os::raw::c_void,
        src: *const ::std::os::raw::c_void,
        count: usize,
        kind: cudaMemcpyKind,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaMemcpyPeer(
        dst: *mut ::std::os::raw::c_void,
        dstDevice: ::std::os::raw::c_int,
        src: *const ::std::os::raw::c_void,
        srcDevice: ::std::os::raw::c_int,
        count: usize,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaMemcpyToArray(
        dst: cudaArray_t,
        wOffset: usize,
        hOffset: usize,
        src: *const ::std::os::raw::c_void,
        count: usize,
        kind: cudaMemcpyKind,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaMemcpyFromArray(
        dst: *mut ::std::os::raw::c_void,
        src: cudaArray_const_t,
        wOffset: usize,
        hOffset: usize,
        count: usize,
        kind: cudaMemcpyKind,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaMemcpyArrayToArray(
        dst: cudaArray_t,
        wOffsetDst: usize,
        hOffsetDst: usize,
        src: cudaArray_const_t,
        wOffsetSrc: usize,
        hOffsetSrc: usize,
        count: usize,
        kind: cudaMemcpyKind,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaMemcpy2D(
        dst: *mut ::std::os::raw::c_void,
        dpitch: usize,
        src: *const ::std::os::raw::c_void,
        spitch: usize,
        width: usize,
        height: usize,
        kind: cudaMemcpyKind,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaMemcpy2DToArray(
        dst: cudaArray_t,
        wOffset: usize,
        hOffset: usize,
        src: *const ::std::os::raw::c_void,
        spitch: usize,
        width: usize,
        height: usize,
        kind: cudaMemcpyKind,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaMemcpy2DFromArray(
        dst: *mut ::std::os::raw::c_void,
        dpitch: usize,
        src: cudaArray_const_t,
        wOffset: usize,
        hOffset: usize,
        width: usize,
        height: usize,
        kind: cudaMemcpyKind,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaMemcpy2DArrayToArray(
        dst: cudaArray_t,
        wOffsetDst: usize,
        hOffsetDst: usize,
        src: cudaArray_const_t,
        wOffsetSrc: usize,
        hOffsetSrc: usize,
        width: usize,
        height: usize,
        kind: cudaMemcpyKind,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaMemcpyToSymbol(
        symbol: *const ::std::os::raw::c_void,
        src: *const ::std::os::raw::c_void,
        count: usize,
        offset: usize,
        kind: cudaMemcpyKind,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaMemcpyFromSymbol(
        dst: *mut ::std::os::raw::c_void,
        symbol: *const ::std::os::raw::c_void,
        count: usize,
        offset: usize,
        kind: cudaMemcpyKind,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaMemcpyAsync(
        dst: *mut ::std::os::raw::c_void,
        src: *const ::std::os::raw::c_void,
        count: usize,
        kind: cudaMemcpyKind,
        stream: cudaStream_t,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaMemcpyPeerAsync(
        dst: *mut ::std::os::raw::c_void,
        dstDevice: ::std::os::raw::c_int,
        src: *const ::std::os::raw::c_void,
        srcDevice: ::std::os::raw::c_int,
        count: usize,
        stream: cudaStream_t,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaMemcpyToArrayAsync(
        dst: cudaArray_t,
        wOffset: usize,
        hOffset: usize,
        src: *const ::std::os::raw::c_void,
        count: usize,
        kind: cudaMemcpyKind,
        stream: cudaStream_t,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaMemcpyFromArrayAsync(
        dst: *mut ::std::os::raw::c_void,
        src: cudaArray_const_t,
        wOffset: usize,
        hOffset: usize,
        count: usize,
        kind: cudaMemcpyKind,
        stream: cudaStream_t,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaMemcpy2DAsync(
        dst: *mut ::std::os::raw::c_void,
        dpitch: usize,
        src: *const ::std::os::raw::c_void,
        spitch: usize,
        width: usize,
        height: usize,
        kind: cudaMemcpyKind,
        stream: cudaStream_t,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaMemcpy2DToArrayAsync(
        dst: cudaArray_t,
        wOffset: usize,
        hOffset: usize,
        src: *const ::std::os::raw::c_void,
        spitch: usize,
        width: usize,
        height: usize,
        kind: cudaMemcpyKind,
        stream: cudaStream_t,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaMemcpy2DFromArrayAsync(
        dst: *mut ::std::os::raw::c_void,
        dpitch: usize,
        src: cudaArray_const_t,
        wOffset: usize,
        hOffset: usize,
        width: usize,
        height: usize,
        kind: cudaMemcpyKind,
        stream: cudaStream_t,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaMemcpyToSymbolAsync(
        symbol: *const ::std::os::raw::c_void,
        src: *const ::std::os::raw::c_void,
        count: usize,
        offset: usize,
        kind: cudaMemcpyKind,
        stream: cudaStream_t,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaMemcpyFromSymbolAsync(
        dst: *mut ::std::os::raw::c_void,
        symbol: *const ::std::os::raw::c_void,
        count: usize,
        offset: usize,
        kind: cudaMemcpyKind,
        stream: cudaStream_t,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaMemset(devPtr: *mut ::std::os::raw::c_void, value: ::std::os::raw::c_int, count: usize) -> cudaError_t;
}
extern "C" {
    pub fn cudaMemset2D(
        devPtr: *mut ::std::os::raw::c_void,
        pitch: usize,
        value: ::std::os::raw::c_int,
        width: usize,
        height: usize,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaMemset3D(pitchedDevPtr: cudaPitchedPtr, value: ::std::os::raw::c_int, extent: cudaExtent)
        -> cudaError_t;
}
extern "C" {
    pub fn cudaMemsetAsync(
        devPtr: *mut ::std::os::raw::c_void,
        value: ::std::os::raw::c_int,
        count: usize,
        stream: cudaStream_t,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaMemset2DAsync(
        devPtr: *mut ::std::os::raw::c_void,
        pitch: usize,
        value: ::std::os::raw::c_int,
        width: usize,
        height: usize,
        stream: cudaStream_t,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaMemset3DAsync(
        pitchedDevPtr: cudaPitchedPtr,
        value: ::std::os::raw::c_int,
        extent: cudaExtent,
        stream: cudaStream_t,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaGetSymbolAddress(
        devPtr: *mut *mut ::std::os::raw::c_void,
        symbol: *const ::std::os::raw::c_void,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaGetSymbolSize(size: *mut usize, symbol: *const ::std::os::raw::c_void) -> cudaError_t;
}
extern "C" {
    pub fn cudaMemPrefetchAsync(
        devPtr: *const ::std::os::raw::c_void,
        count: usize,
        dstDevice: ::std::os::raw::c_int,
        stream: cudaStream_t,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaMemAdvise(
        devPtr: *const ::std::os::raw::c_void,
        count: usize,
        advice: cudaMemoryAdvise,
        device: ::std::os::raw::c_int,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaMemRangeGetAttribute(
        data: *mut ::std::os::raw::c_void,
        dataSize: usize,
        attribute: cudaMemRangeAttribute,
        devPtr: *const ::std::os::raw::c_void,
        count: usize,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaMemRangeGetAttributes(
        data: *mut *mut ::std::os::raw::c_void,
        dataSizes: *mut usize,
        attributes: *mut cudaMemRangeAttribute,
        numAttributes: usize,
        devPtr: *const ::std::os::raw::c_void,
        count: usize,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaPointerGetAttributes(
        attributes: *mut cudaPointerAttributes,
        ptr: *const ::std::os::raw::c_void,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaDeviceCanAccessPeer(
        canAccessPeer: *mut ::std::os::raw::c_int,
        device: ::std::os::raw::c_int,
        peerDevice: ::std::os::raw::c_int,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaDeviceEnablePeerAccess(peerDevice: ::std::os::raw::c_int, flags: ::std::os::raw::c_uint) -> cudaError_t;
}
extern "C" {
    pub fn cudaDeviceDisablePeerAccess(peerDevice: ::std::os::raw::c_int) -> cudaError_t;
}
extern "C" {
    pub fn cudaGraphicsUnregisterResource(resource: cudaGraphicsResource_t) -> cudaError_t;
}
extern "C" {
    pub fn cudaGraphicsResourceSetMapFlags(
        resource: cudaGraphicsResource_t,
        flags: ::std::os::raw::c_uint,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaGraphicsMapResources(
        count: ::std::os::raw::c_int,
        resources: *mut cudaGraphicsResource_t,
        stream: cudaStream_t,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaGraphicsUnmapResources(
        count: ::std::os::raw::c_int,
        resources: *mut cudaGraphicsResource_t,
        stream: cudaStream_t,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaGraphicsResourceGetMappedPointer(
        devPtr: *mut *mut ::std::os::raw::c_void,
        size: *mut usize,
        resource: cudaGraphicsResource_t,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaGraphicsSubResourceGetMappedArray(
        array: *mut cudaArray_t,
        resource: cudaGraphicsResource_t,
        arrayIndex: ::std::os::raw::c_uint,
        mipLevel: ::std::os::raw::c_uint,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaGraphicsResourceGetMappedMipmappedArray(
        mipmappedArray: *mut cudaMipmappedArray_t,
        resource: cudaGraphicsResource_t,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaGetChannelDesc(desc: *mut cudaChannelFormatDesc, array: cudaArray_const_t) -> cudaError_t;
}
extern "C" {
    pub fn cudaCreateChannelDesc(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        z: ::std::os::raw::c_int,
        w: ::std::os::raw::c_int,
        f: cudaChannelFormatKind,
    ) -> cudaChannelFormatDesc;
}
extern "C" {
    pub fn cudaBindTexture(
        offset: *mut usize,
        texref: *const textureReference,
        devPtr: *const ::std::os::raw::c_void,
        desc: *const cudaChannelFormatDesc,
        size: usize,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaBindTexture2D(
        offset: *mut usize,
        texref: *const textureReference,
        devPtr: *const ::std::os::raw::c_void,
        desc: *const cudaChannelFormatDesc,
        width: usize,
        height: usize,
        pitch: usize,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaBindTextureToArray(
        texref: *const textureReference,
        array: cudaArray_const_t,
        desc: *const cudaChannelFormatDesc,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaBindTextureToMipmappedArray(
        texref: *const textureReference,
        mipmappedArray: cudaMipmappedArray_const_t,
        desc: *const cudaChannelFormatDesc,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaUnbindTexture(texref: *const textureReference) -> cudaError_t;
}
extern "C" {
    pub fn cudaGetTextureAlignmentOffset(offset: *mut usize, texref: *const textureReference) -> cudaError_t;
}
extern "C" {
    pub fn cudaGetTextureReference(
        texref: *mut *const textureReference,
        symbol: *const ::std::os::raw::c_void,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaBindSurfaceToArray(
        surfref: *const surfaceReference,
        array: cudaArray_const_t,
        desc: *const cudaChannelFormatDesc,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaGetSurfaceReference(
        surfref: *mut *const surfaceReference,
        symbol: *const ::std::os::raw::c_void,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaCreateTextureObject(
        pTexObject: *mut cudaTextureObject_t,
        pResDesc: *const cudaResourceDesc,
        pTexDesc: *const cudaTextureDesc,
        pResViewDesc: *const cudaResourceViewDesc,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaDestroyTextureObject(texObject: cudaTextureObject_t) -> cudaError_t;
}
extern "C" {
    pub fn cudaGetTextureObjectResourceDesc(
        pResDesc: *mut cudaResourceDesc,
        texObject: cudaTextureObject_t,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaGetTextureObjectTextureDesc(
        pTexDesc: *mut cudaTextureDesc,
        texObject: cudaTextureObject_t,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaGetTextureObjectResourceViewDesc(
        pResViewDesc: *mut cudaResourceViewDesc,
        texObject: cudaTextureObject_t,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaCreateSurfaceObject(
        pSurfObject: *mut cudaSurfaceObject_t,
        pResDesc: *const cudaResourceDesc,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaDestroySurfaceObject(surfObject: cudaSurfaceObject_t) -> cudaError_t;
}
extern "C" {
    pub fn cudaGetSurfaceObjectResourceDesc(
        pResDesc: *mut cudaResourceDesc,
        surfObject: cudaSurfaceObject_t,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaDriverGetVersion(driverVersion: *mut ::std::os::raw::c_int) -> cudaError_t;
}
extern "C" {
    pub fn cudaRuntimeGetVersion(runtimeVersion: *mut ::std::os::raw::c_int) -> cudaError_t;
}
extern "C" {
    pub fn cudaGetExportTable(
        ppExportTable: *mut *const ::std::os::raw::c_void,
        pExportTableId: *const cudaUUID_t,
    ) -> cudaError_t;
}