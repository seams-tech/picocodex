use super::*;

#[tokio::test]
async fn missing_stored_checkpoint_replays_local_history_once() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let endpoint = format!("ws://{}", listener.local_addr()?);
    let server = tokio::spawn(async move {
        let (stream, _) = listener.accept().await?;
        let mut root = accept_async(stream).await?;
        assert_warmup_with_store(&next_json(&mut root).await?, true);
        send_warmup(&mut root, "resp-warmup").await?;
        let first = next_json(&mut root).await?;
        send_final(&mut root, "resp-first").await?;

        let (stream, _) = listener.accept().await?;
        let mut branch = accept_async(stream).await?;
        let checkpoint = next_json(&mut branch).await?;
        assert_eq!(checkpoint["previous_response_id"], "resp-first");
        assert_eq!(checkpoint["input"].as_array().map(Vec::len), Some(1));
        send_json(
            &mut branch,
            json!({
                "type": "error",
                "error": {
                    "code": "previous_response_not_found",
                    "message": "checkpoint expired"
                }
            }),
        )
        .await?;

        let replay = next_json(&mut branch).await?;
        assert!(replay.get("previous_response_id").is_none());
        assert_eq!(replay["store"], true);
        assert_eq!(replay["input"][0]["type"], "additional_tools");
        assert_eq!(replay["input"][1]["role"], "developer");
        let replay_text = replay.to_string();
        assert!(replay_text.contains("root prompt"));
        assert!(replay_text.contains("branch after eviction"));
        assert!(
            replay["input"]
                .as_array()
                .is_some_and(|items| items.len() > 4)
        );
        send_final(&mut branch, "resp-replayed").await?;
        drop((root, first));
        Result::<()>::Ok(())
    });

    let workspace = temporary_workspace("checkpoint-miss")?;
    let openai = OpenAi::builder("test-key")
        .websocket_url(endpoint)
        .store(true)
        .build()?;
    let (agent, root_events) = Picocodex::builder(openai)
        .thinking(Thinking::Low)
        .workspace(&workspace)
        .session_id(test_session_id())
        .build()?;
    let first = agent
        .prompt(Prompt::new("root prompt"))
        .await?
        .result()
        .await?;
    let (fork, mut fork_events) = agent.fork_from(&first).await?;
    let branch = fork.prompt("branch after eviction").await?;
    assert_eq!(branch.result().await?.final_message(), "done");

    drop((agent, fork, root_events));
    let mut observed_checkpoint_retry = false;
    while let Some(event) = fork_events.recv().await {
        if event.kind == AgentEventKind::ModelAttemptRetrying {
            let payload = event.decode_payload::<Value>()?;
            observed_checkpoint_retry = payload["error_class"] == "checkpoint_missing"
                && payload["replay_mode"] == "full_history"
                && payload["opens_new_socket"] == false;
        }
    }
    assert!(observed_checkpoint_retry);
    timeout(std::time::Duration::from_secs(5), server)
        .await
        .map_err(|_| eyre!("mock Responses server did not finish"))???;
    std::fs::remove_dir_all(workspace)?;
    Ok(())
}

#[tokio::test]
async fn serialized_session_resumes_over_ephemeral_https() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let endpoint = format!("http://{}", listener.local_addr()?);
    let server = tokio::spawn(async move {
        let first = next_http_json(&listener).await?;
        assert_eq!(first.body["store"], false);
        assert!(first.body.get("previous_response_id").is_none());
        assert!(first.body.to_string().contains("first prompt"));
        send_http_final(first.stream, "resp-first").await?;

        let resumed = next_http_json(&listener).await?;
        assert_eq!(resumed.body["store"], false);
        assert!(resumed.body.get("previous_response_id").is_none());
        let replay = resumed.body.to_string();
        assert!(replay.contains("first prompt"));
        assert!(replay.contains("done"));
        assert!(replay.contains("resume prompt"));
        send_http_final(resumed.stream, "resp-resumed").await
    });

    let workspace = temporary_workspace("serialized-resume-https")?;
    let openai = OpenAi::builder("test-key")
        .transport(ResponsesTransport::Https)
        .store(false)
        .api_base_url(endpoint.clone())
        .build()?;
    let (agent, events) = Picocodex::builder(openai)
        .instructions("durable instructions")
        .thinking(Thinking::Low)
        .workspace(&workspace)
        .prompt_cache_key("durable-cache")
        .build()?;
    let first = agent.prompt("first prompt").await?.result().await?;
    let snapshot: SessionSnapshot =
        serde_json::from_slice(&serde_json::to_vec(&first.snapshot())?)?;
    drop((agent, events, first));

    let mismatched = Picocodex::builder(
        OpenAi::builder("test-key")
            .transport(ResponsesTransport::Https)
            .store(false)
            .api_base_url(endpoint.clone())
            .build()?,
    )
    .model(Model::Luna)
    .instructions("durable instructions")
    .thinking(Thinking::Low)
    .resume(snapshot.clone())
    .build();
    assert!(matches!(
        mismatched,
        Err(PicocodexError::InvalidSessionSnapshot(message))
            if message.contains("does not match configured model")
    ));

    let openai = OpenAi::builder("test-key")
        .transport(ResponsesTransport::Https)
        .store(false)
        .api_base_url(endpoint)
        .build()?;
    let (resumed, resumed_events) = Picocodex::builder(openai)
        .instructions("durable instructions")
        .thinking(Thinking::Low)
        .resume(snapshot)
        .build()?;
    assert_eq!(
        resumed
            .prompt("resume prompt")
            .await?
            .result()
            .await?
            .final_message(),
        "done"
    );

    drop((resumed, resumed_events));
    timeout(std::time::Duration::from_secs(5), server)
        .await
        .map_err(|_| eyre!("mock HTTPS Responses server did not finish"))???;
    std::fs::remove_dir_all(workspace)?;
    Ok(())
}
