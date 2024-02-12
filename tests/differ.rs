use differ_from_spec::DifferFromSpec;

#[derive(Debug, PartialEq, DifferFromSpec)]
pub struct RepositoryResponse {
    pub full_name: String,
    pub security_and_analysis: Option<SecurityAndAnalysisResponse>,
    pub delete_branch_on_merge: Option<bool>,
    pub allow_auto_merge: Option<bool>,
    pub allow_squash_merge: Option<bool>,
    pub allow_merge_commit: Option<bool>,
    pub allow_rebase_merge: Option<bool>,
    pub allow_update_branch: Option<bool>,
}

#[derive(Debug, PartialEq, DifferFromSpec, Default)]
pub struct SecurityAndAnalysisResponse {
    pub secret_scanning: Option<SecurityAndAnalysisStatusResponse>,
    pub secret_scanning_push_protection: Option<SecurityAndAnalysisStatusResponse>,
    pub dependabot_security_updates: Option<SecurityAndAnalysisStatusResponse>,
    pub secret_scanning_validity_checks: Option<SecurityAndAnalysisStatusResponse>,
}

#[derive(Debug, PartialEq, DifferFromSpec, Default, Eq, Hash)]
pub struct SecurityAndAnalysisStatusResponse {
    pub status: Status,
}

#[derive(Debug, PartialEq, DifferFromSpec, Default, Eq, Hash)]
pub enum Status {
    Enabled,
    #[default]
    Disabled,
}

impl Default for RepositoryResponse {
    fn default() -> Self {
        Self {
            full_name: "my-repo".into(),
            security_and_analysis: None,
            delete_branch_on_merge: None,
            allow_auto_merge: None,
            allow_squash_merge: None,
            allow_merge_commit: None,
            allow_rebase_merge: None,
            allow_update_branch: None,
        }
    }
}

#[test]
fn github_should_not_differ_simple_option() {
    let spec = RepositoryResponse {
        ..Default::default()
    };
    let actual = RepositoryResponse {
        delete_branch_on_merge: Some(true),
        ..Default::default()
    };
    assert_eq!(false, actual.differ_from_spec(&spec));
}

#[test]
fn github_should_not_differ_struct_option() {
    let spec = RepositoryResponse {
        ..Default::default()
    };
    let actual = RepositoryResponse {
        security_and_analysis: Some(SecurityAndAnalysisResponse {
            ..Default::default()
        }),
        ..Default::default()
    };
    assert_eq!(false, actual.differ_from_spec(&spec));
}

#[test]
fn github_should_not_differ_nested_struct_option() {
    let spec = RepositoryResponse {
        ..Default::default()
    };
    let actual = RepositoryResponse {
        security_and_analysis: Some(SecurityAndAnalysisResponse {
            secret_scanning: Some(SecurityAndAnalysisStatusResponse {
                status: Status::Enabled,
            }),
            ..Default::default()
        }),
        ..Default::default()
    };
    assert_eq!(false, actual.differ_from_spec(&spec));
}

#[test]
fn github_should_differ_simple_option() {
    let spec = RepositoryResponse {
        delete_branch_on_merge: Some(true),
        ..Default::default()
    };
    let actual = RepositoryResponse {
        delete_branch_on_merge: Some(false),
        ..Default::default()
    };
    assert!(actual.differ_from_spec(&spec));
}

#[test]
fn github_should_differ_simple_option_actual_is_none() {
    let spec = RepositoryResponse {
        delete_branch_on_merge: Some(true),
        ..Default::default()
    };
    let actual = RepositoryResponse {
        ..Default::default()
    };
    assert!(actual.differ_from_spec(&spec));
}

#[test]
fn github_should_differ_struct_option() {
    let spec = RepositoryResponse {
        security_and_analysis: Some(SecurityAndAnalysisResponse {
            secret_scanning: Some(SecurityAndAnalysisStatusResponse {
                status: Status::Enabled,
            }),
            ..Default::default()
        }),
        ..Default::default()
    };
    let actual = RepositoryResponse {
        security_and_analysis: Some(SecurityAndAnalysisResponse {
            secret_scanning: Some(SecurityAndAnalysisStatusResponse {
                status: Status::Disabled,
            }),
            ..Default::default()
        }),
        ..Default::default()
    };
    assert!(actual.differ_from_spec(&spec));
}

#[test]
fn github_should_differ_struct_option_actual_none() {
    let spec = RepositoryResponse {
        security_and_analysis: Some(SecurityAndAnalysisResponse {
            secret_scanning: Some(SecurityAndAnalysisStatusResponse {
                status: Status::Enabled,
            }),
            ..Default::default()
        }),
        ..Default::default()
    };
    let actual = RepositoryResponse {
        security_and_analysis: Some(SecurityAndAnalysisResponse {
            ..Default::default()
        }),
        ..Default::default()
    };
    assert!(actual.differ_from_spec(&spec));
}

#[test]
fn vector_should_not_differ() {
    let spec = vec![
        SecurityAndAnalysisStatusResponse {
            status: Status::Enabled,
        },
        SecurityAndAnalysisStatusResponse {
            status: Status::Disabled,
        },
    ];
    let actual = vec![
        SecurityAndAnalysisStatusResponse {
            status: Status::Disabled,
        },
        SecurityAndAnalysisStatusResponse {
            status: Status::Enabled,
        },
    ];
    assert_eq!(false, actual.differ_from_spec(&spec));
}

#[test]
fn vector_should_not_differ_different_order() {
    let spec = vec![Status::Enabled, Status::Disabled];
    let actual = vec![Status::Disabled, Status::Enabled];
    assert_eq!(false, actual.differ_from_spec(&spec));
}

#[test]
fn vector_should_differ() {
    let spec = vec![Status::Enabled, Status::Disabled];
    let actual = vec![Status::Enabled];
    assert_eq!(true, actual.differ_from_spec(&spec));
}
