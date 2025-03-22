use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;
use pdf_writer::{Pdf, Rect, Ref, Content, Name, Finish};
use pdf_writer::writers::Page;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

// print_pdf version
#[wasm_bindgen]
pub fn create_letter(sender: JsValue, recipient: JsValue) -> Result<JsValue, JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    let sender_str: String = serde_wasm_bindgen::from_value(sender)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    log::info!("{}", sender_str);
    let _recipient_str: String = serde_wasm_bindgen::from_value(recipient)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    // Define an ID allocator. Every time we need a new object, we just call
    // `alloc.bump()`, which increases `alloc` by one and returns its previous
    // value.
    let mut alloc = Ref::new(1);
    let mut pdf = Pdf::new();

    // We'll collect the page IDs here.
    let page_tree_id = alloc.bump();
    let mut page_ids = vec![];

    let address_page_id = alloc.bump();
    page_ids.push(address_page_id);
    let mut address_page = pdf.page(page_ids[0]);
    address_page.parent(page_tree_id);
    address_page.media_box(Rect::new(0.0, 0.0, 595.0, 842.0));

    let sender_content_id = alloc.bump();
    address_page.contents(sender_content_id);
    address_page.finish();

    let mut sender_content = Content::new();
    sender_content.begin_text();
    sender_content.set_font(Name(b"Helvetica"), 14.0);
    sender_content.next_line(108.0, 734.0);
    sender_content.
    sender_content.show(pdf_writer::TextStr("Oderstraße!"));
    sender_content.end_text();
    pdf.stream(sender_content_id, &sender_content.finish());

    // Finish up
    // Write the root of the page tree.
    pdf.pages(page_tree_id)
        .kids(page_ids.iter().copied())
        .count(page_ids.len() as i32);
    // Write the document catalog.
    pdf.catalog(alloc.bump()).pages(page_tree_id);

    let output = ByteBuf::from(pdf.finish());
    // Ok(serde_wasm_bindgen::to_value(&pdf_bytes)?)
    Ok(serde_wasm_bindgen::to_value(&output)
        .map_err(|e| JsValue::from_str(&e.to_string()))?)
}