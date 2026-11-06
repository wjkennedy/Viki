use serde_json::{Value, json};
use std::{fs, path::Path};
use tiny_http::{Header, Response, Server};
mod intake;

fn main() {
    fs::create_dir_all("data").unwrap();
    let path = Path::new("data/notes.json");
    if !path.exists() {
        fs::write(path, include_str!("../assets/seed.json")).unwrap();
    }
    if std::env::args().nth(1).as_deref() == Some("--local-call") {
        let mut input = String::new();
        std::io::stdin().read_to_string(&mut input).unwrap();
        let result = (|| -> Result<Value, String> {
            let v: Value = serde_json::from_str(&input).map_err(|e| e.to_string())?;
            match v["tool"].as_str().unwrap_or("") {
                "discover_documents" => intake::run(&v["arguments"], false, path),
                "import_documents" => intake::run(&v["arguments"], true, path),
                "list_notes" => serde_json::from_slice(&fs::read(path).map_err(|e| e.to_string())?)
                    .map_err(|e| e.to_string()),
                "search_notes" => {
                    let notes: Vec<Value> =
                        serde_json::from_slice(&fs::read(path).map_err(|e| e.to_string())?)
                            .map_err(|e| e.to_string())?;
                    let q = v["arguments"]["query"]
                        .as_str()
                        .ok_or("query required")?
                        .to_lowercase();
                    Ok(json!(
                        notes
                            .into_iter()
                            .filter(|n| format!(
                                "{} {}",
                                n["title"].as_str().unwrap_or(""),
                                n["body"].as_str().unwrap_or("")
                            )
                            .to_lowercase()
                            .contains(&q))
                            .collect::<Vec<_>>()
                    ))
                }
                _ => Err("Unsupported local test tool".into()),
            }
        })();
        println!(
            "{}",
            match result {
                Ok(v) => json!({"result":v}),
                Err(e) => json!({"error":e}),
            }
        );
        return;
    }
    let server = Server::http("127.0.0.1:8787").unwrap();
    println!("Viki → http://127.0.0.1:8787");
    for mut request in server.incoming_requests() {
        let method = request.method().as_str().to_owned();
        let url = request.url().to_owned();
        if method != "GET"
            && request.headers().iter().any(|h| {
                h.field.equiv("Origin")
                    && h.value.as_str() != "http://127.0.0.1:8787"
                    && h.value.as_str() != "http://localhost:8787"
            })
        {
            let _ = request.respond(Response::from_string("Origin rejected").with_status_code(403));
            continue;
        }
        let (body, mime, status) = match (method.as_str(), url.as_str()) {
            ("POST", "/api/discover" | "/api/import") => {
                let mut body = String::new();
                let _ = request.as_reader().take(100_000).read_to_string(&mut body);
                match serde_json::from_str::<Value>(&body)
                    .map_err(|e| e.to_string())
                    .and_then(|v| intake::run(&v, url == "/api/import", path))
                {
                    Ok(v) => (v.to_string(), "application/json", 200),
                    Err(e) => (json!({"error":e}).to_string(), "application/json", 400),
                }
            }
            ("GET", "/") => (
                include_str!("../assets/index.html").to_string(),
                "text/html",
                200,
            ),
            ("GET", "/intake.js") => (
                include_str!("../assets/intake.js").into(),
                "text/javascript",
                200,
            ),
            ("GET", "/api/notes") => (fs::read_to_string(path).unwrap(), "application/json", 200),
            ("GET", "/marked.js") => (
                include_str!("../assets/marked.min.js").into(),
                "text/javascript",
                200,
            ),
            ("GET", "/purify.js") => (
                include_str!("../assets/purify.min.js").into(),
                "text/javascript",
                200,
            ),
            ("GET", "/markup.js") => (
                include_str!("../assets/markup.js").into(),
                "text/javascript",
                200,
            ),
            ("POST", "/api/notes") => {
                let mut body = String::new();
                let _ = request.as_reader().take(100_000).read_to_string(&mut body);
                match serde_json::from_str::<Value>(&body) {
                    Ok(v)
                        if v["title"].as_str().is_some_and(|s| !s.trim().is_empty())
                            && v["body"].is_string() =>
                    {
                        let mut notes: Vec<Value> =
                            serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap();
                        let id = std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_micros() as u64;
                        notes.push(json!({"id":id,"title":v["title"],"body":v["body"],"tags":["inbox"],"links":[]}));
                        fs::write("data/notes.tmp", serde_json::to_vec_pretty(&notes).unwrap())
                            .unwrap();
                        fs::rename("data/notes.tmp", path).unwrap();
                        (json!({"id":id}).to_string(), "application/json", 201)
                    }
                    _ => (
                        "{\"error\":\"Title and body required\"}".into(),
                        "application/json",
                        400,
                    ),
                }
            }
            (method @ ("PUT" | "DELETE"), url) if url.starts_with("/api/notes/") => {
                let id = url.trim_start_matches("/api/notes/").parse::<u64>().ok();
                let mut notes: Vec<Value> =
                    serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap();
                if let Some(index) = notes.iter().position(|n| n["id"].as_u64() == id) {
                    let mut body = String::new();
                    let _ = request.as_reader().take(100_000).read_to_string(&mut body);
                    let input = serde_json::from_str::<Value>(&body).unwrap_or(Value::Null);
                    if method == "DELETE" {
                        notes.remove(index);
                        for n in &mut notes {
                            if let Some(links) = n["links"].as_array_mut() {
                                links.retain(|v| v.as_u64() != id);
                            }
                        }
                    } else if input["title"]
                        .as_str()
                        .is_some_and(|s| !s.trim().is_empty())
                        && input["body"].is_string()
                    {
                        notes[index]["title"] = input["title"].clone();
                        notes[index]["body"] = input["body"].clone();
                        if let Some(links) = input["links"].as_array() {
                            let valid: Vec<Value> = links
                                .iter()
                                .filter(|v| {
                                    v.as_u64() != id && notes.iter().any(|n| n["id"] == **v)
                                })
                                .cloned()
                                .collect();
                            notes[index]["links"] = json!(valid);
                        }
                    } else {
                        let _ = request
                            .respond(Response::from_string("Invalid note").with_status_code(400));
                        continue;
                    }
                    fs::write("data/notes.tmp", serde_json::to_vec_pretty(&notes).unwrap())
                        .unwrap();
                    fs::rename("data/notes.tmp", path).unwrap();
                    ("{}".into(), "application/json", 200)
                } else {
                    ("Not found".into(), "text/plain", 404)
                }
            }
            _ => ("Not found".into(), "text/plain", 404),
        };
        let _ = request.respond(
            Response::from_string(body)
                .with_status_code(status)
                .with_header(Header::from_bytes("Content-Type", mime).unwrap()),
        );
    }
}
use std::io::Read;
