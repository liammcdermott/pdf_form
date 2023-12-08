use pdf_form::Form;
use std::process;

fn main() {
    // Load the pdf into a form from a path
    let mut form = Form::load("examples/demo-form.pdf").unwrap_or_else(|err| {
        eprintln!("Failed to load the form: {}", err);
        process::exit(1)
    });

    let field_names = form.get_all_names();
    eprintln!("Form element names: {:#?}", field_names);

    eprintln!("Radio state: {:#?}", form.get_state(3));
    let _ = form.set_radio(3, "Choice2".to_string()).or_else(|err| {
        eprintln!("Failed to update radio: {}", err);
        Err(err)
    });
    eprintln!("Radio state (after update): {:#?}", form.get_state(3));

    let _ = form.set_check_box(2, true).or_else(|err| {
        eprintln!("Failed to update check box: {}", err);
        Err(err)
    });

    let _ = match form.set_text(0, String::from("some text in a text field")) {
        Ok(..) => form.save("examples/demo-form-output.pdf").or_else(|err| {
            eprintln!("Failed to save output PDF: {}", err);
            Err(err)
        }),
        Err(err) => {
            eprintln!("Failed to fill the PDF form: {}", err);
            // Convert `ValueError` to `std::io::Error` or a common error type
            let io_error = std::io::Error::new(std::io::ErrorKind::Other, err.to_string());
            Err(io_error)
        }
    };
}
