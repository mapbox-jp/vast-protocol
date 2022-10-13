#[cfg(feature = "chrono")]
#[test]
fn test_parse_from_vast_timestamp() {
    use chrono::DateTime;
    use vast_protocol::v4::util::FromVastTimestamp;

    assert_eq!(
        "2016-01-17 08:15:07.127 -05:00",
        DateTime::parse_from_vast_timestamp("2016-01-17T08:15:07.127-05")
            .unwrap()
            .to_string()
    );

    assert_eq!(
        "2016-01-17 08:15:07.127 +09:00",
        DateTime::parse_from_vast_timestamp("2016-01-17T08:15:07.127+09")
            .unwrap()
            .to_string()
    );

    assert_eq!(
        "2016-01-17 08:15:07.127 +00:00",
        DateTime::parse_from_vast_timestamp("2016-01-17T08:15:07.127+00")
            .unwrap()
            .to_string()
    );

    // No milliseconds
    assert_eq!(
        "2016-01-17 08:15:07 +00:00",
        DateTime::parse_from_vast_timestamp("2016-01-17T08:15:07+00")
            .unwrap()
            .to_string()
    );

    // without offset
    assert!(DateTime::parse_from_vast_timestamp("2016-01-17T08:15:07.127").is_err());

    // invalid offset
    assert!(DateTime::parse_from_vast_timestamp("2016-01-17T08:15:07.127+99").is_err());

    // without time
    assert!(DateTime::parse_from_vast_timestamp("2016-01-17+00").is_err());
}
