//! Windows 7 random-number compatibility for the prebuilt V8 archive.
//!
//! Current PartitionAlloc loads `ProcessPrng` from `bcryptprimitives.dll`, but
//! that entry point was introduced in Windows 8. Supplying PartitionAlloc's
//! `RandBytes` symbol here keeps its `rand_util_win.obj` archive member (and its
//! unconditional `CHECK(process_prng_fn)`) out of this executable.

use std::ffi::c_void;

#[link(name = "advapi32")]
unsafe extern "system" {
    #[link_name = "SystemFunction036"]
    fn rtl_gen_random(random_buffer: *mut c_void, random_buffer_length: u32) -> u8;
}

const MAX_RTL_GEN_RANDOM_LENGTH: usize = u32::MAX as usize;

// This is the MSVC-mangled name of:
//
//   void partition_alloc::internal::base::RandBytes(void*, size_t)
//
// The function is intentionally limited to the Win7 x86_64 build by the module
// declaration in `main.rs`.
#[unsafe(export_name = "?RandBytes@base@internal@partition_alloc@@YAXPEAX_K@Z")]
unsafe extern "C" fn partition_alloc_rand_bytes(mut output: *mut c_void, mut length: usize) {
    while length != 0 {
        let chunk_length = length.min(MAX_RTL_GEN_RANDOM_LENGTH);
        let success = unsafe { rtl_gen_random(output, chunk_length as u32) };
        if success == 0 {
            std::process::abort();
        }

        output = unsafe { output.cast::<u8>().add(chunk_length).cast() };
        length -= chunk_length;
    }
}

// Keep the replacement in the object even before the V8 archive is examined.
#[used]
static KEEP_PARTITION_ALLOC_RAND_BYTES: unsafe extern "C" fn(*mut c_void, usize) =
    partition_alloc_rand_bytes;
