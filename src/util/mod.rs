use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::bytes::complete::take_while;
use nom::character::complete::newline;
use nom::combinator::opt;
use nom::multi::separated_list0;
use nom::sequence::delimited;
use nom::IResult;
use nom::Parser as _;

pub mod dt;
pub mod tm;

/// Compare two transmissions by their functional-group payload.
///
/// Gated behind `v004010` because it references that version's `Transmission`
/// type; without this gate `util` would not compile when `v004010` is disabled.
#[cfg(feature = "v004010")]
pub fn is_equal_payload<T: PartialEq>(
    src: &crate::v004010::Transmission<T>,
    target: &crate::v004010::Transmission<T>,
) -> bool {
    let Some(target_first) = target.functional_group.first() else {
        return src.functional_group.is_empty();
    };
    src.functional_group
        .iter()
        .all(|item| item.eq(target_first))
}

pub fn parse_line<'a>(input: &'a str, segment_name: &str) -> IResult<&'a str, Vec<&'a str>> {
    let tag_name = format!("{segment_name}*");
    let (rest, vars) = delimited(tag(tag_name.as_str()), take_until("~"), tag("~")).parse(input)?;
    let (_, vars) = separated_list0(
        tag("*"),
        take_while(|x: char| {
            x != '*' && (x.is_alphanumeric() || x.is_whitespace() || x.is_ascii_punctuation())
        }),
    )
    .parse(vars)?;
    // look for trailing newline
    let (rest, _) = opt(newline).parse(rest)?;
    Ok((rest, vars))
}

pub trait Parser<I, O, E> {
    fn parse(str: I) -> IResult<I, O>;
}

pub fn unborrow_string(input: &&str) -> String {
    input.to_string()
}

/// Define an X12 numeric data element (`R` decimal or `N` numeric).
///
/// Stores the raw text exactly (so `"007"`, `"5.00"`, `"+5"` all round-trip
/// byte-for-byte and never lose precision or format) and exposes typed views via
/// `as_f64()` / `as_i64()`. Named by X12 element number (e.g. `E739`).
#[macro_export]
macro_rules! num_element {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        #[derive(Clone, Default, Debug, PartialEq, Eq)]
        pub struct $name {
            raw: ::std::string::String,
        }

        impl $name {
            /// The raw element text, exactly as parsed.
            pub fn raw(&self) -> &str {
                &self.raw
            }
            /// The value as `f64`, parsed on demand (`None` if empty/unparseable).
            pub fn as_f64(&self) -> ::core::option::Option<f64> {
                self.raw.parse().ok()
            }
            /// The value as `i64`, parsed on demand (`None` if empty/unparseable).
            pub fn as_i64(&self) -> ::core::option::Option<i64> {
                self.raw.parse().ok()
            }
        }

        impl ::core::fmt::Display for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.write_str(&self.raw)
            }
        }

        impl $crate::util::X12Element for $name {
            fn from_x12(s: &str) -> Self {
                Self { raw: s.to_string() }
            }
        }

        impl ::serde::Serialize for $name {
            fn serialize<S: ::serde::Serializer>(&self, ser: S) -> ::core::result::Result<S::Ok, S::Error> {
                ser.serialize_str(&self.raw)
            }
        }

        impl<'de> ::serde::Deserialize<'de> for $name {
            fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> ::core::result::Result<Self, D::Error> {
                let raw = <::std::string::String as ::serde::Deserialize>::deserialize(d)?;
                ::core::result::Result::Ok(Self { raw })
            }
        }
    };
}

/// Define an X12 time data element (`TM`, format `HHMM`/`HHMMSS`).
///
/// Stores the raw text exactly (byte-for-byte round-trip) and exposes a typed
/// [`chrono::NaiveTime`] view via `time()`. Named by X12 element number (e.g. `E337`).
#[macro_export]
macro_rules! time_element {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        #[derive(Clone, Default, Debug, PartialEq, Eq)]
        pub struct $name {
            raw: ::std::string::String,
        }

        impl $name {
            /// The raw element text, exactly as parsed.
            pub fn raw(&self) -> &str {
                &self.raw
            }
            /// The typed time, parsed on demand from `HHMMSS` or `HHMM`.
            pub fn time(&self) -> ::core::option::Option<::chrono::NaiveTime> {
                ::chrono::NaiveTime::parse_from_str(&self.raw, "%H%M%S")
                    .or_else(|_| ::chrono::NaiveTime::parse_from_str(&self.raw, "%H%M"))
                    .ok()
            }
        }

        impl ::core::fmt::Display for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.write_str(&self.raw)
            }
        }

        impl $crate::util::X12Element for $name {
            fn from_x12(s: &str) -> Self {
                Self { raw: s.to_string() }
            }
        }

        impl ::serde::Serialize for $name {
            fn serialize<S: ::serde::Serializer>(&self, ser: S) -> ::core::result::Result<S::Ok, S::Error> {
                ser.serialize_str(&self.raw)
            }
        }

        impl<'de> ::serde::Deserialize<'de> for $name {
            fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> ::core::result::Result<Self, D::Error> {
                let raw = <::std::string::String as ::serde::Deserialize>::deserialize(d)?;
                ::core::result::Result::Ok(Self { raw })
            }
        }
    };
}

/// Conversion from a raw X12 element string into a field type.
///
/// Implemented for [`String`] (identity) and for the typed data-element types
/// generated by [`crate::code_enum`] / [`crate::date_element`]. The `ParseSegment`
/// derive uses this trait to populate any segment field that is not a plain
/// `String`/`Option<String>`, so new field types can opt in to typing without the
/// macro needing to know about them.
pub trait X12Element {
    fn from_x12(s: &str) -> Self;
}

impl X12Element for String {
    fn from_x12(s: &str) -> Self {
        s.to_string()
    }
}

/// Define an X12 code-list data element as an enum.
///
/// Generates the enum (one variant per code plus an `Unknown(String)` catch-all so
/// unknown/future codes still round-trip), its `Display` (variant -> code string),
/// `X12Element` parsing (code string -> variant), and serde (as the code string, so
/// JSON stays a plain string). The variant identifiers are supplied explicitly
/// because many X12 codes are not valid Rust identifiers (e.g. "9L", "C1").
///
/// At full scale (element 737 has 196 codes, 738 ~600, 355 has 844) these
/// invocations are expected to be machine-generated from the code lists.
#[macro_export]
macro_rules! code_enum {
    (
        $(#[$meta:meta])*
        $name:ident {
            $($(#[$vmeta:meta])* $code:literal => $variant:ident),* $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(Clone, Debug, PartialEq, Eq)]
        pub enum $name {
            $($(#[$vmeta])* $variant,)*
            /// A code not present in the published list; preserved verbatim so the
            /// element still renders and round-trips byte-for-byte.
            Unknown(String),
        }

        impl ::core::fmt::Display for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let s = match self {
                    $(Self::$variant => $code,)*
                    Self::Unknown(s) => s.as_str(),
                };
                f.write_str(s)
            }
        }

        impl $crate::util::X12Element for $name {
            fn from_x12(s: &str) -> Self {
                match s {
                    $($code => Self::$variant,)*
                    other => Self::Unknown(other.to_string()),
                }
            }
        }

        // Lets the enum sit in a mandatory (non-`Option`) field whose segment derives
        // `Default`. A real value is always supplied by the parser; the default is the
        // empty-string `Unknown`, which renders as nothing.
        impl ::core::default::Default for $name {
            fn default() -> Self {
                Self::Unknown(::std::string::String::new())
            }
        }

        impl ::serde::Serialize for $name {
            fn serialize<S: ::serde::Serializer>(&self, ser: S) -> ::core::result::Result<S::Ok, S::Error> {
                ser.serialize_str(&::std::string::ToString::to_string(self))
            }
        }

        impl<'de> ::serde::Deserialize<'de> for $name {
            fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> ::core::result::Result<Self, D::Error> {
                let s = <::std::string::String as ::serde::Deserialize>::deserialize(d)?;
                ::core::result::Result::Ok(<Self as $crate::util::X12Element>::from_x12(&s))
            }
        }
    };
}

/// Define an X12 date data element (format `CCYYMMDD`).
///
/// The element stores the raw 8-char text (so `Display` reproduces it exactly and it
/// round-trips byte-for-byte) and exposes a typed [`chrono::NaiveDate`] view via
/// `date()`. Naming follows the X12 element number (e.g. `E373`, `E109`).
#[macro_export]
macro_rules! date_element {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        #[derive(Clone, Default, Debug, PartialEq, Eq)]
        pub struct $name {
            raw: ::std::string::String,
        }

        impl $name {
            /// The raw element text, exactly as parsed (`CCYYMMDD`).
            pub fn raw(&self) -> &str {
                &self.raw
            }
            /// The typed date, parsed on demand from `CCYYMMDD`. `None` if empty or
            /// not a valid date (never panics, never alters the stored text).
            pub fn date(&self) -> ::core::option::Option<::chrono::NaiveDate> {
                ::chrono::NaiveDate::parse_from_str(&self.raw, "%Y%m%d").ok()
            }
            /// Build from a typed date, rendering it as `CCYYMMDD`.
            pub fn from_date(d: ::chrono::NaiveDate) -> Self {
                Self { raw: d.format("%Y%m%d").to_string() }
            }
        }

        impl ::core::fmt::Display for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.write_str(&self.raw)
            }
        }

        impl $crate::util::X12Element for $name {
            fn from_x12(s: &str) -> Self {
                Self { raw: s.to_string() }
            }
        }

        impl ::serde::Serialize for $name {
            fn serialize<S: ::serde::Serializer>(&self, ser: S) -> ::core::result::Result<S::Ok, S::Error> {
                ser.serialize_str(&self.raw)
            }
        }

        impl<'de> ::serde::Deserialize<'de> for $name {
            fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> ::core::result::Result<Self, D::Error> {
                let raw = <::std::string::String as ::serde::Deserialize>::deserialize(d)?;
                ::core::result::Result::Ok(Self { raw })
            }
        }
    };
}
