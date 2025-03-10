// TODO: you have something to do in each of the modules in this crate!
mod description;
mod status;
mod title;

// A common pattern in Rust is to split code into multiple (private) modules
// and then re-export the public parts of those modules at the root of the crate.
//
// This hides the internal structure of the crate from your users, while still
// allowing you to organize your code however you like.
pub use description::TicketDescription;
pub use status::Status;
pub use title::TicketTitle;

// We no longer need to make the fields private!
// Since each field encapsulates its own validation logic, there is no risk of
// a user of `Ticket` modifying the fields in a way that would break the
// invariants of the struct.
//
// Careful though: if you had any invariants that spanned multiple fields, you
// would need to ensure that those invariants are still maintained and go back
// to making the fields private.
#[derive(Debug, PartialEq, Clone)]
pub struct Ticket {
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}


#[cfg(test)]
mod tests{
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn it_works(){
        let title = TicketTitle::try_from("A title".to_string()).unwrap();
        let description = TicketDescription::try_from("A description".to_string()).unwrap();
        let status = Status::try_from("ToDO".to_string()).unwrap();
        let ticket = Ticket {
            title,
            description,
            status,
        };
        assert_eq!(ticket.title.get_title(), "A title");
        assert_eq!(ticket.description.get_description(), "A description");
        assert_eq!(ticket.status, Status::ToDo);
    }
    #[test]
    #[should_panic]
    fn empty_title(){
        let _ = TicketTitle::try_from("".to_string()).unwrap();
    }
 
}