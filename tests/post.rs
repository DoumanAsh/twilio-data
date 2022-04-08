//use twilio_data::TwilioRequest;
//
//const ACCOUNT_SID: &str = "<account sid>";
//const ACCOUNT_TOKEN: &str = "<account token>";
//
//#[test]
//fn should_sms() {
//    const URL: twilio_data::UrlBuffer = twilio_data::sms_resource_url(ACCOUNT_SID);
//    let auth = format!("Basic {}", base64::encode(format!("{}:{}", ACCOUNT_SID, ACCOUNT_TOKEN)));
//
//    let mut req = TwilioRequest::new();
//    req.from("+12184758724")
//       .to("+817039769774")
//       .body("Testing SMS");
//
//    println!("req: {}", req.as_form());
//    let res = ureq::post(&URL).set("Authorization", auth.as_str())
//                              .set("Content-Type", "application/x-www-form-urlencoded")
//                              .set("Accept", "application/json")
//                              .send_string(req.as_form());
//
//    match res {
//        Ok(res) => {
//            let res: twilio_data::SmsResult = res.into_json().expect("Get response");
//            println!("res={:#?}", res);
//            assert_eq!(res.from, "+12184758724");
//            assert_eq!(res.to, "+817039769774");
//        },
//        Err(ureq::Error::Status(_status, resp)) => {
//            panic!("Error: {:?}", resp.into_json::<twilio_data::TwilioError>());
//        },
//        Err(ureq::Error::Transport(err)) => {
//            panic!("{}", err);
//        },
//    }
//}
//
//#[test]
//fn should_call() {
//    const URL: twilio_data::UrlBuffer = twilio_data::call_resource_url(ACCOUNT_SID);
//    let auth = format!("Basic {}", base64::encode(format!("{}:{}", ACCOUNT_SID, ACCOUNT_TOKEN)));
//
//    let mut req = TwilioRequest::new();
//    req.from("+12184758724")
//       .to("+817039769774")
//       .caller_id("+12184758724")
//       .twiml("<Response><Say>Ahoy</Say></Response>");
//
//    println!("req: {}", req.as_form());
//    let res = ureq::post(&URL).set("Authorization", auth.as_str())
//                              .set("Content-Type", TwilioRequest::CONTENT_TYPE)
//                              .set("Accept", "application/json")
//                              .send_string(req.as_form());
//
//    match res {
//        Ok(res) => {
//            let res: twilio_data::CallResult = res.into_json().expect("Get response");
//            println!("res={:#?}", res);
//            assert_eq!(res.from, "+12184758724");
//            assert_eq!(res.to, "+817039769774");
//        },
//        Err(ureq::Error::Status(_status, resp)) => {
//            panic!("Error: {:?}", resp.into_json::<twilio_data::TwilioError>());
//        },
//        Err(ureq::Error::Transport(err)) => {
//            panic!("{}", err);
//        },
//    }
//}
