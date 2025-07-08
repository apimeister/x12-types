use super::*;

#[test]
fn parse_999_01() {
    //source: https://github.com/Project-Herophilus/Project-Herophilus-Assets/blob/main/Testing/TestData/samples-edi/999%20Sample%20(Accepted).txt
    let str = r#"ISA*00*          *00*          *ZZ*EMEDNYBAT      *ZZ*ETIN           *110311*0521*^*00501*052127406*0*P*|~GS*FA*EMEDNYBAT*ABCD*20110311*0521*52127406*X*005010X231A1~ST*999*4001*005010X231A1~AK1*HC*28*005010X223A2~AK2*837*0028~IK5*A~AK9*A*1*1*1~SE*6*4001~GE*1*52127406~IEA*1*052127406~"#;
    let (rest, obj) = Transmission::<_999>::parse(str).unwrap();
    println!("{rest}");
    println!("{obj:?}");
}

#[test]
fn parse_999_2() {
    //source: https://github.com/Project-Herophilus/Project-Herophilus-Assets/blob/main/Testing/TestData/samples-edi/999%20Sample%20(Rejected).txt
    let str = r#"ISA*00*          *00*          *ZZ*EMEDNYBAT      *ZZ*ETIN           *110311*0958*^*00501*095832987*0*T*|~GS*FA*EMEDNY*ABCD*20110311*0958*95832987*X*005010X231A1~ST*999*95832987*005010X231A1~AK1*HC*28*005010X223A2~AK2*837*0028~IK3*NM*17**1~IK5*R*5~AK9*R*1*1*0~SE*7*95832987~GE*1*95832987~IEA*1*095832987~"#;
    let (rest, obj) = Transmission::<_999>::parse(str).unwrap();
    println!("{rest}");
    println!("{obj:?}");
}
