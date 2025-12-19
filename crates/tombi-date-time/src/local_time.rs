#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug, Hash)]
pub struct LocalTime(crate::private::Time);

impl LocalTime {
    #[cfg(feature = "serde")]
    #[must_use]
    pub const fn type_name() -> &'static str {
        "local time"
    }

    #[must_use]
    pub const fn from_hms(hour: u8, minute: u8, second: u8) -> Self {
        Self(crate::private::Time {
            hour,
            minute,
            second,
            nanosecond: 0,
        })
    }

    #[must_use]
    pub fn from_hms_milli(hour: u8, minute: u8, second: u8, millisecond: u32) -> Self {
        debug_assert!(millisecond < 1_000);

        Self(crate::private::Time {
            hour,
            minute,
            second,
            nanosecond: millisecond * 1_000_000,
        })
    }

    #[must_use]
    pub fn from_hms_nano(hour: u8, minute: u8, second: u8, nanosecond: u32) -> Self {
        debug_assert!(nanosecond < 1_000_000_000);

        Self(crate::private::Time {
            hour,
            minute,
            second,
            nanosecond,
        })
    }

    #[must_use]
    pub const fn hour(&self) -> u8 {
        self.0.hour
    }

    #[must_use]
    pub const fn minute(&self) -> u8 {
        self.0.minute
    }

    #[must_use]
    pub const fn second(&self) -> u8 {
        self.0.second
    }

    #[must_use]
    pub const fn nanosecond(&self) -> u32 {
        self.0.nanosecond
    }
}

impl std::str::FromStr for LocalTime {
    type Err = crate::parse::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match crate::private::DateTime::from_str(s) {
            Ok(crate::private::DateTime {
                date: None,
                time: Some(time),
                offset: None,
            }) => Ok(Self(time)),
            Ok(_) => Err(crate::parse::Error::ExpectedLocalTime),
            Err(error) => Err(error),
        }
    }
}

impl std::fmt::Display for LocalTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(feature = "serde")]
impl serde::ser::Serialize for LocalTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        crate::private::DateTime::from(self.0).serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::de::Deserialize<'de> for LocalTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        match deserializer.deserialize_newtype_struct(
            crate::LOCAL_TIME_NEWTYPE_NAME,
            crate::private::DateTimeVisitor,
        )? {
            crate::private::DateTime {
                date: None,
                time: Some(time),
                offset: None,
            } => Ok(Self(time)),
            datetime => Err(serde::de::Error::invalid_type(
                serde::de::Unexpected::Other(datetime.type_name()),
                &Self::type_name(),
            )),
        }
    }
}

#[cfg(feature = "chrono")]
impl From<chrono::NaiveTime> for LocalTime {
    fn from(value: chrono::NaiveTime) -> Self {
        use chrono::Timelike;

        Self(crate::private::Time {
            hour: value.hour() as u8,
            minute: value.minute() as u8,
            second: value.second() as u8,
            nanosecond: value.nanosecond(),
        })
    }
}
