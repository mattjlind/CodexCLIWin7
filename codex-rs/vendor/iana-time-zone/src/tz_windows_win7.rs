//! Windows 7 system time-zone discovery without Windows Runtime activation.

pub(crate) fn get_timezone_inner() -> Result<String, crate::GetTimezoneError> {
    // The upstream implementation creates a Windows.Globalization.Calendar,
    // which imports combase.dll and the WinRT API set that are unavailable on
    // Windows 7. Jiff uses GetDynamicTimeZoneInformation (available since
    // Vista) and maps the returned Windows key through the CLDR table.
    let time_zone =
        jiff::tz::TimeZone::try_system().map_err(|_| crate::GetTimezoneError::OsError)?;
    time_zone
        .iana_name()
        .map(str::to_owned)
        .ok_or(crate::GetTimezoneError::FailedParsingString)
}
