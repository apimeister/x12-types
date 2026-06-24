use crate::util::Parser;
use crate::v005010::*;

// Application advice: two party loops, then an OTI loop carrying NM1, a technical-error
// (TED) loop with context/note/related-data, and a code-source (LM -> LQ) loop.
const SAMPLE: &str = r#"ST*824*0001~
BGN*11*FFA123*20200709~
N1*41*ABC INSURANCE*46*111111111~
PER*IC*JOHN*TE*8005551212~
N1*40*SMITHCO*46*A1234~
OTI*TA*TN*NA***20200709*0902*2*0001*834~
REF*1L*REF123~
NM1*41*2*ABC INSURANCE~
TED*024*MISSING DATA~
CTX*CLM01:123~
NTE*ZZZ*FREEFORM NOTE~
RED*DETAIL~
LM*AS~
LQ*0*001~
RED*CODE DETAIL~
SE*16*0001~"#;

#[test]
fn parse_824() {
    let (rest, obj) = _824::parse(SAMPLE).unwrap();
    assert_eq!(rest, "");
    assert_eq!(obj.st._01, "824");
    assert_eq!(obj.loop_n1.len(), 2);
    assert_eq!(obj.loop_n1[0].n1._01.to_string(), "41");
    assert_eq!(obj.loop_n1[0].per.len(), 1);
    assert_eq!(obj.loop_n1[1].n1._01.to_string(), "40");
    assert_eq!(obj.loop_oti.len(), 1);
    let oti = &obj.loop_oti[0];
    assert_eq!(oti.oti._01, "TA");
    assert_eq!(oti.r#ref.len(), 1);
    assert_eq!(oti.nm1.len(), 1);
    // technical-error loop with context/note/related-data
    assert_eq!(oti.loop_ted.len(), 1);
    assert_eq!(oti.loop_ted[0].ctx.len(), 1);
    assert_eq!(oti.loop_ted[0].nte.len(), 1);
    assert_eq!(oti.loop_ted[0].red.len(), 1);
    // code-source loop with its LQ sub-loop
    assert_eq!(oti.loop_lm.len(), 1);
    assert_eq!(oti.loop_lm[0].loop_lq.len(), 1);
    assert_eq!(oti.loop_lm[0].loop_lq[0].red.len(), 1);
}

#[test]
fn roundtrip_824() {
    let (_, first) = _824::parse(SAMPLE).unwrap();
    let rendered = format!("{first}");
    let (rest, second) = _824::parse(&rendered).unwrap();
    assert_eq!(rest, "");
    assert_eq!(first, second);
}

#[test]
fn full_transmission_824() {
    let str = r#"ISA*00*          *00*          *08*9254110060     *ZZ*123456789      *041216*0805*U*00501*000095071*0*P*>~
GS*AG*5137624388*123456789*20041216*0805*95071*X*005010~
ST*824*021390001~
BGN*11*FFA.ABCDEF.123456*20020709~
N1*41*ABC INSURANCE*46*111111111~
OTI*TA*TN*NA***20020709*0902*2*0001*834~
SE*4*021390001~
GE*1*95071~
IEA*1*000095071~"#;
    let (rest, obj) = Transmission::<_824>::parse(str).unwrap();
    assert!(rest.is_empty());
    assert_eq!(
        obj.functional_group[0].segments[0].loop_oti[0].oti._01,
        "TA"
    );
}
