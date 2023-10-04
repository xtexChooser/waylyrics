use gtk::{prelude::*, Window};

pub fn show_dialog(parent: Option<&impl IsA<Window>>, msg: &str, level: gtk::MessageType) {
    let msg_dialog = gtk::MessageDialog::new(
        parent,
        gtk::DialogFlags::MODAL,
        level,
        gtk::ButtonsType::Ok,
        msg,
    );
    msg_dialog.connect_response(|dialog, _| {
        dialog.close();
    });
    msg_dialog.present();
}
