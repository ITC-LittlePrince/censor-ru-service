use clap::Parser;
use futures_util::{StreamExt, TryStreamExt};
use std::time::Duration;
use tokio::runtime::Builder;
use wreq::Client;
use wreq_util::Emulation;
use tokio::io::{AsyncWriteExt, BufWriter};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "./blocklist-ru-domain.txt")]
    output: String,
}

fn main() {
	let args = Args::parse();
    let blocklist_file_path_0 = args.output.clone();
    let blocklist_file_path_1 = args.output.clone();

    println!("Используемый путь к файлу: {}", blocklist_file_path_0.clone());

    let rt0 = Builder::new_multi_thread()
        .worker_threads(3)
        .enable_all()
        .build()
        .unwrap();

    rt0.spawn(async move {
	    match tokio::fs::remove_file(blocklist_file_path_0.clone()).await {
	        Ok(_) => println!("Файл успешно очищен"),
			Err(err0) if err0.kind() == std::io::ErrorKind::NotFound => {
				println!("Цензор запущен впервые. Не забудьте добавить в Blocky конфигурацию этот файл: {:?}", blocklist_file_path_0.clone());
			},
	        Err(err0) => eprintln!("Ошибка при удалении файла: {}", err0),
	    }
    });

   	println!("start parsed list");
    rt0.block_on(async move {
        let body = wreq::Client::new()
            .get("https://raw.githubusercontent.com/hxehex/russia-mobile-internet-whitelist/refs/heads/main/whitelist.txt")
            .send()
            .await.unwrap()
            .text()
            .await.unwrap();

       	let mut fs0 = tokio::fs::File::options().write(true).create(true).open(blocklist_file_path_1.clone()).await.unwrap();
        let mut writer = BufWriter::new(fs0);

        for line0 in body.lines() {
        	// println!("{:?}", format!("0.0.0.0 {}", line0.trim()));
           	let line_content = format!("0.0.0.0 {}", line0.trim());
            writer.write(format!("0.0.0.0 {}\n", line0.trim()).as_bytes()).await;
            writer.flush().await;
        }
    });
   	println!("complited parsed list");
}
