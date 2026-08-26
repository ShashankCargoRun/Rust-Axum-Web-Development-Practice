Response {
    if current_user.is_authenticated {
        return Redirect::to("/todos").into_response();
    }

    next.run(req).await
}