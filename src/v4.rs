use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VAST {
    #[serde(rename = "Ad")]
    pub ad: Ad,
    pub version: String,
    #[serde(rename = "xmlns:xs", default = "default_namespace")]
    pub default_namespace: String,
    #[serde(rename = "xmlns", default = "vast_namespace")]
    pub vast_namespace: String,
}

impl VAST {
    pub fn new(version: impl Into<String>, ad: Ad) -> VAST {
        VAST {
            ad,
            version: version.into(),
            default_namespace: default_namespace(),
            vast_namespace: vast_namespace(),
        }
    }
}

fn default_namespace() -> String {
    "http://www.w3.org/2001/XMLSchema".into()
}

fn vast_namespace() -> String {
    "http://www.iab.com/VAST".into()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ad {
    #[serde(rename = "InLine")]
    pub in_line: Option<InLine>,
    #[serde(rename = "Wrapper")]
    pub wrapper: Option<Wrapper>,
    pub id: String,
    pub sequence: Option<i64>,
    pub conditional_ad: Option<bool>,
}

/// Not yet implemented.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Wrapper;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct InLine {
    pub ad_system: AdSystem,
    pub error: Option<Error>,
    pub impression: Impression,
    pub description: Option<String>,
    pub advertiser: Option<String>,
    pub pricing: Option<Pricing>,
    pub ad_title: AdTitle,
    pub creatives: Creatives,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Error(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdTitle(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdSystem {
    pub version: String,
    #[serde(rename = "$value")]
    pub content: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Impression {
    pub id: String,
    #[serde(rename = "$value")]
    pub content: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pricing {
    pub model: String,
    pub currency: String,
    #[serde(rename = "$value")]
    pub content: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Creatives {
    #[serde(rename(serialize = "Creative", deserialize = "$value"))]
    pub content: Vec<Creative>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Creative {
    pub id: String,
    pub sequence: i64,
    pub ad_id: String,
    pub api_framework: Option<String>,
    #[serde(rename = "UniversalAdId")]
    pub universal_ad_ids: Vec<UniversalAdId>,
    #[serde(rename = "Linear")]
    pub linear: Option<Linear>,
    #[serde(rename = "NonLinearAds")]
    pub non_linear_ads: Option<NonLinear>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UniversalAdId {
    pub id_registry: String,
    pub id_value: Option<String>,
    #[serde(rename = "$value")]
    pub content: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Linear {
    pub tracking_events: TrackingEvents,
    pub duration: Duration,
    pub media_files: MediaFiles,
    pub video_clicks: Option<VideoClicks>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NonLinear {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Duration(pub String);

impl From<std::time::Duration> for Duration {
    fn from(d: std::time::Duration) -> Self {
        let ms = d.as_millis() % 1000;
        let s = d.as_secs() % 60;
        let m = (d.as_secs() / 60) % 60;
        let h = (d.as_secs() / 60) / 60;

        if d.as_secs() >= 100 * 60 * 60 {
            return Self("99:59:59".into());
        }

        if ms > 0 {
            Self(format!("{h:02}:{m:02}:{s:02}.{ms:03}"))
        } else {
            Self(format!("{h:02}:{m:02}:{s:02}"))
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackingEvents {
    #[serde(rename(serialize = "Tracking", deserialize = "$value"))]
    pub content: Vec<Tracking>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tracking {
    pub event: String,
    pub offset: Option<String>,
    #[serde(rename = "$value")]
    pub content: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaFiles {
    // This doesn't use `$value` tag name to workaround variant vector issue
    // https://github.com/tafia/quick-xml/pull/387
    #[serde(rename = "MediaFile")]
    pub content: Vec<MediaFile>,
    #[serde(rename = "Mezzanine")]
    pub mezzanine: Option<Mezzanine>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaFile {
    pub id: Option<String>,
    pub delivery: String,
    pub r#type: String,
    pub bitrate: Option<u64>,
    pub width: u64,
    pub height: u64,
    pub min_bitrate: Option<u64>,
    pub max_bitrate: Option<u64>,
    pub scalable: Option<String>,
    pub maintain_aspect_ratio: Option<String>,
    pub codec: Option<String>,
    pub api_framework: Option<String>,
    #[serde(rename = "$value")]
    pub content: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mezzanine {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct VideoClicks {
    pub click_through: Option<ClickThrough>,
    pub click_tracking: Option<ClickTracking>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClickThrough {
    pub id: String,
    #[serde(rename = "$value")]
    pub content: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClickTracking {
    pub id: Option<String>,
    #[serde(rename = "$value")]
    pub content: Option<String>,
}

pub mod util {
    #[cfg(feature = "chrono")]
    pub trait FromVastTimestamp {
        fn parse_from_vast_timestamp(
            s: &str,
        ) -> chrono::ParseResult<chrono::DateTime<chrono::FixedOffset>> {
            chrono::DateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S%.3f%#z")
        }
    }

    #[cfg(feature = "chrono")]
    impl FromVastTimestamp for chrono::DateTime<chrono::FixedOffset> {}
}
