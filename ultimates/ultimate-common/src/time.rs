use std::sync::OnceLock;

pub use chrono::{DateTime, Duration, FixedOffset, Local, Utc};

use super::Result;

pub type OffsetDateTime = DateTime<FixedOffset>;
pub type UtcDateTime = DateTime<Utc>;

static LOCAL_OFFSET: OnceLock<FixedOffset> = OnceLock::new();

pub fn local_offset() -> &'static FixedOffset {
  LOCAL_OFFSET.get_or_init(_local_offset)
}

fn _local_offset() -> FixedOffset {
  FixedOffset::east_opt(3600 * 8).unwrap()
}

pub fn now_utc() -> OffsetDateTime {
  Utc::now().fixed_offset()
}

#[inline]
pub fn now() -> OffsetDateTime {
  Local::now().fixed_offset()
}

pub fn now_epoch_millis() -> i64 {
  let now = now_utc();
  now.timestamp_millis()
}

#[inline]
pub fn now_epoch_seconds() -> i64 {
  now_utc().timestamp()
}

pub fn format_time(time: OffsetDateTime) -> Result<String> {
  Ok(time.to_rfc3339())
}

pub fn now_utc_plus_sec_str(sec: u64) -> Result<String> {
  let new_time = now_utc() + Duration::seconds(sec as i64);
  format_time(new_time)
}

pub fn parse_utc(moment: &str) -> Result<OffsetDateTime> {
  let time = moment.parse::<OffsetDateTime>().unwrap();
  Ok(time)
}

#[cfg(feature = "prost-types")]
pub fn to_prost_timestamp(d: &OffsetDateTime) -> prost_types::Timestamp {
  prost_types::Timestamp { seconds: d.timestamp(), nanos: d.timestamp_subsec_nanos() as i32 }
}

#[cfg(feature = "prost-types")]
pub fn from_prost_timestamp(t: &prost_types::Timestamp) -> Option<OffsetDateTime> {
  DateTime::from_timestamp(t.seconds, t.nanos as u32).map(|d| d.fixed_offset())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_now_utc() {
    let now = now_utc();
    assert_eq!(*now.offset(), FixedOffset::east_opt(0).unwrap());
  }

  #[test]
  fn test_convert_std() {
    let now_utc = now_utc();
    println!("now is: {}", now_utc);

    let now_local = now();
    println!("now is {}", now_local);
  }
}