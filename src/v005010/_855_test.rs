use crate::{
    util::Parser,
    v005010::{Transmission, _855},
};

#[test]
fn test_855_1() {
    //source: https://www.dandh.com/docs/EDI_Guides%5CCustomer%5CImplementation%20Guide%20855,%204010.pdf
    let str = r#"ISA*00* *00* *ZZ*111111111 *01*007911209*150129*2215*U*00401*000122406*0*P*>~
GS*PR*111111111*007911209*20150129*2215*3152*X*004010~
ST*855*3152~
BAK*00*AC*801222*20150129~
PO1*1*140*******UP*893647*VP*EXPI9301CTBLK*BP*999999999999~
ACK*IA*****010*20150205~
CTT*1~
SE*10*3152~
GE*1*3152~
IEA*1*000122406~"#;
    let (rest, obj) = Transmission::<_855>::parse(str).unwrap();
    println!("{rest}");
    println!("{obj:?}");
}
