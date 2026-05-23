use snekkja::{AisMessage, Parser};

fn parse(sentence: &[u8]) -> AisMessage {
    let mut parser = Parser::default();
    parser
        .parse(sentence)
        .expect("parse failed")
        .message
        .expect("no message")
}

// --- Type 1 ---

#[test]
fn type1_french_vessel() {
    insta::assert_debug_snapshot!(parse(b"!AIVDM,1,1,,A,13HOI:0P0000VOHLCnHQKwvL05Ip,0*23"));
}
#[test]
fn type1_mediterranean() {
    insta::assert_debug_snapshot!(parse(b"!AIVDM,1,1,,B,13eaJF0P00Qd388Eew6aagvH85Ip,0*45"));
}
#[test]
fn type1_canadian() {
    insta::assert_debug_snapshot!(parse(b"!AIVDM,1,1,,A,14eGrSPP00ncMJTO5C6aBwvP2D0?,0*7A"));
}
#[test]
fn type1_caribbean() {
    insta::assert_debug_snapshot!(parse(b"!AIVDM,1,1,,A,15MrVH0000KH<:V:NtBLoqFP2H9:,0*2F"));
}
#[test]
fn type1_mississippi() {
    insta::assert_debug_snapshot!(parse(b"!AIVDM,1,1,,A,15N9NLPP01IS<RFF7fLVmgvN00Rv,0*7F"));
}
#[test]
fn type1_belgian_port() {
    insta::assert_debug_snapshot!(parse(b"!AIVDM,1,1,,B,133hGvP0000CjLHMG0u==:VN05Ip,0*61"));
}
#[test]
fn type1_korean() {
    insta::assert_debug_snapshot!(parse(b"!AIVDM,1,1,,B,16S`2cPP00a3UF6EKT@2:?vOr0S2,0*00"));
}
#[test]
fn type1_st_lawrence() {
    insta::assert_debug_snapshot!(parse(b"!AIVDM,1,1,,A,14eGKMhP00rkraHJPivPFwvL0<0<,0*23"));
}
#[test]
fn type1_bohai_sea() {
    insta::assert_debug_snapshot!(parse(b"!AIVDM,1,1,,A,16:=?;0P00`SstvFnFbeGH6L088h,0*44"));
}
#[test]
fn type1_yangtze_underway() {
    insta::assert_debug_snapshot!(parse(b"!AIVDM,1,1,,A,169a:nP01g`hm4pB7:E0;@0L088i,0*5E"));
}

// --- Type 2 ---

#[test]
fn type2_barbados() {
    insta::assert_debug_snapshot!(parse(b"!AIVDM,1,1,,B,24SaQh500G0Cu7nMErpJ680N0@9C,0*3F"));
}
#[test]
fn type2_finnish() {
    insta::assert_debug_snapshot!(parse(b"!AIVDM,1,1,,B,23L?MH001UP>QqDMIOlC7BHN05Ip,0*17"));
}
#[test]
fn type2_dutch_moored() {
    insta::assert_debug_snapshot!(parse(b"!AIVDM,1,1,,A,23aFfl0P00PCR>4MEBBP0?vN20S<,0*23"));
}
#[test]
fn type2_scheldt_inbound() {
    insta::assert_debug_snapshot!(parse(b"!AIVDM,1,1,,B,23NCqt5OiL0>Ve4MI0rtEb0P2@9G,0*03"));
}
#[test]
fn type2_north_sea() {
    insta::assert_debug_snapshot!(parse(b"!AIVDM,1,1,,A,239a=5RP0wPBGTpMKnQP`gv<R<1t,0*4C"));
}

// --- Type 3 ---

#[test]
fn type3_gulf_of_mexico() {
    insta::assert_debug_snapshot!(parse(b"!AIVDM,1,1,,B,35Mtp?0016J5ohD?ofRWSF2R0000,0*28"));
}
#[test]
fn type3_rotterdam() {
    insta::assert_debug_snapshot!(parse(b"!AIVDM,1,1,,B,33aDqfhP00PD2OnMDdF@QOvN205A,0*13"));
}
#[test]
fn type3_pearl_river() {
    insta::assert_debug_snapshot!(parse(b"!AIVDM,1,1,,B,36:RS:001?87bnt=:rq68TnN00nh,0*20"));
}
#[test]
fn type3_yangtze_slow() {
    insta::assert_debug_snapshot!(parse(b"!AIVDM,1,1,,A,369AM`1P028d;40Aohk1EgvN2000,0*72"));
}
#[test]
fn type3_tokyo_bay() {
    insta::assert_debug_snapshot!(parse(b"!AIVDM,1,1,,A,33bi=B5wP0awIEJDAdSEoVLN0000,0*6C"));
}

// --- Type 18 ---

#[test]
fn type18_class_b_channel_b() {
    insta::assert_debug_snapshot!(parse(b"!AIVDM,1,1,,B,B43JRq00LhTWc5VejDI>wwWUoP06,0*29"));
}
#[test]
fn type18_class_b_display() {
    insta::assert_debug_snapshot!(parse(b"!AIVDM,1,1,,B,B6:io8@0=21k=`3C:eDJSww4SP00,0*68"));
}
#[test]
fn type18_class_b_channel_a() {
    insta::assert_debug_snapshot!(parse(b"!AIVDM,1,1,,A,B6:Qno000279<OSPdw`03wWUoP06,0*78"));
}

// --- Type 4 ---

#[test]
fn type4_denmark() {
    insta::assert_debug_snapshot!(parse(b"!AIVDM,1,1,,B,4020ssAuho;N?PeNwjOAp<70089A,0*09"));
}
#[test]
fn type4_canary_islands() {
    insta::assert_debug_snapshot!(parse(b"!AIVDM,1,1,,B,4028j=1uho;N>Npi9j?wtk700@8a,0*09"));
}
#[test]
fn type4_ligurian_sea() {
    insta::assert_debug_snapshot!(parse(b"!AIVDM,1,1,,B,402FhaQuho6Nj0dsn4I=k<i004ip,0*77"));
}
#[test]
fn type4_south_china_sea() {
    insta::assert_debug_snapshot!(parse(b"!AIVDM,1,1,,B,403tAEiuho;N?89E5:<nomQ00<8u,0*08"));
}
#[test]
fn type4_puget_sound() {
    insta::assert_debug_snapshot!(parse(b"!AIVDM,1,1,,A,4h3OvjAuho;N>o=cf0Knevo00l3;,0*38"));
}
#[test]
fn type4_valencia() {
    insta::assert_debug_snapshot!(parse(b"!AIVDM,1,1,,A,4028j:Quho;MgOvPkdFl:VG000S:,0*74"));
}
#[test]
fn type4_norway_no_timestamp() {
    insta::assert_debug_snapshot!(parse(b"!AIVDM,1,1,,A,402M4L@000Htt12>B0VPVH700`Ct,0*2A"));
}
