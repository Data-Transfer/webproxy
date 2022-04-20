use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::{convert::Infallible, net::SocketAddr};

async fn handle(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let headers = req.headers();
    println!("{:?}", headers);
    let mut h = String::new();
    h += r#"<!DOCTYPE html>
            <html lang="en">
            <head></head>
            <body>"#;
    //h += "<ul>\n";
    //for (k, v) in headers {
    //    h += format!("<li>{}: {}</li>\n", k, v.to_str().unwrap()).as_str();
    //}
    let uri = req.uri().to_string();
    let method = req.method().to_string();
    h += format!(
        r#"<table>
            <tr><td>URI:<td><td>{}</td></tr>
            <tr><td>Method:<td><td>{}</td></tr>
           </table>
        "#,
        uri, method
    )
    .as_str();
    h += "<p/><h2>Headers</h2><p/>\n";
    h += r#"<table><tr><th>Name</th><th>Value</th></tr>"#;
    for (k, v) in headers {
        h += "<tr>\n";
        let value = match v.to_str() {
            Ok(s) => s,
            Err(_) => "",
        };
        h += format!("<td>{}</td> <td>{}</td>\n", k, value).as_str();
        h += "</tr>\n";
    }

    h += "</table>\n";
    h += "</body>\n</html>";
    Ok(Response::new(h.into()))
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));

    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle)) });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
