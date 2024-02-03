use std::env;
use poise::serenity_prelude as serenity;

struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx, .. } => {
            println!("Error in command `{}`: {:?}", ctx.command().name, error);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {}", e)
            }
        }
    }
}

#[poise::command(
    prefix_command,
    track_edits,
    slash_command,
    aliases("disrust"),
    required_permissions = "SEND_MESSAGES",
    on_error = "on_error"
)]
async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            extra_text_at_bottom: "Example text",
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let token = env::var("DTOK").unwrap_or_else(|e| {
        eprintln!("Error reading token: {}", e);
        std::process::exit(1);
    });

    let intents = serenity::GatewayIntents::non_privileged()
        | serenity::GatewayIntents::MESSAGE_CONTENT;

    let poise = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![help()],
            on_error: |e| Box::pin(on_error(e)),
            ..Default::default()
        }).setup(move |ctx, ready, framework| {
            Box::pin(async move {
                println!("Logged in as {}", ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        }).build();

    let client = serenity::ClientBuilder::new(token, intents).framework(poise).await;

    client.unwrap().start().await.unwrap();
}