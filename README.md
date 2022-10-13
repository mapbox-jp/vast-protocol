## `vast-protocol`

[VAST](https://www.iab.com/guidelines/vast/) protocol v4 parser on top of [serde-rs](https://github.com/serde-rs/serde).

## Supported Tags

* [x] Error
* [ ] Extensions
  * [ ] Extension
* [x] Pricing
* [ ] AdVerifications
  * [ ] Verification
    * [ ] JavaScriptResource
* [ ] Wrapper
* [x] InLine
* [x] AdTitle
* [x] AdSystem
* [x] Impression
* [ ] ViewableImpression
  * [ ] Viewable
  * [ ] NotViewable
  * [ ] ViewableUndetermined
* [ ] Category
* [ ] VASTAdTagURI
* [x] Creatives
* [x] Creative
  * [ ] CompanionAds
    * [ ] Companion
      * [ ] StaticResource
      * [ ] CompanionClickThrough
  * [x] UniversalAdId
  * [ ] CreativeExtension
  * [x] Linear
  * [ ] NonLinearAds
    * [ ] NonLinear
      * [ ] StaticResource
      * [ ] NonLinearClickThrough
  * [x] Duration
  * [x] TrackingEvents
    * [ ] TrackingEvent
  * [x] MediaFiles
    * [x] MediaFile
    * [ ] Mezzanine
  * [x] VideoClicks
    * [x] ClickThrough
    * [x] ClickTracking

## Features flags

* `chrono`: Enables DateTime related helpers e.g. `v4::util::FromVastTimestamp`.

## Test

`vast-protocol` is tested against v4.0, v4.1 and v.4.2 XMLs found in [VAST_Samples](https://github.com/InteractiveAdvertisingBureau/VAST_Samples). To test in your local machine, clone VAST_Samples repo
```
git clone https://github.com/InteractiveAdvertisingBureau/VAST_Samples.git tests/VAST_Samples
```

Then run test with this command
```
cargo test --all-features -- --nocapture
```
