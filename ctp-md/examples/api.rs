use ctp_md::*;
use std::ffi::CString;

struct Spi;
impl MdSpi for Spi {}

fn new_login() -> CThostFtdcReqUserLoginField {
    let mut f: CThostFtdcReqUserLoginField = Default::default();

    let broker_id = &std::env::var("CTP_BROKER_ID").unwrap();
    let user_id = &std::env::var("CTP_USER_ID").unwrap();
    let password = &std::env::var("CTP_PASSWORD").unwrap();
    set_cstr_from_str_truncate(&mut f.BrokerID, broker_id);
    set_cstr_from_str_truncate(&mut f.UserID, user_id);
    set_cstr_from_str_truncate(&mut f.Password, password);
    f
}

fn main() {
    let mut request_id = 0;
    let mut md_api = MdApi::new(CString::new("").unwrap(), false, false);
    md_api.register_spi(Box::new(Spi));
    md_api.register_front(CString::new(std::env::var("CTP_MD_URL").unwrap()).unwrap());
    md_api.init();

    sleep(2);
    println!("=== try req_user_login");
    request_id += 1;
    match md_api.req_user_login(&new_login(), request_id) {
        Ok(_) => println!("req_user_login ok"),
        Err(err) => println!("req_user_login err: {:?}", err),
    };

    sleep(2);
    println!("=== try subscribe_market_data");
    let instrument_ids = vec![
        CString::new("cu2003").unwrap(),
        CString::new("cu2004").unwrap(),
        CString::new("au2006").unwrap(),
        CString::new("au2008").unwrap(),
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

    sleep(100);
    println!("=== THE END");
}
