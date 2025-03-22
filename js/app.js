import * as wasm from "rust_lib";

document.getElementById('addressForm').addEventListener('submit', function(e) {
    e.preventDefault();  // Prevent form from submitting normally

    // Get the values from textareas
    const senderAddress = document.getElementById('sender_address').value;
    const recipientAddress = document.getElementById('recipient_address').value;

    // Create content for the file
    const content = `Sender Address:\n${senderAddress}\n\nRecipient Address:\n${recipientAddress}`;

    let pdf_bytes = wasm.create_letter(senderAddress, recipientAddress)

    // Create a Blob containing the file content
    const blob = new Blob([pdf_bytes], { type: "application/pdf" });

    // Create a URL for the Blob
    const url = window.URL.createObjectURL(blob);

    // Open in new tab
    window.open(url, '_blank');

    // Create a temporary link element
    // const link = document.createElement('a');
    // link.href = url;
    // link.download = 'addresses.txt'; // Name of the file to be downloaded
    //
    // // Append link to body, click it, and remove it
    // document.body.appendChild(link);
    // link.click();
    // document.body.removeChild(link);
    //
    // // Clean up the URL object
    // window.URL.revokeObjectURL(url);
});