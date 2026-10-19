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

// Windows imports are indirected through this `__imp_` pointer on x86_64
// MSVC. Defining both symbols locally prevents an import-library member for
// the Windows 8 function from being selected, including when the reference
// originates in a transitive crate or in a previously cached build of std.
#[unsafe(export_name = "GetSystemTimePreciseAsFileTime")]
unsafe extern "system" fn get_system_time_precise_as_file_time(
    system_time_as_file_time: *mut FileTime,
) {
    unsafe { GetSystemTimeAsFileTime(system_time_as_file_time) }
}

#[used]
#[unsafe(export_name = "__imp_GetSystemTimePreciseAsFileTime")]
static GET_SYSTEM_TIME_PRECISE_AS_FILE_TIME_IMPORT: unsafe extern "system" fn(*mut FileTime) =
    get_system_time_precise_as_file_time;
