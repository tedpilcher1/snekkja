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

#[inline(always)]
pub fn parse_draught(data: u8) -> Option<f32> {
    match data {
        0 => None,
        d => Some(d as f32 / 10.0),
    }
}

// I1 encoding: 0.1-minute units (messages 17, 22, 23). Not-available matches the short form.
#[inline(always)]
pub fn parse_lon_i1(data: i32) -> Option<f32> {
    match data {
        108_600 => None,
        _ => Some(data as f32 / 600.0),
    }
}

#[inline(always)]
pub fn parse_lat_i1(data: i32) -> Option<f32> {
    match data {
        54_600 => None,
        _ => Some(data as f32 / 600.0),
    }
}
