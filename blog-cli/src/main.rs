use std::fs;
use std::path::Path;

use blog_client::BlogClient;
use clap::{Parser, Subcommand};

const DEFAULT_HTTP_SERVER: &str = "http://127.0.0.1:8080";
const TOKEN_FILE: &str = ".blog_token";

#[derive(Debug, Parser)]
#[command(name = "blog-cli")]
#[command(about = "CLI-клиент для управления блогом")]
struct Cli {
    #[arg(long, default_value = DEFAULT_HTTP_SERVER, global = true)]
    server: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Register {
        #[arg(long)]
        username: String,

        #[arg(long)]
        email: String,

        #[arg(long)]
        password: String,
    },

    Login {
        #[arg(long)]
        username: String,

        #[arg(long)]
        password: String,
    },

    Create {
        #[arg(long)]
        title: String,

        #[arg(long)]
        content: String,
    },

    Get {
        #[arg(long)]
        id: i64,
    },

    Update {
        #[arg(long)]
        id: i64,

        #[arg(long)]
        title: String,

        #[arg(long)]
        content: String,
    },

    Delete {
        #[arg(long)]
        id: i64,
    },

    List {
        #[arg(long, default_value_t = 10)]
        limit: i64,

        #[arg(long, default_value_t = 0)]
        offset: i64,
    },
}

#[tokio::main]
async fn main() {
    if let Err(error) = run().await {
        eprintln!("Ошибка: {error}");
        std::process::exit(1);
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let mut client = BlogClient::new_http(cli.server)?;

    if let Some(token) = load_token()? {
        client.set_token(token);
    }

    match cli.command {
        Commands::Register {
            username,
            email,
            password,
        } => {
            let response = client.register(username, email, password).await?;

            save_token(&response.token)?;

            println!("Пользователь зарегистрирован:");
            println!("ID: {}", response.user.id);
            println!("Имя: {}", response.user.username);
            println!("Email: {}", response.user.email);
        }

        Commands::Login { username, password } => {
            let response = client.login(username, password).await?;

            save_token(&response.token)?;

            println!("Вход выполнен:");
            println!("ID: {}", response.user.id);
            println!("Имя: {}", response.user.username);
        }

        Commands::Create { title, content } => {
            let post = client.create_post(title, content).await?;

            println!("Пост создан:");
            print_post(&post);
        }

        Commands::Get { id } => {
            let post = client.get_post(id).await?;

            print_post(&post);
        }

        Commands::Update { id, title, content } => {
            let post = client.update_post(id, title, content).await?;

            println!("Пост обновлён:");
            print_post(&post);
        }

        Commands::Delete { id } => {
            client.delete_post(id).await?;

            println!("Пост {id} удалён");
        }

        Commands::List { limit, offset } => {
            let posts = client.list_posts(limit, offset).await?;

            if posts.is_empty() {
                println!("Посты не найдены");
            } else {
                for post in &posts {
                    print_post(post);
                    println!("--------------------");
                }
            }
        }
    }

    Ok(())
}

fn save_token(token: &str) -> Result<(), std::io::Error> {
    fs::write(TOKEN_FILE, token)
}

fn load_token() -> Result<Option<String>, std::io::Error> {
    if !Path::new(TOKEN_FILE).exists() {
        return Ok(None);
    }

    let token = fs::read_to_string(TOKEN_FILE)?;
    let token = token.trim().to_owned();

    if token.is_empty() {
        Ok(None)
    } else {
        Ok(Some(token))
    }
}

fn print_post(post: &blog_client::models::Post) {
    println!("ID: {}", post.id);
    println!("Автор: {}", post.author_id);
    println!("Заголовок: {}", post.title);
    println!("Содержание: {}", post.content);
    println!("Создан: {}", post.created_at);
    println!("Обновлён: {}", post.updated_at);
}
