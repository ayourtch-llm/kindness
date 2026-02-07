use solution::*;

#[test]
fn parse_u8_and_u16() {
    let p = u8_parser();
    let (val, rest) = (p.parse)(&[0x42, 0x01]).unwrap();
    assert_eq!(val, 0x42);
    assert_eq!(rest, &[0x01]);

    let p2 = u16_be();
    let (val2, rest2) = (p2.parse)(&[0x01, 0x02, 0xFF]).unwrap();
    assert_eq!(val2, 0x0102);
    assert_eq!(rest2, &[0xFF]);
}

#[test]
fn parse_bytes() {
    let p = bytes(3);
    let (val, rest) = (p.parse)(&[1, 2, 3, 4, 5]).unwrap();
    assert_eq!(val, vec![1, 2, 3]);
    assert_eq!(rest, &[4, 5]);

    let p2 = bytes(10);
    let result = (p2.parse)(&[1, 2, 3]);
    assert!(result.is_err());
}

#[test]
fn map_combinator() {
    let p = u8_parser().map(|b| b as u16 * 2);
    let (val, _) = (p.parse)(&[0x05]).unwrap();
    assert_eq!(val, 10u16);
}

#[test]
fn then_combinator() {
    let p = u8_parser().then(u16_be());
    let (val, rest) = (p.parse)(&[0x01, 0x00, 0x02, 0xFF]).unwrap();
    assert_eq!(val, (0x01u8, 0x0002u16));
    assert_eq!(rest, &[0xFF]);
}

#[test]
fn repeat_combinator() {
    let p = u8_parser().repeat(4);
    let (val, rest) = (p.parse)(&[10, 20, 30, 40, 50]).unwrap();
    assert_eq!(val, vec![10, 20, 30, 40]);
    assert_eq!(rest, &[50]);
}

#[test]
fn single_tlv_record() {
    // tag=0x01, length=3, value=[0xAA, 0xBB, 0xCC]
    let data = vec![0x01, 0x00, 0x03, 0xAA, 0xBB, 0xCC];
    let records = parse_tlv_stream(&data).unwrap();
    assert_eq!(records.len(), 1);
    assert_eq!(records[0], TlvRecord { tag: 0x01, value: vec![0xAA, 0xBB, 0xCC] });
}

#[test]
fn multiple_tlv_records() {
    let mut data = Vec::new();
    // Record 1: tag=0x01, length=2, value=[0x0A, 0x0B]
    data.extend_from_slice(&[0x01, 0x00, 0x02, 0x0A, 0x0B]);
    // Record 2: tag=0x02, length=0, value=[]
    data.extend_from_slice(&[0x02, 0x00, 0x00]);
    // Record 3: tag=0xFF, length=1, value=[0x42]
    data.extend_from_slice(&[0xFF, 0x00, 0x01, 0x42]);

    let records = parse_tlv_stream(&data).unwrap();
    assert_eq!(records.len(), 3);
    assert_eq!(records[0].tag, 0x01);
    assert_eq!(records[1].tag, 0x02);
    assert_eq!(records[1].value, Vec::<u8>::new());
    assert_eq!(records[2].value, vec![0x42]);
}
