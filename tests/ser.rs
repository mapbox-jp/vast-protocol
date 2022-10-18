use vast_protocol::v4::*;

#[test]
fn serialize_v4() {
    let xml = "\
<VAST version=\"4.0\" xmlns:xs=\"http://www.w3.org/2001/XMLSchema\" xmlns=\"http://www.iab.com/VAST\">\
  <Ad id=\"20001\" sequence=\"1\" conditionalAd=\"false\">\
    <InLine>\
      <AdSystem version=\"4.0\">iabtechlab</AdSystem>\
      <Error>http://example.com/error</Error>\
      <Impression id=\"Impression-ID\">http://example.com/track/impression</Impression>\
      <Pricing model=\"cpm\" currency=\"USD\"> 25.00 </Pricing>\
      <AdTitle>iabtechlab video ad</AdTitle>\
      <Creatives>\
        <Creative id=\"5480\" sequence=\"1\" adId=\"2447226\">\
          <UniversalAdId idRegistry=\"Ad-ID\" idValue=\"8465\">8465</UniversalAdId>\
          <Linear>\
            <TrackingEvents>\
              <Tracking event=\"start\">http://example.com/tracking/start</Tracking>\
              <Tracking event=\"firstQuartile\">http://example.com/tracking/firstQuartile</Tracking>\
              <Tracking event=\"midpoint\">http://example.com/tracking/midpoint</Tracking>\
              <Tracking event=\"thirdQuartile\">http://example.com/tracking/thirdQuartile</Tracking>\
              <Tracking event=\"complete\">http://example.com/tracking/complete</Tracking>\
              <Tracking event=\"progress\" offset=\"00:00:10\">http://example.com/tracking/progress-10</Tracking>\
            </TrackingEvents>\
            <Duration>00:00:16</Duration>\
            <MediaFiles>\
              <MediaFile id=\"5241\" delivery=\"progressive\" type=\"video/mp4\" bitrate=\"2000\" width=\"1280\" height=\"720\" minBitrate=\"1500\" maxBitrate=\"2500\" scalable=\"1\" maintainAspectRatio=\"1\" codec=\"H.264\">\
                https://iab-publicfiles.s3.amazonaws.com/vast/VAST-4.0-Short-Intro.mp4\
              </MediaFile>\
            </MediaFiles>\
            <VideoClicks>\
              <ClickThrough id=\"blog\">\
                https://iabtechlab.com\
              </ClickThrough>\
            </VideoClicks>\
          </Linear>\
        </Creative>\
      </Creatives>\
    </InLine>\
  </Ad>\
</VAST>\
";

    let vast = VAST::new("4.0", Ad {
        id: "20001".into(),
        sequence: Some(1),
        conditional_ad: Some(false),
        wrapper: None,
        in_line: Some(InLine {
            ad_system: AdSystem {
                version: "4.0".into(),
                content: "iabtechlab".into(),
            },
            ad_title: AdTitle("iabtechlab video ad".into()),
            advertiser: None,
            description: None,
            error: Some(Error("http://example.com/error".into())),
            impression: Impression {
                id: "Impression-ID".into(),
                content: "http://example.com/track/impression".into(),
            },
            pricing: Some(Pricing {
                model: "cpm".into(),
                currency: "USD".into(),
                content: " 25.00 ".into(),
            }),
            creatives: Creatives {
                content: vec![Creative {
                    id: "5480".into(),
                    sequence: 1,
                    ad_id: "2447226".into(),
                    universal_ad_ids: vec![UniversalAdId {
                        id_registry: "Ad-ID".into(),
                        id_value: Some("8465".into()),
                        content: "8465".into(),
                    }],
                    non_linear_ads: None,
                    linear: Some(Linear {
                        duration: Duration("00:00:16".into()),
                        media_files: MediaFiles {
                            mezzanine: None,
                            content: vec![
                                MediaFile {
                                    id: Some("5241".into()),
                                    delivery: "progressive".into(),
                                    r#type: "video/mp4".into(),
                                    width: 1280,
                                    height: 720,
                                    bitrate: Some(2000),
                                    min_bitrate: Some(1500),
                                    max_bitrate: Some(2500),
                                    scalable: Some("1".into()),
                                    maintain_aspect_ratio: Some("1".into()),
                                    codec: Some("H.264".into()),
                                    content: "https://iab-publicfiles.s3.amazonaws.com/vast/VAST-4.0-Short-Intro.mp4".into(),
                                    api_framework: None,
                                }
                            ],
                        },
                        tracking_events: TrackingEvents {
                            content: vec![
                                Tracking { event: "start".into(), offset: None, content:"http://example.com/tracking/start".into() },
                                Tracking { event: "firstQuartile".into(), offset: None, content: "http://example.com/tracking/firstQuartile".into() },
                                Tracking { event: "midpoint".into(), offset: None, content: "http://example.com/tracking/midpoint".into() },
                                Tracking { event: "thirdQuartile".into(), offset: None, content: "http://example.com/tracking/thirdQuartile".into() },
                                Tracking { event: "complete".into(), offset: None, content: "http://example.com/tracking/complete".into() },
                                Tracking { event: "progress".into(),  offset: Some("00:00:10".into()), content: "http://example.com/tracking/progress-10".into() },
                            ]
                        },
                        video_clicks: Some(VideoClicks {
                               click_through: Some(ClickThrough {
                                   id: "blog".into(),
                                    content: "https://iabtechlab.com".into(),
                               }),
                               click_tracking: None,
                        })
                    }),
                    api_framework: None,
                }],
            },
        }),
    });

    let s = vast_protocol::to_string(&vast).unwrap();
    assert_eq!(xml, s);

    let _: VAST = vast_protocol::from_str(&s).unwrap();
}

#[test]
fn duration_from_std() {
    use std::time::Duration as StdDuration;
    assert_eq!("00:00:00", Duration::from(StdDuration::from_secs(0)).0);
    assert_eq!("00:00:50", Duration::from(StdDuration::from_secs(50)).0);
    assert_eq!("00:05:00", Duration::from(StdDuration::from_secs(300)).0);
    assert_eq!(
        "15:00:00",
        Duration::from(StdDuration::from_secs(60 * 60 * 15)).0
    );
    assert_eq!(
        "30:00:00",
        Duration::from(StdDuration::from_secs(60 * 60 * 30)).0
    );
    assert_eq!(
        "99:59:59",
        Duration::from(StdDuration::from_secs(60 * 60 * 99 + 60 * 59 + 59)).0
    );
    assert_eq!(
        "99:59:59",
        Duration::from(StdDuration::from_secs(60 * 60 * 100)).0 // 100 hours
    );

    assert_eq!("00:00:00", Duration::from(StdDuration::from_millis(0)).0);
    assert_eq!(
        "00:00:00.333",
        Duration::from(StdDuration::from_millis(333)).0
    );
    assert_eq!(
        "00:00:01.333",
        Duration::from(StdDuration::from_millis(1333)).0
    );
}
