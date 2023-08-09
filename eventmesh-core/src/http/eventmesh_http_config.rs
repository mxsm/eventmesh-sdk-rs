
#[derive(Debug,Clone,Copy)]
pub struct HttpConfig {
    pub lite_eventmesh_addr: String,
    pub load_balance_type: LoadBalanceType,
    pub consume_thread_core: i32,
    pub consume_thread_max: i32,
    pub env: String,
    pub consumer_group: String,
    pub producer_group: String,
    pub idc: String,
    pub ip: String,
    pub pid: String,
    pub sys: String,
    pub user_name: String,
    pub password: String,
    pub use_tls: bool,
    pub ssl_client_protocol: String,
    pub max_connection_pool_size: i32,
    pub connection_idle_time_seconds: i32,
}

impl HttpConfig {
    fn new() -> Self{
        Self::default()
    }
}

impl Default for HttpConfig{
    fn default() -> Self {
        Self{
            lite_eventmesh_addr: String::from("localhost:10105"),
            load_balance_type: LoadBalanceType::Random,
            consume_thread_core: 2,
            consume_thread_max: 5,
            env: String::from(""),
            consumer_group: String::from("DefaultConsumerGroup"),
            producer_group: String::from("DefaultProducerGroup"),
            idc: String::from(""),
            ip: String::from("localhost"),
            pid: String::from(""),
            sys: String::from(""),
            user_name: String::from(""),
            password: String::from(""),
            use_tls: false,
            ssl_client_protocol: String::from("TLSv1.2"),
            max_connection_pool_size: 30,
            connection_idle_time_seconds: 10,
        }
    }
}

pub enum LoadBalanceType {
    Random,
    WeightRoundRobin,
    WeightRandom
}