#[inline(always)]
pub fn parse_sog(data: u16) -> Option<f32> {
    match data {
        1023 => None,
        _ => Some(data as f32 / 10.0),
    }
}

#[inline(always)]
pub fn parse_longitude(data: i32) -> Option<f32> {
    match data {
        108_600_000 => None,
        _ => Some(data as f32 / 600_000.0),
    }
}

#[inline(always)]
pub fn parse_latitude(data: i32) -> Option<f32> {
    match data {
        54_600_000 => None,
        _ => Some(data as f32 / 600_000.0),
    }
}

#[inline(always)]
pub fn parse_cog(data: u16) -> Option<f32> {
    match data {
        3600 => None,
        _ => Some(data as f32 / 10.0),
    }
}

#[inline(always)]
pub fn parse_true_heading(data: u16) -> Option<u16> {
    match data {
        511 => None,
        _ => Some(data),
    }
}
