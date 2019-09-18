use bitflags::bitflags;

use crate::error::{errcode_to_result, Result};

use ffi::BLRuntimeBuildType::*;
bl_enum! {
    /// Blend2D runtime build type.
    pub enum BuildType {
        /// Describes a Blend2D debug build.
        Debug   = BL_RUNTIME_BUILD_TYPE_DEBUG,
        /// Describes a Blend2D release build.
        Release = BL_RUNTIME_BUILD_TYPE_RELEASE,
    }
    Default => Debug
}

use ffi::BLRuntimeCpuArch::*;
bl_enum! {
    /// CPU architecture that can be queried by [`query_system_info`].
    pub enum CpuArch {
        /// Unknown architecture.
        Unknown = BL_RUNTIME_CPU_ARCH_UNKNOWN,
        /// 32-bit or 64-bit X86 architecture.
        X86     = BL_RUNTIME_CPU_ARCH_X86,
        /// 32-bit or 64-bit ARM architecture.
        Arm     = BL_RUNTIME_CPU_ARCH_ARM,
        /// 32-bit or 64-bit MIPS architecture.
        Mips    = BL_RUNTIME_CPU_ARCH_MIPS,
    }
    Default => Unknown
}

use ffi::BLRuntimeCpuFeatures::*;
bitflags! {
    /// CPU features Blend2D supports.
    #[derive(Default)]
    pub struct CpuFeatures: u32 {
        const X86_SSE2   = BL_RUNTIME_CPU_FEATURE_X86_SSE2 as u32;
        const X86_SSE3   = BL_RUNTIME_CPU_FEATURE_X86_SSE3 as u32;
        const X86_SSSE3  = BL_RUNTIME_CPU_FEATURE_X86_SSSE3 as u32;
        const X86_SSE4_1 = BL_RUNTIME_CPU_FEATURE_X86_SSE4_1 as u32;
        const X86_SSE4_2 = BL_RUNTIME_CPU_FEATURE_X86_SSE4_2 as u32;
        const X86_AVX    = BL_RUNTIME_CPU_FEATURE_X86_AVX as u32;
        const X86_AVX2   = BL_RUNTIME_CPU_FEATURE_X86_AVX2 as u32;
    }
}

use ffi::BLRuntimeCleanupFlags::*;
bitflags! {
    /// Runtime cleanup flags that can be used through [`cleanup`].
    #[derive(Default)]
    pub struct CleanupFlags: u32 {
        /// Cleanup object memory pool.
        const OBJECT_POOL = BL_RUNTIME_CLEANUP_OBJECT_POOL as u32;
        /// Cleanup zeroed memory pool.
        const ZEROED_POOL = BL_RUNTIME_CLEANUP_ZEROED_POOL as u32;
        /// Cleanup thread pool (would join unused threads).
        const THREAD_POOL = BL_RUNTIME_CLEANUP_THREAD_POOL as u32;
    }
}

/// Tell the runtime to clean up resources according to the specified
/// [`CleanupFlags`].
pub fn cleanup(flags: CleanupFlags) -> Result<()> {
    unsafe { errcode_to_result(ffi::blRuntimeCleanup(flags.bits())) }
}

/// Blend2D build information.
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct BuildInfo {
    /// Blend2D version stored as `((MAJOR << 16) | (MINOR << 8) | PATCH)`.
    pub version: u32,
    /// Blend2D build type, see [`BuildType`].
    pub build_type: BuildType,
    /// Baseline CPU features, see [`CpuFeatures`].
    ///
    /// These features describe CPU features that were detected at compile-time.
    /// Baseline features are used to compile all source files so they represent
    /// the minimum feature-set the target CPU must support to run Blend2D.
    ///
    /// Official Blend2D builds set baseline at SSE2 on X86 target and NEON on
    /// ARM target. Custom builds can set use different baseline, which can be
    /// read through `BLRuntimeBuildInfo`.
    pub baseline_cpu_features: CpuFeatures,
    /// Supported CPU features, see [`CpuFeatures`].
    ///
    /// These features do not represent the features that the host CPU must
    /// support, instead, they represent all features that Blend2D can take
    /// advantage of in C++ code that uses instruction intrinsics. For
    /// example if AVX2 is part of `supportedCpuFeatures` it means that
    /// Blend2D can take advantage of it if there is a separate code-path.
    pub supported_cpu_features: CpuFeatures,
    /// Maximum size of an image (both width and height).
    pub max_image_size: u32,
    /// Maximum number of threads for asynchronous operations, including
    /// rendering.
    pub max_thread_count: u32,
    /// Reserved, must be zero.
    reserved: [u32; 2],
    /// Identification of the C++ compiler used to build Blend2D.
    compiler_info: [u8; 32],
}

impl BuildInfo {
    /// Queries the runtime's build info.
    #[inline]
    pub fn query() -> Result<Self> {
        query_build_info()
    }
}

/// Queries the runtime's build info.
pub fn query_build_info() -> Result<BuildInfo> {
    unsafe {
        let mut info = BuildInfo::default();
        errcode_to_result(ffi::blRuntimeQueryInfo(
            ffi::BLRuntimeInfoType::BL_RUNTIME_INFO_TYPE_BUILD as u32,
            &mut info as *mut _ as *mut _,
        ))
        .map(|_| info)
    }
}

/// System information queried by the runtime.
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct SystemInfo {
    /// Host CPU architecture, see [`CpuArch`].
    pub cpu_arch: CpuArch,
    /// Host CPU features, see [`CpuFeatures`].
    pub cpu_features: CpuFeatures,
    /// Number of cores of the host CPU/CPUs.
    pub core_count: u32,
    /// Number of threads of the host CPU/CPUs.
    pub thread_count: u32,
    /// Minimum stack size of threads.
    pub min_thread_stack_size: u32,
    /// Minimum stack size of worker threads used by Blend2D.
    pub min_worker_stack_size: u32,
    /// Allocation granularity of virtual memory (includes thread's stack).
    pub allocation_granularity: u32,
    /// Reserved for future use.
    reserved: [u32; 5],
}

impl SystemInfo {
    /// Queries the runtime's system info.
    #[inline]
    pub fn query() -> Result<Self> {
        query_system_info()
    }
}

/// Queries the runtime's system info.
pub fn query_system_info() -> Result<SystemInfo> {
    unsafe {
        let mut info = SystemInfo::default();
        errcode_to_result(ffi::blRuntimeQueryInfo(
            ffi::BLRuntimeInfoType::BL_RUNTIME_INFO_TYPE_SYSTEM as u32,
            &mut info as *mut _ as *mut _,
        ))
        .map(|_| info)
    }
}

/// Blend2D memory information that provides how much memory Blend2D allocated
/// and some other details about memory use.
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct MemoryInfo {
    /// Virtual memory used at this time.
    pub vm_used: usize,
    /// Virtual memory reserved (allocated internally).
    pub vm_reserved: usize,
    /// Overhead required to manage virtual memory allocations.
    pub vm_overhead: usize,
    /// Number of blocks of virtual memory allocated.
    pub vm_block_count: usize,
    /// Zeroed memory used at this time.
    pub zm_used: usize,
    /// Zeroed memory reserved (allocated internally).
    pub zm_reserved: usize,
    /// Overhead required to manage zeroed memory allocations.
    pub zm_overhead: usize,
    /// Number of blocks of zeroed memory allocated.
    pub zm_block_count: usize,
    /// Count of dynamic pipelines created and cached.
    pub dynamic_pipeline_count: usize,
}

impl MemoryInfo {
    /// Queries the runtime's memory info.
    #[inline]
    pub fn query() -> Result<Self> {
        query_memory_info()
    }
}

/// Queries the runtime's memory info.
pub fn query_memory_info() -> Result<MemoryInfo> {
    unsafe {
        let mut info = MemoryInfo::default();
        errcode_to_result(ffi::blRuntimeQueryInfo(
            ffi::BLRuntimeInfoType::BL_RUNTIME_INFO_TYPE_MEMORY as u32,
            &mut info as *mut _ as *mut _,
        ))
        .map(|_| info)
    }
}
