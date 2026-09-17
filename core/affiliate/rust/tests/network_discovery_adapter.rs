use cat_affiliate::{AffiliateDomainResult, DiscoverySource, DiscoverySourceRequest, NetworkAdapter, NetworkConversion, NetworkDiscoveryAdapter, NetworkId, NetworkInfo, NetworkProgram};
use std::{future::Future, pin::Pin};

struct StubNetwork { info: NetworkInfo, programs: Vec<NetworkProgram> }
impl NetworkAdapter for StubNetwork {
    fn info(&self) -> &NetworkInfo { &self.info }
    fn list_programs(&self, _: Option<&str>, _: u32, _: u32) -> Pin<Box<dyn Future<Output=AffiliateDomainResult<Vec<NetworkProgram>>> + Send + '_>> { let programs=self.programs.clone(); Box::pin(async move { Ok(programs) }) }
    fn generate_link(&self, _: &str, _: &str, _: &str) -> Pin<Box<dyn Future<Output=AffiliateDomainResult<String>> + Send + '_>> { Box::pin(async { Ok("https://example.invalid/link".into()) }) }
    fn fetch_conversions(&self, _: i64, _: i64) -> Pin<Box<dyn Future<Output=AffiliateDomainResult<Vec<NetworkConversion>>> + Send + '_>> { Box::pin(async { Ok(vec![]) }) }
    fn validate_credentials(&self) -> Pin<Box<dyn Future<Output=AffiliateDomainResult<bool>> + Send + '_>> { Box::pin(async { Ok(true) }) }
}

fn block_on<F: Future>(future: F) -> F::Output {
    use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
    fn clone(_: *const ()) -> RawWaker { RawWaker::new(std::ptr::null(), &VTABLE) }
    fn wake(_: *const ()) {}
    fn wake_by_ref(_: *const ()) {}
    fn drop(_: *const ()) {}
    static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);
    let waker=unsafe { Waker::from_raw(RawWaker::new(std::ptr::null(), &VTABLE)) };
    let mut cx=Context::from_waker(&waker);
    let mut future=std::pin::pin!(future);
    loop { match future.as_mut().poll(&mut cx) { Poll::Ready(v)=>return v, Poll::Pending=>std::thread::yield_now() } }
}

fn program(rate: &str) -> NetworkProgram { NetworkProgram { external_id:"p1".into(), network_id:NetworkId("net-a".into()), name:"Widget".into(), merchant_name:"Acme".into(), commission_rate:rate.into(), cookie_days:30, categories:vec!["electronics".into()], url:"https://merchant.invalid/widget".into(), description:None, accepting_applications:true } }
fn network(programs: Vec<NetworkProgram>) -> StubNetwork { StubNetwork { info:NetworkInfo { id:NetworkId("net-a".into()), name:"Network A".into(), base_url:"https://network.invalid".into(), supports_real_time_reporting:false, supports_deep_linking:true, default_cookie_days:30 }, programs } }

#[test]
fn network_programs_are_ingested_as_candidates() {
    let network=network(vec![program("6%")]);
    let adapter=NetworkDiscoveryAdapter::new(&network);
    let result=block_on(adapter.discover(DiscoverySourceRequest { page:1, per_page:10, ..Default::default() })).unwrap();
    assert_eq!(result.candidates.len(),1);
    assert_eq!(result.candidates[0].source,"net-a");
    assert_eq!(result.candidates[0].commission_bps,Some(600));
    assert!(!result.has_more);
    assert_eq!(result.next_page,None);
}

#[test]
fn unsupported_filters_fail_before_network_call() {
    let network=network(vec![]);
    let adapter=NetworkDiscoveryAdapter::new(&network);
    let result=block_on(adapter.discover(DiscoverySourceRequest { page:1, per_page:10, currency:Some("EUR".into()), ..Default::default() }));
    assert!(result.is_err());
}
