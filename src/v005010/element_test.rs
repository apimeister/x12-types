use crate::util::Parser;
use crate::util::X12Element;
use crate::v005010::element::{E373, E374, E737, E738};
use crate::v005010::*;

// DTM is now fully typed: _01 = 374 Date/Time Qualifier (enum), _02 = 373 Date,
// _03 = 337 Time. DTM appears in nearly every transaction set.
#[test]
fn dtm_typed() {
    let (rest, dtm) = DTM::parse("DTM*011*20200101~").unwrap();
    assert_eq!(rest, "");
    assert_eq!(dtm._01, E374::Shipped); // 011
    assert_eq!(
        dtm._02.as_ref().and_then(|d| d.date()),
        chrono::NaiveDate::from_ymd_opt(2020, 1, 1)
    );
    assert_eq!(format!("{dtm}"), "DTM*011*20200101~\n");

    // an unrecognized qualifier still parses and round-trips losslessly
    let (_, dtm2) = DTM::parse("DTM*ZZZ~").unwrap();
    assert_eq!(dtm2._01, E374::Unknown("ZZZ".to_string()));
    assert_eq!(format!("{dtm2}"), "DTM*ZZZ~\n");
}

// MEA-01/-02 are now typed X12 data elements (E737/E738), parsed straight from the
// segment. Known codes map to variants; unknown codes are preserved verbatim so the
// segment still renders and round-trips byte-for-byte.
#[test]
fn mea_typed_codes() {
    let (rest, mea) = MEA::parse("MEA*PD*HT*5~").unwrap();
    assert_eq!(rest, "");
    assert_eq!(mea._01, Some(E737::Pd)); // 737 "Physical Dimensions"
    assert_eq!(mea._02, Some(E738::Ht)); // 738 "Height"
                                         // 739 is a numeric element: raw text preserved, typed view via as_f64()
    assert_eq!(mea._03.as_ref().map(|e| e.raw()), Some("5"));
    assert_eq!(mea._03.as_ref().and_then(|e| e.as_f64()), Some(5.0));
    // rendering reproduces the codes/values exactly
    assert_eq!(format!("{mea}"), "MEA*PD*HT*5~\n");
}

#[test]
fn mea_unknown_code_round_trips() {
    // codes outside the (abbreviated) list land in Unknown and still round-trip
    let (_, mea) = MEA::parse("MEA*ZZ*QQ~").unwrap();
    assert_eq!(mea._01, Some(E737::Unknown("ZZ".to_string())));
    assert_eq!(mea._02, Some(E738::Unknown("QQ".to_string())));
    assert_eq!(format!("{mea}"), "MEA*ZZ*QQ~\n");
}

#[test]
fn mea_object_round_trip() {
    let (_, a) = MEA::parse("MEA*WT*N*100*LB~").unwrap();
    let r = format!("{a}");
    let (_, b) = MEA::parse(r.trim_end()).unwrap();
    assert_eq!(a, b);
}

// A date data element: Display produces the 8-char CCYYMMDD string, parsing yields a
// typed NaiveDate, and the element is identified by its X12 element number (E373).
#[test]
fn date_element() {
    let d = E373::from_x12("20200101");
    assert_eq!(d.raw(), "20200101");
    assert_eq!(format!("{d}"), "20200101");
    assert_eq!(d.date(), chrono::NaiveDate::from_ymd_opt(2020, 1, 1));

    let built = E373::from_date(chrono::NaiveDate::from_ymd_opt(2020, 1, 1).unwrap());
    assert_eq!(built, d);
    assert_eq!(built.raw(), "20200101");
}
