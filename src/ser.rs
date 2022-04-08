use super::{Call, CallInstruction, Sms, Mms, TwilioRequest};

use core::fmt;
use std::borrow::Cow;

use serde::ser::{Serialize, Serializer, SerializeMap, SerializeStruct};
use serde::de::{Deserializer, Deserialize, Visitor, MapAccess, SeqAccess};

impl Serialize for TwilioRequest {
    #[inline]
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let buffer = self.buffer.as_slice();
        let mut ser = serializer.serialize_map(Some(self.len))?;

        for (key, value) in form_urlencoded::parse(buffer) {
            ser.serialize_entry(key.as_ref(), value.as_ref())?;
        }

        ser.end()
    }
}

struct TwilioRequestVisitor;

impl<'de> Visitor<'de> for TwilioRequestVisitor {
    type Value = TwilioRequest;

    #[inline(always)]
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("TwilioRequest as sequence of key and value pairs or map")
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut result = TwilioRequest::new();

        //Cow - because deserialization might need decode special symbols
        while let Some(element) = map.next_entry::<Cow<'_, str>, Cow<'_, str>>()? {
            result.add_pair(element.0.as_ref(), element.1.as_ref());
        }

        Ok(result)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let mut result = TwilioRequest::new();

        //Cow - because deserialization might need decode special symbols
        while let Some(element) = seq.next_element::<(Cow<'_, str>, Cow<'_, str>)>()? {
            result.add_pair(element.0.as_ref(), element.1.as_ref());
        }

        Ok(result)
    }
}

impl<'de> Deserialize<'de> for TwilioRequest {
    #[inline]
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<TwilioRequest, D::Error> {
        deserializer.deserialize_any(TwilioRequestVisitor)
    }
}

impl<'a> Serialize for Mms<'a> {
    #[inline]
    fn serialize<S: serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut ser = serializer.serialize_struct("Sms", 3)?;

        ser.serialize_field("From", &self.sms.from)?;
        ser.serialize_field("To", &self.sms.to)?;
        ser.serialize_field("Body", &self.sms.body)?;
        ser.serialize_field("MediaUrl", &self.media_url)?;
        ser.end()
    }
}

impl<'a> Serialize for Sms<'a> {
    #[inline]
    fn serialize<S: serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut ser = serializer.serialize_struct("Sms", 3)?;

        ser.serialize_field("From", &self.from)?;
        ser.serialize_field("To", &self.to)?;
        ser.serialize_field("Body", &self.body)?;
        ser.end()
    }
}

impl<'a> Serialize for Call<'a> {
    #[inline]
    fn serialize<S: serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut ser = serializer.serialize_struct("Call", 3)?;

        ser.serialize_field("From", &self.from)?;
        ser.serialize_field("To", &self.to)?;
        match self.instruction {
            CallInstruction::Twiml(twiml) => ser.serialize_field("Twiml", twiml)?,
            CallInstruction::Url(url) => ser.serialize_field("Url", url)?,
        };
        ser.end()
    }
}
