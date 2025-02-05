use wasm_bindgen::prelude::*;
// use printpdf::*;
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;
use pdf_writer::{Content, Name, Pdf, Rect, Ref, Str};
// use printpdf::*;
// use printpdf::path::{PaintMode, WindingOrder};
use wasm_bindgen::prelude::*;
use std::io::Write;
use std::ptr::write;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

// #[wasm_bindgen]
// pub fn create_letter(sender: JsValue, recipient: JsValue) -> Result<JsValue, ()> {
//     let sender_str: &str = serde_wasm_bindgen::from_value(sender)?;
//     let recipient_str = serde_wasm_bindgen::from_value(recipient);
//     let (doc, page1, layer1) = PdfDocument::new("PDF_Document_title", Mm(247.0), Mm(210.0), "Layer 1");
//     let (page2, layer1) = doc.add_page(Mm(10.0), Mm(250.0),"Page 2, Layer 1");
//
//     let pdf_bytes = doc.save_to_bytes().unwrap();
//     Ok(serde_wasm_bindgen::to_value(&pdf_bytes)?)
// }

#[wasm_bindgen]
pub fn create_letter(sender: JsValue, recipient: JsValue) -> Result<JsValue, JsValue> {
    let sender_str: String = serde_wasm_bindgen::from_value(sender)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    let recipient_str: String = serde_wasm_bindgen::from_value(recipient)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    let catalog_id = Ref::new(1);
    let page_tree_id = Ref::new(2);
    let page_id = Ref::new(3);
    let font_id = Ref::new(4);
    let content_id = Ref::new(5);

    let font_name = Name(b"F1");

    // Create a new PDF document
    let mut pdf = Pdf::new();

    pdf.catalog(catalog_id).pages(page_tree_id);
    pdf.pages(page_tree_id).kids([page_id]).count(1);
    pdf.page(page_id)
        .parent(page_tree_id)
        .media_box(Rect::new(0.0, 0.0, 595.0, 842.0))
        .contents(content_id)
        .resources().fonts().pair(font_name, font_id);

    let mut content = Content::new();
    content.begin_text();
    content.set_font(font_name, 14.0);
    content.next_line(108.0, 734.0);
    content.show(Str(b"Hello World from Rust!"));
    content.end_text();
    pdf.stream(content_id, &content.finish());


    // Get the PDF bytes
    let bytes = pdf.finish();

    let output = ByteBuf::from(bytes);

    // Convert to JsValue and return
    Ok(serde_wasm_bindgen::to_value(&output)
        .map_err(|e| JsValue::from_str(&e.to_string()))?)
}