use cat_content::{
    ContentDomain, ContentDomainError, ContentKind, ContentStatus, InMemoryContentRepository,
    PublicationPlan, PublicationState, PublicationTarget,
};

fn approved_record() -> cat_content::ContentRecord {
    let mut repo = InMemoryContentRepository::default();
    let record = ContentDomain::create(
        &mut repo,
        ContentKind::Article,
        "Publication contract",
        "canonical content",
        1,
    )
    .unwrap();
    ContentDomain::submit_for_review(&mut repo, record.id).unwrap();
    ContentDomain::approve(&mut repo, record.id).unwrap()
}

#[test]
fn publication_plan_is_derived_from_approved_content() {
    let record = approved_record();
    assert_eq!(record.status, ContentStatus::Approved);

    let plan = PublicationPlan::from_record(&record, PublicationTarget::Web).unwrap();
    assert_eq!(plan.content_id, record.id);
    assert_eq!(plan.content_version, 1);
    assert_eq!(plan.target, PublicationTarget::Web);

    let publication = plan.materialize();
    assert_eq!(publication.state, PublicationState::Planned);
    assert_eq!(publication.publication_id, plan.publication_id);
}

#[test]
fn publication_plan_rejects_unapproved_content() {
    let mut repo = InMemoryContentRepository::default();
    let record = ContentDomain::create(
        &mut repo,
        ContentKind::Article,
        "Draft",
        "not approved",
        1,
    )
    .unwrap();

    assert_eq!(
        PublicationPlan::from_record(&record, PublicationTarget::Web),
        Err(ContentDomainError::InvalidState(
            "publication planning requires approved or published content",
        ))
    );
}

#[test]
fn publication_state_machine_is_forward_only_for_success() {
    let record = approved_record();
    let mut publication = PublicationPlan::from_record(&record, PublicationTarget::AgentFeed)
        .unwrap()
        .materialize();

    publication.submit().unwrap();
    assert_eq!(publication.state, PublicationState::Submitted);

    publication.confirm().unwrap();
    assert_eq!(publication.state, PublicationState::Confirmed);

    assert_eq!(
        publication.submit(),
        Err(ContentDomainError::InvalidState(
            "publication submission requires planned state",
        ))
    );
}

#[test]
fn failed_or_cancelled_publications_are_terminal() {
    let record = approved_record();

    let mut failed = PublicationPlan::from_record(&record, PublicationTarget::Mobile)
        .unwrap()
        .materialize();
    failed.fail().unwrap();
    assert_eq!(failed.state, PublicationState::Failed);
    assert_eq!(
        failed.confirm(),
        Err(ContentDomainError::InvalidState(
            "publication confirmation requires submitted state",
        ))
    );

    let mut cancelled = PublicationPlan::from_record(&record, PublicationTarget::Email)
        .unwrap()
        .materialize();
    cancelled.submit().unwrap();
    cancelled.cancel().unwrap();
    assert_eq!(cancelled.state, PublicationState::Cancelled);
}
