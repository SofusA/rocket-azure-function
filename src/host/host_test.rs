#[cfg(test)]
mod host_tests {
    use crate::host::host::{Binding, Host, AuthLevel, TriggerType, Method, Direction};

    #[test]
    fn host_serialisation_test() {
        let host = Host {
            bindings: vec![Binding {
                auth_level: AuthLevel::Anonymous,
                trigger_type: TriggerType::HttpTrigger,
                direction: Direction::In,
                name: "test".to_string(),
                methods: vec![Method::Get],
           },] 
        };

        let serialised = serde_json::to_string(&host).expect("Unable to serialise");

        print!("{:?}", serialised);

        assert_eq!(serialised, "hej".to_string())
    }
}
