use std::time::Duration;

use sea_orm::prelude::DateTimeUtc;

/// Validates the duration is greater than zero
pub fn is_non_zero_duration(value: &Duration, _ctx: &()) -> garde::Result {
    if value.as_secs() < 1 {
        return Err(garde::Error::new("duration cannot be zero"));
    }
    Ok(())
}

/// Validator that ensures the provided end of the date time
/// range is not before the start of the range
pub fn valid_range(start: &DateTimeUtc) -> impl FnOnce(&DateTimeUtc, &()) -> garde::Result + '_ {
    move |end, _| {
        if end.lt(start) {
            return Err(garde::Error::new("end timestamp cannot be before start"));
        }

        Ok(())
    }
}
