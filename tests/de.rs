use vast_protocol::v4::*;

#[test]
fn deserialize_v4() {
    let xml = r#"
<VAST version="4.0" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns="http://www.iab.com/VAST">
  <Ad id="20001" sequence="1" conditionalAd="false">
    <InLine>
      <AdSystem version="4.0">iabtechlab</AdSystem>
      <Error>http://example.com/error</Error>
      <Impression id="Impression-ID">http://example.com/track/impression</Impression>
      <Pricing model="cpm" currency="USD">
        <![CDATA[ 25.00 ]]>
      </Pricing>
      <AdTitle>iabtechlab video ad</AdTitle>
      <Creatives>
        <Creative id="5480" sequence="1" adId="2447226">
          <UniversalAdId idRegistry="Ad-ID" idValue="8465">8465</UniversalAdId>
          <Linear>
            <TrackingEvents>
              <Tracking event="start">http://example.com/tracking/start</Tracking>
              <Tracking event="firstQuartile">http://example.com/tracking/firstQuartile</Tracking>
              <Tracking event="midpoint">http://example.com/tracking/midpoint</Tracking>
              <Tracking event="thirdQuartile">http://example.com/tracking/thirdQuartile</Tracking>
              <Tracking event="complete">http://example.com/tracking/complete</Tracking>
              <Tracking event="progress" offset="00:00:10">http://example.com/tracking/progress-10</Tracking>
            </TrackingEvents>
            <Duration>00:00:16</Duration>
            <MediaFiles>
              <MediaFile id="5241" delivery="progressive" type="video/mp4" bitrate="2000" width="1280" height="720" minBitrate="1500" maxBitrate="2500" scalable="1" maintainAspectRatio="1" codec="H.264">
                <![CDATA[https://iab-publicfiles.s3.amazonaws.com/vast/VAST-4.0-Short-Intro.mp4]]>
              </MediaFile>
            </MediaFiles>
            <VideoClicks>
              <ClickThrough id="blog">
                <![CDATA[https://iabtechlab.com]]>
              </ClickThrough>
            </VideoClicks>
          </Linear>
        </Creative>
      </Creatives>
    </InLine>
  </Ad>
</VAST>
"#;

    let vast: VAST = vast_protocol::from_str(xml).unwrap();
    assert_eq!("4.0", vast.version);
    assert_eq!("20001", vast.ad.id);
    assert_eq!(1, vast.ad.sequence.unwrap());
    assert!(!vast.ad.conditional_ad.unwrap());

    // InLine tag
    let in_line = vast.ad.in_line.unwrap();
    assert_eq!("4.0", in_line.ad_system.version);

    // AdSystem tag
    assert_eq!("iabtechlab", in_line.ad_system.content);

    // Error tag
    assert_eq!(
        "http://example.com/error",
        in_line.error.as_ref().unwrap().0
    );

    // Impression tag
    assert_eq!("Impression-ID", in_line.impression.id);
    assert_eq!(
        "http://example.com/track/impression",
        in_line.impression.content
    );

    // Pricing tag
    assert_eq!("cpm", in_line.pricing.as_ref().unwrap().model);
    assert_eq!("USD", in_line.pricing.as_ref().unwrap().currency);
    assert_eq!(" 25.00 ", in_line.pricing.as_ref().unwrap().content);

    // Creative tag
    let creative = &in_line.creatives.content[0];
    assert_eq!("5480", creative.id);
    assert_eq!(1, creative.sequence);
    assert_eq!("2447226", creative.ad_id);

    // UniversalAdId tag
    let universal_ad_id = &creative.universal_ad_ids[0];
    assert_eq!("Ad-ID", universal_ad_id.id_registry);
    assert_eq!("8465", universal_ad_id.id_value.as_ref().unwrap());
    assert_eq!("8465", universal_ad_id.content);

    // Linear tag
    let linear = &creative.linear.as_ref().unwrap();

    // TrackingEvent tag
    let event0 = &linear.tracking_events.content[0];
    let event1 = &linear.tracking_events.content[1];
    let event2 = &linear.tracking_events.content[2];
    let event3 = &linear.tracking_events.content[3];
    let event4 = &linear.tracking_events.content[4];
    let event5 = &linear.tracking_events.content[5];
    assert_eq!("start", event0.event);
    assert_eq!("firstQuartile", event1.event);
    assert_eq!("midpoint", event2.event);
    assert_eq!("thirdQuartile", event3.event);
    assert_eq!("complete", event4.event);
    assert_eq!("progress", event5.event);

    assert!(event0.offset.is_none());
    assert!(event1.offset.is_none());
    assert!(event2.offset.is_none());
    assert!(event3.offset.is_none());
    assert!(event4.offset.is_none());
    assert_eq!("00:00:10", event5.offset.as_ref().unwrap());

    assert_eq!("http://example.com/tracking/start", event0.content);
    assert_eq!("http://example.com/tracking/firstQuartile", event1.content);
    assert_eq!("http://example.com/tracking/midpoint", event2.content);
    assert_eq!("http://example.com/tracking/thirdQuartile", event3.content);
    assert_eq!("http://example.com/tracking/complete", event4.content);
    assert_eq!("http://example.com/tracking/progress-10", event5.content);

    // Duration tag
    assert_eq!("00:00:16", linear.duration.0);

    // MediaFile tag
    let media = &linear.media_files.content[0];
    assert_eq!("5241", media.id.as_ref().unwrap());
    assert_eq!("progressive", media.delivery);
    assert_eq!("video/mp4", media.r#type);
    assert_eq!(2000, media.bitrate.unwrap());
    assert_eq!(1500, media.min_bitrate.unwrap());
    assert_eq!(2500, media.max_bitrate.unwrap());
    assert_eq!("1", media.scalable.as_ref().unwrap());
    assert_eq!("H.264", media.codec.as_ref().unwrap());
    assert_eq!("1", media.maintain_aspect_ratio.as_ref().unwrap());
    assert!(media.api_framework.is_none());
    assert_eq!(
        "https://iab-publicfiles.s3.amazonaws.com/vast/VAST-4.0-Short-Intro.mp4",
        media.content
    );

    // ClickThrough tag
    let click_through = &linear
        .video_clicks
        .as_ref()
        .unwrap()
        .click_through
        .as_ref()
        .unwrap();
    assert_eq!("blog", click_through.id);
    assert_eq!("https://iabtechlab.com", click_through.content);
}
