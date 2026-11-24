use ticket_management_system::store::models::{Status, TicketDraft, TicketPatch};
use ticket_management_system::launch;

#[tokio::test]
async fn works() {
    let client = launch(5).await;
    let draft = TicketDraft {
        title: String::from("This is titile"),
        description: String::from("We are showing the description"),
    };
    let ticket_id = client.insert(draft.clone()).await.unwrap();

    let ticket = client.get(ticket_id).await.unwrap().unwrap();
    assert_eq!(ticket_id, ticket.id);
    assert_eq!(ticket.status, Status::ToDo);
    assert_eq!(ticket.title, draft.title);
    assert_eq!(ticket.description, draft.description);

    let patch = TicketPatch {
        id: ticket_id,
        version_number: ticket.version_number,
        title: None,
        description: None,
        status: Some(Status::InProgress),
    };
    client.update(patch).await.unwrap();

    let ticket = client.get(ticket_id).await.unwrap().unwrap();
    assert_eq!(ticket.id, ticket_id);
    assert_eq!(ticket.status, Status::InProgress);
}
