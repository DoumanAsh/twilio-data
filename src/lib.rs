//! Twilio API data structs
//!
//! To be used as building blocks.

#![warn(missing_docs)]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::style))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::needless_lifetimes))]

use core::fmt::{self, Write};

use serde::Deserialize;

mod encoder;
mod ser;

///Twilio REST API base url
pub const REST_API_URL: &str = "api.twilio.com/2010-04-01/Accounts";
///Twilio REST API endpoint for SMS
pub const REST_API_SMS_ENDPOINT: &str = "Messages.json";
///Twilio REST API endpoint for calls
pub const REST_API_CALL_ENDPOINT: &str = "Calls.json";

//Fetch SMS link probably max
//https://api.twilio.com/2010-04-01/Accounts/XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX/Messages.json
const URL_BUFFER_SIZE: usize = 92;
///URL storage for `const fn` creation.
pub type UrlBuffer = str_buf::StrBuf<URL_BUFFER_SIZE>;

///Creates base URL to fetch SMS onto.
///
///To fetch SMS you need to call `<base>/<id>.json`
pub const fn get_sms_base(account_sid: &str) -> UrlBuffer {
    UrlBuffer::new().and("https://").and(REST_API_URL).and("/").and(account_sid).and("/").and("Messages")
}

///Creates URL to post SMS onto or fetch multiple SMS
pub const fn sms_resource_url(account_sid: &str) -> UrlBuffer {
    UrlBuffer::new().and("https://").and(REST_API_URL).and("/").and(account_sid).and("/").and(REST_API_SMS_ENDPOINT)
}

///Creates base URL to fetch Call onto.
///
///To fetch Call you need to call `<base>/<id>.json`
pub const fn get_call_base(account_sid: &str) -> UrlBuffer {
    UrlBuffer::new().and("https://").and(REST_API_URL).and("/").and(account_sid).and("/").and("Calls")
}

///Creates URL to post Call onto or fetch multiple Call
pub const fn call_resource_url(account_sid: &str) -> UrlBuffer {
    UrlBuffer::new().and("https://").and(REST_API_URL).and("/").and(account_sid).and("/").and(REST_API_CALL_ENDPOINT)
}

///Describes possible http methods, twilio can use to invoke callback.
pub enum TwilioMethod {
    ///Get
    GET,
    ///POST is default method
    POST
}

impl TwilioMethod {
    fn as_str(&self) -> &'static str {
        match self {
            TwilioMethod::GET => "GET",
            TwilioMethod::POST => "POST",
        }
    }
}

impl Default for TwilioMethod {
    #[inline(always)]
    fn default() -> Self {
        TwilioMethod::POST
    }
}

///Generic Twilio request builder.
///
///Data is encoded as `application/x-www-form-urlencode`.
///
///When performing `GET` should be appended to URL as query string.
///
///When performing `POST` should be placed as body of HTTP Request with `Content-Type` equal to `application/x-www-form-urlencode`.
///
///While it implements `Serialize`, there is no need to employ it as internally data is already encoded.
pub struct TwilioRequest {
    buffer: Vec<u8>,
    len: usize,
}

impl TwilioRequest {
    ///Content type of Twilio API request.
    ///
    ///To be used for HTTP Post requests
    pub const CONTENT_TYPE: &'static str = "application/x-www-form-urlencode";

    ///Creates new request.
    pub const fn new() -> Self {
        Self {
            buffer: Vec::new(),
            len: 0
        }
    }

    #[inline]
    ///Returns raw `application/x-www-form-urlencoded` data.
    pub fn into_bytes(self) -> Vec<u8> {
        self.buffer
    }

    #[inline]
    ///Returns string `application/x-www-form-urlencoded` data.
    pub fn into_string(self) -> String {
        unsafe {
            String::from_utf8_unchecked(self.buffer)
        }
    }

    #[inline]
    ///Returns reference as string `application/x-www-form-urlencoded` data.
    pub fn as_form(&self) -> &str {
        unsafe {
            core::str::from_utf8_unchecked(&self.buffer)
        }
    }

    fn add_pair(&mut self, field: &str, value: &str) -> &mut Self {
        self.len += 1;
        encoder::push_pair(field, value, &mut self.buffer);
        self
    }

    #[inline]
    ///Adds `AccountSid` to specify owner of the resource.
    pub fn account_sid(&mut self, sid: &str) -> &mut Self {
        self.add_pair("AccountSid", sid)
    }

    #[inline]
    ///Adds `From` field, which is identifier of caller.
    ///
    ///Type should be the same as for `To`
    pub fn from(&mut self, from: &str) -> &mut Self {
        self.add_pair("From", from)
    }

    #[inline]
    ///Adds `To` field, which is identifier of callee.
    ///
    ///Type should be the same as for `To`
    pub fn to(&mut self, to: &str) -> &mut Self {
        self.add_pair("To", to)
    }

    #[inline]
    ///Adds `Body` field.
    pub fn body(&mut self, body: &str) -> &mut Self {
        debug_assert!(body.len() <= 1_600, "Text body cannot exceed 1600 characters");
        self.add_pair("Body", body)
    }

    #[inline]
    ///Adds `MediaUrl` field.
    pub fn media_url(&mut self, media_url: &str) -> &mut Self {
        self.add_pair("MediaUrl", media_url)
    }

    #[inline]
    ///Adds `StatusCallback` field, which is url where to perform POST request
    pub fn post_status_callback(&mut self, url: &str) -> &mut Self {
        self.add_pair("StatusCallback", url)
    }

    #[inline]
    ///Sets `ProvideFeedback` field, to specify whether message delivery should be tracked.
    pub fn provide_feedback(&mut self, value: bool) -> &mut Self {
        match value {
            true => self.add_pair("ProvideFeedback", "true"),
            false => self.add_pair("ProvideFeedback", "false"),
        }
    }

    #[inline]
    ///Sets `Attempt` field, to indicate total number of attempts to post message.
    pub fn attempt(&mut self, attempt: u32) -> &mut Self {
        let mut buf = str_buf::StrBuf::<10>::new();
        let _ = write!(buf, "{}", attempt);
        self.add_pair("Attempt", buf.as_str())
    }

    #[inline]
    ///Sets `ValidityPeriod` field, to indicate number of seconds allowed in waiting queue.
    ///
    ///If message is enqueuedfor longer, it is discarded by Twilio
    pub fn validity_period(&mut self, attempt: u16) -> &mut Self {
        let mut buf = str_buf::StrBuf::<5>::new();
        let _ = write!(buf, "{}", attempt);
        self.add_pair("ValidityPeriod", buf.as_str())
    }

    #[inline]
    ///Sets `SendAt` field, to indicate where message is to be sent.
    pub fn send_at(&mut self, date: &str) -> &mut Self {
        self.add_pair("SendAt", date)
    }

    #[inline]
    ///Sets `Twiml` field, to provide call's content as xml string.
    pub fn twiml(&mut self, twiml: &str) -> &mut Self {
        self.add_pair("Twiml", twiml)
    }

    #[inline]
    ///Sets `Url` field, to provide call's content via GET to the provided url
    pub fn url(&mut self, url: &str) -> &mut Self {
        self.add_pair("Url", url)
    }

    #[inline]
    ///Sets `Url` field, to provide call's content via GET to the provided url.
    ///
    ///With option of setting HTTP method to access URL.
    pub fn url_with_method(&mut self, method: TwilioMethod, url: &str) -> &mut Self {
        self.add_pair("Method", method.as_str()).add_pair("Url", url)
    }

    #[inline]
    ///Sets `StatusCallback` field, to provide URL where to post status information.
    pub fn status_url(&mut self, url: &str) -> &mut Self {
        self.add_pair("StatusCallback", url)
    }

    #[inline]
    ///Sets `StatusCallback` field, to provide URL where to post status information.
    ///
    ///With option of setting HTTP method to access URL.
    pub fn status_url_with_method(&mut self, method: TwilioMethod, url: &str) -> &mut Self {
        self.add_pair("StatusCallbackMethod", method.as_str()).add_pair("StatusCallback", url)
    }


    #[inline]
    ///Sets `CallerId` field, to provide caller identification.
    pub fn caller_id(&mut self, id: &str) -> &mut Self {
        self.add_pair("CallerId", id)
    }

    #[inline]
    ///Sets `SendDigits` field, to provide set of keys to dial after call is established.
    pub fn send_digits(&mut self, digits: &str) -> &mut Self {
        debug_assert!(digits.len() <= 32, "SendDigits cannot exceed 32");
        self.add_pair("SendDigits", digits)
    }

    #[inline]
    ///Sets `PageSize` field, to provide number of resources max for when reading multiple resources
    pub fn page_size(&mut self, size: u32) -> &mut Self {
        debug_assert_ne!(size, 0);
        let mut buf = str_buf::StrBuf::<10>::new();
        let _ = write!(buf, "{}", size);
        self.add_pair("PageSize", buf.as_str())
    }

    #[inline]
    ///Sets `StartDate` field, to provide starting date for when reading multiple calls
    pub fn start_date(&mut self, date: &str) -> &mut Self {
        self.add_pair("StartDate", date)
    }

    #[inline]
    ///Sets `EndDate` field, to provide ending date for when reading multiple calls
    pub fn end_date(&mut self, date: &str) -> &mut Self {
        self.add_pair("EndDate", date)
    }

    #[inline]
    ///Sets `DateSent` field, to provide message date for when reading multiple message
    pub fn date_sent(&mut self, date: &str) -> &mut Self {
        self.add_pair("DateSent", date)
    }
}

#[derive(Debug)]
///Call instruction.
pub enum CallInstruction<'a> {
    ///Provides xml with Twiml instructions
    Twiml(&'a str),
    ///Provides URL pointing to xml file with Twiml instructions
    Url(&'a str),
}

#[derive(Debug)]
///Describes minimal Call request, suitable for urlencoded serialization
pub struct Call<'a> {
    ///Phone number of source
    pub from: &'a str,
    ///Phone number of destination
    pub to: &'a str,
    ///Call content
    pub instruction: CallInstruction<'a>,
}

impl<'a> Call<'a> {
    #[inline]
    ///Converts to generic TwilioRequest
    pub fn request(&self) -> TwilioRequest {
        let mut res = TwilioRequest::new();
        res.from(self.from).to(self.to);
        match self.instruction {
            CallInstruction::Twiml(twiml) => res.twiml(twiml),
            CallInstruction::Url(url) => res.url(url),
        };
        res
    }
}

impl<'a> fmt::Display for Call<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        encoder::format_pair("From", self.from, fmt)?;
        fmt.write_str(encoder::SEP)?;
        encoder::format_pair("To", self.to, fmt)?;
        fmt.write_str(encoder::SEP)?;
        match self.instruction {
            CallInstruction::Twiml(twiml) => encoder::format_pair("Twiml", twiml, fmt)?,
            CallInstruction::Url(url) => encoder::format_pair("Url", url, fmt)?,
        }

        Ok(())
    }
}

impl<'a> Into<TwilioRequest> for Call<'a> {
    #[inline(always)]
    fn into(self) -> TwilioRequest {
        self.request()
    }
}

#[derive(Debug)]
///Describes SMS, suitable for urlencoded serialization
pub struct Sms<'a> {
    ///Phone number of source
    pub from: &'a str,
    ///Phone number of destination
    pub to: &'a str,
    ///Text body
    pub body: &'a str,
}

impl<'a> Sms<'a> {
    #[inline]
    ///Converts to generic TwilioRequest
    pub fn request(&self) -> TwilioRequest {
        let mut res = TwilioRequest::new();
        res.from(self.from).to(self.to).body(self.body);
        res
    }
}

impl<'a> fmt::Display for Sms<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        encoder::format_pair("From", self.from, fmt)?;
        fmt.write_str(encoder::SEP)?;
        encoder::format_pair("To", self.to, fmt)?;
        fmt.write_str(encoder::SEP)?;
        encoder::format_pair("Body", self.body, fmt)?;

        Ok(())
    }
}

impl<'a> Into<TwilioRequest> for Sms<'a> {
    #[inline(always)]
    fn into(self) -> TwilioRequest {
        self.request()
    }
}

#[derive(Debug)]
///Describes MMS, suitable for urlencoded serialization
pub struct Mms<'a> {
    ///Flattened SMS part
    pub sms: Sms<'a>,
    ///Url with media content.
    ///
    ///Twilio generally accepts `.gif`, `.png` and `.jpeg` images so it formats it for device.
    ///Other formats are sent as it is, but MMS is limited to 5mb.
    pub media_url: &'a str
}

impl<'a> Mms<'a> {
    #[inline]
    ///Converts to generic TwilioRequest
    pub fn request(&self) -> TwilioRequest {
        let mut res = self.sms.request();
        res.media_url(self.media_url);
        res
    }
}

impl<'a> fmt::Display for Mms<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.sms.fmt(fmt)?;
        fmt.write_str(encoder::SEP)?;
        encoder::format_pair("MediaUrl", self.media_url, fmt)?;

        Ok(())
    }
}

impl<'a> Into<TwilioRequest> for Mms<'a> {
    #[inline(always)]
    fn into(self) -> TwilioRequest {
        self.request()
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
///Status of message.
pub enum SmsStatus {
    ///In queue for sending.
    Queued,
    ///Sending is in progress.
    Sending,
    ///Sent
    Sent,
    ///Failed to send
    Failed,
    ///Successfully delivered.
    Delivered,
    ///Not delivered yet.
    Undelivered,
    ///Receiving.
    Receiving,
    ///Received.
    Received,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
///Status of call.
pub enum CallStatus {
    ///In queue for sending.
    Queued,
    ///The call is ringing.
    Ringing,
    ///The call is ongoing.
    InProgress,
    ///The call is cancelled.
    Canceled,
    ///The call is finished.
    Completed,
    ///The callee is busy.
    Busy,
    ///The callee is not answering.
    NoAnswer,
    ///Cannot perform call.
    Failed,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
///Status of call.
pub enum CallDirection {
    ///Inbound
    Inbound,
    ///Output API
    OutboundApi,
    ///Output Dial
    OutboundDial,
    ///Trunking Terminating
    TrunkingTerminating,
    ///Trunking Originating
    TrunkingOriginating,
}

#[derive(Debug, Deserialize)]
///Result of correct SMS request.
pub struct SmsResult {
    ///Originator of message.
    pub from: String,
    ///Destination of message.
    pub to: String,
    ///Message content.
    pub body: String,
    ///ID of message
    ///
    ///Can be used to query SMS via following link:
    ///`/2010-04-01/Accounts/{account_sid}/Messages/{sid}.json`
    pub sid: String,
    ///Status of message.
    pub status: SmsStatus,
    ///URL of optional media attachment
    pub media_url: Option<String>,
    ///Cost of message
    pub price: Option<String>,
    ///Currency unit of `cost`.
    pub price_unit: String,
    ///Timestamp (including zone) of when message is created.
    pub date_created: String,
    ///Timestamp (including zone) of when message is sent.
    pub date_sent: Option<String>,
    ///Timestamp (including zone) of when message is updated.
    pub date_updated: String,
}

fn deserialize_number_from_any<'de, D: serde::de::Deserializer<'de>>(deserializer: D) -> Result<i64, D::Error> {
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrInt {
        String(String),
        Number(i64),
    }

    match StringOrInt::deserialize(deserializer)? {
        StringOrInt::String(s) => s.parse::<i64>().map_err(serde::de::Error::custom),
        StringOrInt::Number(i) => Ok(i),
    }
}

#[derive(Debug, Deserialize)]
///Result of correct SMS request.
pub struct CallResult {
    ///Originator of message.
    pub from: String,
    ///Destination of message.
    pub to: String,
    ///ID of call
    ///
    ///Can be used to query SMS via following link:
    ///`/2010-04-01/Accounts/{account_sid}/Calls/{sid}.json`
    pub sid: String,
    ///Status of message.
    pub status: CallStatus,
    ///Caller's name
    pub caller_name: Option<String>,
    ///Call's duration.
    pub duration: Option<i64>,
    ///Cost of call
    pub price: Option<String>,
    ///Currency unit of `cost`.
    pub price_unit: String,
    ///Timestamp (including zone) of when call is created.
    pub date_created: String,
    ///Timestamp (including zone) of when call is established.
    pub start_time: Option<String>,
    ///Timestamp (including zone) of when call is finished.
    pub end_time: Option<String>,
    ///Call's direction.
    pub direction: CallDirection,
    #[serde(deserialize_with = "deserialize_number_from_any")]
    ///The wait time in milliseconds before call is started.
    pub queue_time: i64
}


#[derive(Debug, Deserialize)]
///Error returned by Twilio REST API.
pub struct TwilioError {
    ///Error code
    pub code: usize,
    ///Error description
    pub message: String,
    ///Corresponding HTTP status code
    pub status: usize,
}

impl fmt::Display for TwilioError {
    #[inline(always)]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_fmt(format_args!("Twilio API responded with status={}, code={}, message: {}", self.status, self.code, self.message))
    }
}

impl std::error::Error for TwilioError {
}
