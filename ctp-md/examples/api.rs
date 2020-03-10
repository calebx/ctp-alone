use ctp_common::sleep;
use ctp_md::*;
use std::ffi::CString;

struct Spi;
impl MdSpi for Spi {}

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

fn new_logout() -> CThostFtdcUserLogoutField {
    let mut f: CThostFtdcUserLogoutField = Default::default();

    let broker_id = "9999";
    let user_id = &std::env::var("CTP_USER_ID").unwrap();

    set_cstr_from_str_truncate(&mut f.BrokerID, broker_id);
    set_cstr_from_str_truncate(&mut f.UserID, user_id);
    f
}

fn main() {
    let mut current_request_id = 0;
    let flow_path = CString::new("").unwrap();
    let mut md_api = MdApi::new(flow_path, false, false);

    md_api.register_spi(Box::new(Spi));
    md_api.register_front(CString::new("tcp://180.168.146.187:10130").unwrap());
    md_api.init();

    sleep(2);
    println!("=== try req_user_login");
    current_request_id += 1;
    match md_api.req_user_login(&new_login(), current_request_id) {
        Ok(()) => println!("req_user_login ok"),
        Err(err) => println!("req_user_login err: {:?}", err),
    };

    sleep(2);
    println!("=== try subscribe_market_data");
    let instrument_ids = vec![
        CString::new("IF1703").unwrap(),
        CString::new("au1712").unwrap(),
        CString::new("m1709").unwrap(),
        CString::new("CF709").unwrap(),
    ];
    match md_api.subscribe_market_data(&instrument_ids.clone()) {
        Ok(()) => println!("subscribe_market_data ok"),
        Err(err) => println!("subscribe_market_data err: {:?}", err),
    };

    sleep(2);
    println!("=== try subscribe_for_quote_rsp");
    match md_api.subscribe_for_quote_rsp(&instrument_ids.clone()) {
        Ok(()) => println!("subscribe_for_quote_rsp ok"),
        Err(err) => println!("subscribe_for_quote_rsp err: {:?}", err),
    };

    sleep(5);
    println!("=== try req_user_logout");
    current_request_id += 1;
    match md_api.req_user_logout(&new_logout(), current_request_id) {
        Ok(()) => println!("req_user_logout ok"),
        Err(err) => println!("req_user_logout err: {:?}", err),
    };

    sleep(1);
    println!("=== THE END");
}
