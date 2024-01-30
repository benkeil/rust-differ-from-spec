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
    struct DemoSub {
        pub team: Option<String>,
        pub sub: Option<DemoSubSub>,
    }

    #[derive(DifferFromSpec, PartialEq)]
    struct DemoSubSub {
        pub name: Option<String>,
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

    #[test]
    fn should_not_differ() {
        let spec = Demo {
            name: "foo".into(),
            team: Some("bar".into()),
            enabled: Some(true),
            count: Some(1),
            sub: Some(DemoSub {
                team: Some("bar".into()),
                sub: None,
            }),
            status: None,
        };
        let actual = Demo {
            name: "foo".into(),
            team: Some("bar".into()),
            enabled: Some(true),
            count: Some(1),
            sub: Some(DemoSub {
                team: Some("bar".into()),
                sub: Some(DemoSubSub {
                    name: Some("subsub".into()),
                }),
            }),
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
            sub: Some(DemoSub {
                team: Some("xxx".into()),
                sub: Some(DemoSubSub {
                    name: Some("subsub".into()),
                }),
            }),
            status: Some(DemoEnum::Bar),
        };
        let actual = Demo {
            name: "foo".into(),
            team: Some("bar".into()),
            enabled: Some(true),
            count: Some(1),
            sub: Some(DemoSub {
                team: Some("bar".into()),
                sub: Some(DemoSubSub {
                    name: Some("subsub".into()),
                }),
            }),
            status: Some(DemoEnum::Bar),
        };
        assert!(actual.differ_from_spec(&spec));
    }
}
