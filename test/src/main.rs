fn main() {}

#[cfg(test)]
mod tests {
    use differ_from_spec::DifferFromSpec;

    #[derive(DifferFromSpec, PartialEq)]
    struct Demo {
        pub name: String,
        pub team: Option<String>,
        pub enabled: Option<bool>,
        pub count: Option<u8>,
        pub sub: Option<DemoSub>,
        pub status: Option<DemoEnum>,
    }

    #[derive(DifferFromSpec, PartialEq)]
    enum DemoEnum {
        Foo,
        Bar,
    }

    impl Default for Demo {
        fn default() -> Self {
            Self {
                name: "foo".into(),
                team: None,
                enabled: None,
                count: None,
                sub: None,
                status: None,
            }
        }
    }

    #[derive(DifferFromSpec, PartialEq)]
    struct DemoSub {
        pub team: String,
    }

    #[test]
    fn should_not_differ() {
        let spec = Demo {
            name: "foo".into(),
            team: Some("bar".into()),
            enabled: Some(true),
            count: Some(1),
            sub: None,
            status: None,
        };
        let actual = Demo {
            name: "foo".into(),
            team: Some("bar".into()),
            enabled: Some(true),
            count: Some(1),
            sub: Some(DemoSub { team: "bar".into() }),
            status: Some(DemoEnum::Foo),
        };
        assert!(!actual.differ_from_spec(&spec));
    }

    #[test]
    fn should_differ() {
        let spec = Demo {
            name: "foo".into(),
            team: Some("bar".into()),
            enabled: Some(true),
            count: Some(1),
            sub: Some(DemoSub { team: "xxx".into() }),
            status: Some(DemoEnum::Bar),
        };
        let actual = Demo {
            name: "foo".into(),
            team: Some("bar".into()),
            enabled: Some(true),
            count: Some(1),
            sub: Some(DemoSub { team: "bar".into() }),
            status: None,
        };
        assert!(actual.differ_from_spec(&spec));
    }
}
