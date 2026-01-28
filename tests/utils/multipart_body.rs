use axum::body::Body;

pub fn build_multipart_body(fields: &[(&str, &str)]) -> (String, Body) {
	let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
	let mut body = String::new();
	for (name, value) in fields {
		body.push_str(&format!("--{}\r\n", boundary));
		body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", name));
		body.push_str(value);
		body.push_str("\r\n");
	}
	body.push_str(&format!("--{}--\r\n", boundary));

	let content_type = format!("multipart/form-data; boundary={}", boundary);
	(content_type, Body::from(body))
}
