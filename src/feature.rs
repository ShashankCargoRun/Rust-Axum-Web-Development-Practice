Response {
    if current_user.is_authenticated {
        return Redirect::to("/todos").into_response();
    }

    next.run(req).await
}

fn on_response(response: &Response<Body>, latency: Duration, _: &Span) {
    tracing::info!(
        "<- Response generated: status {} in {:?}",
        response.status(),
        latency
    )
}