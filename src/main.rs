use ticket_management_system::launch;
use ticket_management_system::store::models::{Status, TicketDraft, TicketPatch};

#[tokio::main]
async fn main(){
  let client = launch(5).await;

 let draft = TicketDraft {
  title: String::from("This is titile"),
  description: String::from("We are showing the description"),
 };
 let ticket_id = client.insert(draft.clone()).await.unwrap();

 let ticket = client.get(ticket_id).await.unwrap().unwrap();

 println!("Ticket:- {:#?}", ticket);


 let patch = TicketPatch {
  id: ticket_id,
  version_number: ticket.version_number,
  title: Some("Updated new Title".to_string()),
  description: None,
  status: Some(Status::InProgress),
 };
 client.update(patch).await.unwrap();

 let ticket = client.get(ticket_id).await.unwrap().unwrap();

 println!("updated Ticket:- {:#?}", ticket);
}
