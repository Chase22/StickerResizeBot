extern crate futures;
extern crate telegram_bot;
extern crate tokio_core;

use futures::Stream;
use futures::Future;
use tokio_core::reactor::Core;
use telegram_bot::*;

fn main() {
    let mut core = Core::new().unwrap();

    let token = "340581395:AAFtbKSPPt25sCRiPWg7fP66o_zE1U9U-aA";
    let api = Api::configure(token).build(core.handle()).unwrap();

    // Fetch new updates via long poll method
    let future = api.stream().for_each(|update| {


        // If the received update contains a new message...
        if let UpdateKind::Message(message) = update.kind {

            if let MessageKind::Document{ref data, ..} = message.kind {
                api.spawn(SendMessage::new(message.chat.id(), "Converting..."));
                handle_file(&data, &message.chat.id(), &api);
            } else if let MessageKind::Text {..} = message.kind {
                api.spawn(SendMessage::new(message.chat.id(), "Plese send me a Picture as a File"));
            }  else if let MessageKind::Photo {..} = message.kind {
                api.spawn(SendMessage::new(message.chat.id(), "Plese send me the Picture as a File"));
            }
        }

        Ok(())
    });

    core.run(future).unwrap();
}

fn handle_file(doc: &Document, chat_id: &ChatId, api: &Api) {
    //println!("{:?}", doc);

    if doc.file_size.unwrap() > 20000000 {
        api.spawn(SendMessage::new(chat_id, "The file is to big too download. Please send at maximum 20MB files"));
    } else {
        //let file: File;

        let future = api.send(GetFile::new(doc));
        future.and_then(|file| Ok(println!("{:?}", file)));

        //Err(e) => { api.spawn(SendMessage::new(chat_id, "There was an error downloading the file")) },
    };

}