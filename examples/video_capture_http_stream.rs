/// Example code to stream mjpeg over a tcp socket
use std::io::Write;
use std::net::{SocketAddr, TcpListener};

use opencv::core::{Mat, Vector};
use opencv::imgcodecs::{imencode, IMWRITE_JPEG_QUALITY};
use opencv::videoio::{VideoCapture, VideoCaptureTrait};
use opencv::Result;

const BASE_RESPONSE: &[u8] = b"HTTP/1.1 200 OK\r\nContent-Type: multipart/x-mixed-replace; boundary=frame\r\n\r\n";

fn main() -> Result<(), Box<dyn std::error::Error>> {
	// Select camera
	let mut cam = VideoCapture::new_def(0)?;

	// Bind listener to a port
	let address: SocketAddr = "127.0.0.1:8080".parse()?;
	let listener = TcpListener::bind(address)?;
	println!("Listening for connections at {}", address.to_string());

	// Accept the first incoming connection
	let (mut stream, addr) = listener.accept()?;
	println!("Client connected: {}", addr);

	// Write intial response
	stream.write_all(BASE_RESPONSE)?;

	// Reduce jpeg quality for streaming
	let encode_params = Vector::from_slice(&[IMWRITE_JPEG_QUALITY, 70]);
	let mut buffer = Mat::default();
	let mut frame = Vector::default();
	loop {
		// Read frame from the camera & encode it
		cam.read(&mut buffer)?;
		imencode(".jpg", &buffer, &mut frame, &encode_params)?;

		// Construct HTTP frame and send it to the peer
		let header = format!(
			"--frame\r\nContent-Type: image/jpeg\r\nContent-Length: {}\r\n\r\n",
			frame.len()
		);
		let packet = [header.as_bytes(), frame.as_slice()].concat();
		stream.write_all(&packet)?;
	}
}
