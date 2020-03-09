use ctp_trader::*;

struct Spi;
impl TraderSpi for Spi {}

fn new_authenticate() -> CThostFtdcReqAuthenticateField {
    let mut f: CThostFtdcReqAuthenticateField = Default::default();
    
    let app_id = "simnow_client_test";
    let auth_code = "0000000000000000";
    let broker_id = "9999";
    let user_id = &std::env::var("CTP_USER_ID").unwrap();

    set_cstr_from_str_truncate(&mut f.BrokerID, broker_id);
    set_cstr_from_str_truncate(&mut f.UserID, user_id);
    set_cstr_from_str_truncate(&mut f.AuthCode, auth_code);
    set_cstr_from_str_truncate(&mut f.AppID, app_id);
    f
}

fn new_login() -> CThostFtdcReqUserLoginField {
    let mut f: CThostFtdcReqUserLoginField = Default::default();

    let broker_id = "9999";
    let user_id = &std::env::var("CTP_USER_ID").unwrap();
    let password = &std::env::var("CTP_PASSWORD").unwrap();

    set_cstr_from_str_truncate(&mut f.BrokerID, broker_id);
    set_cstr_from_str_truncate(&mut f.UserID, user_id);
    set_cstr_from_str_truncate(&mut f.Password, password);
    f
}

fn new_password() -> CThostFtdcUserPasswordUpdateField {
    let mut f: CThostFtdcUserPasswordUpdateField = Default::default();

    let broker_id = "9999";
    let user_id = &std::env::var("CTP_USER_ID").unwrap();
    let old_password = &std::env::var("CTP_PASSWORD").unwrap();
    let new_password = &std::env::var("CTP_NEW_PASSWORD").unwrap();

    set_cstr_from_str_truncate(&mut f.BrokerID, broker_id);
    set_cstr_from_str_truncate(&mut f.UserID, user_id);
    set_cstr_from_str_truncate(&mut f.OldPassword, old_password);
    set_cstr_from_str_truncate(&mut f.NewPassword, new_password);
    f
}

fn main() {
    let mut current_request_id = 0;
    let flow_path = std::ffi::CString::new("").unwrap();
    let mut trader_api = TraderApi::new(flow_path);

    trader_api.register_spi(Box::new(Spi));
    trader_api.register_front(std::ffi::CString::new("tcp://180.168.146.187:10130").unwrap());
    trader_api.subscribe_private_topic(ResumeType::Quick);
    trader_api.subscribe_public_topic(ResumeType::Quick);
    trader_api.init();
    
    sleep(2);
    current_request_id += 1;
    match trader_api.req_authenticate(&new_authenticate(), current_request_id) {
        Ok(()) => println!("req_authenticate ok"),
        Err(err) => println!("req_authenticate err: {:?}", err),
    };

    sleep(2);
    current_request_id += 1;
    match trader_api.req_user_login(&new_login(), current_request_id) {
        Ok(()) => println!("req_user_login ok"),
        Err(err) => println!("req_user_login err: {:?}", err),
    };

    current_request_id += 1;
    match trader_api.req_user_password_update(&new_password(), current_request_id) {
        Ok(()) => println!("req_user_password_update ok"),
        Err(err) => println!("req_user_password_update err: {:?}", err),
    };

    sleep(2);
    println!("=== THE END");
}
