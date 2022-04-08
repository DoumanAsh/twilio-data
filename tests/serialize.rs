use twilio_data::{TwilioRequest, Mms, Sms, Call, CallInstruction};

#[test]
fn should_serde_request() {
    const EXPECTED: &str = "From=LOLKA&To=Me&Body=My+cute+text";
    let result: TwilioRequest = serde_urlencoded::from_str(EXPECTED).expect("To parse");
    assert_eq!(result.as_form(), EXPECTED);

    let mut req = TwilioRequest::new();
    req.from("LOLKA");
    req.to("Me");
    req.body("My cute text");
    assert_eq!(serde_urlencoded::to_string(req).unwrap(), EXPECTED);
}

#[test]
fn should_fmt_sms() {
    const EXPECTED: &str = "From=LOLKA&To=Me&Body=My+cute+text";
    let sms = Sms {
        from: "LOLKA",
        to: "Me",
        body: "My cute text",
    };

    let raw = sms.request();
    assert_eq!(sms.to_string(), EXPECTED);
    assert_eq!(serde_urlencoded::to_string(sms).unwrap(), EXPECTED);
    assert_eq!(raw.as_form(), EXPECTED);
}

#[test]
fn should_fmt_mms() {
    const EXPECTED: &str = "From=LOLKA&To=Me&Body=My+cute+text&MediaUrl=test.png";
    let mms = Mms {
        sms: Sms {
            from: "LOLKA",
            to: "Me",
            body: "My cute text",
        },
        media_url: "test.png"
    };

    let raw = mms.request();
    assert_eq!(mms.to_string(), EXPECTED);
    assert_eq!(serde_urlencoded::to_string(mms).unwrap(), EXPECTED);
    assert_eq!(raw.as_form(), EXPECTED);
}

#[test]
fn should_fmt_call_with_url() {
    const EXPECTED: &str = "From=LOLKA&To=Me&Url=https%3A%2F%2Fdomain.com%2Ftest.xml";
    let call = Call {
        from: "LOLKA",
        to: "Me",
        instruction: CallInstruction::Url("https://domain.com/test.xml"),
    };

    let raw = call.request();
    assert_eq!(call.to_string(), EXPECTED);
    assert_eq!(serde_urlencoded::to_string(call).unwrap(), EXPECTED);
    assert_eq!(raw.as_form(), EXPECTED);
}

#[test]
fn should_fmt_call_with_xml() {
    const EXPECTED: &str = "From=LOLKA&To=Me&Twiml=%3CResponse%3E%3CSay%3EAhoy%3C%2FSay%3E%3C%2FResponse%3E";
    let call = Call {
        from: "LOLKA",
        to: "Me",
        instruction: CallInstruction::Twiml("<Response><Say>Ahoy</Say></Response>"),
    };

    let raw = call.request();
    assert_eq!(call.to_string(), EXPECTED);
    assert_eq!(serde_urlencoded::to_string(call).unwrap(), EXPECTED);
    assert_eq!(raw.as_form(), EXPECTED);
}
