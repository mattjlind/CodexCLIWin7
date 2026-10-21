//! Link-time compatibility for the Windows 8 precise system-time API.

#[repr(C)]
struct FileTime {
    low_date_time: u32,
    high_date_time: u32,
}

#[link(name = "kernel32")]
unsafe extern "system" {
    fn GetSystemTimeAsFileTime(system_time_as_file_time: *mut FileTime);
}

#[unsafe(export_name = "GetSystemTimePreciseAsFileTime")]
unsafe extern "system" fn get_system_time_precise_as_file_time(
    system_time_as_file_time: *mut FileTime,
) {
    unsafe { GetSystemTimeAsFileTime(system_time_as_file_time) }
}

// x86_64 MSVC calls imported functions through an `__imp_` pointer. Providing
// it locally prevents the linker from selecting the Windows 8 import-library
// member, even when the reference originates in std or a transitive crate.
#[used]
#[unsafe(export_name = "__imp_GetSystemTimePreciseAsFileTime")]
static GET_SYSTEM_TIME_PRECISE_AS_FILE_TIME_IMPORT: unsafe extern "system" fn(*mut FileTime) =
    get_system_time_precise_as_file_time;
