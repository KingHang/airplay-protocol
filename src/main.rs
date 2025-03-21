use std::net::SocketAddr;
use std::sync::Arc;
use std::fs::File;
use std::io::Write;
use airplay2_protocol::airplay::airplay_consumer::{AirPlayConsumer, ArcAirPlayConsumer};
use airplay2_protocol::airplay::server::AudioPacket;
use airplay2_protocol::airplay::AirPlayConfigBuilder;
use airplay2_protocol::airplay_bonjour::AirPlayBonjour;
use airplay2_protocol::control_handle::ControlHandle;
use airplay2_protocol::net::server::Server as MServer;

struct VideoConsumer;

impl AirPlayConsumer for VideoConsumer {
    fn on_video(&self, _bytes: &[u8]) {
        log::info!("on_video...");
    }

    fn on_video_format(
        &self,
        _video_stream_info: airplay2_protocol::airplay::lib::video_stream_info::VideoStreamInfo,
    ) {
        log::info!("on_video format...");
    }

    fn on_video_src_disconnect(&self) {
        log::info!("on_video disconnect...");
    }

    fn on_audio_format(
        &self,
        _audio_stream_info: airplay2_protocol::airplay::lib::audio_stream_info::AudioStreamInfo,
    ) {
        log::info!("on_audio_format...");
    }

    fn on_audio(&self, _bytes: &AudioPacket) {
        log::info!("on_audio...");
    }

    fn on_audio_src_disconnect(&self) {
        log::info!("on audio disconnect");
    }

    fn on_volume(&self, volume: f32) {
        log::info!("volume = {volume}");
    }
}

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    // 创建一个文件用于日志输出
    let file = File::create("log.txt").unwrap();
    let mut writer = std::io::BufWriter::new(file);

    // 初始化env_logger
    env_logger::Builder::new()
        .format(|buf, record| writeln!(buf, "{}", record.args())) // 设置日志格式
        .target(env_logger::Target::Pipe(Box::new(writer))) // 设置日志输出目标为文件
        .filter(None, log::LevelFilter::Trace) // 设置日志级别
        .init(); // 初始化env_logger

    let port = 31927;
    let name = "RustAirplay";

    let _air = AirPlayBonjour::new(name, port, false);

    let addr: SocketAddr = ([0, 0, 0, 0], port).into();
    let airplay_config = AirPlayConfigBuilder::new(name.to_string())
        .width(1920)
        .height(1080)
        .fps(30)
        .volume(0.5)
        //.pin_pwd("123321")
        .build();
    let video_consumer: ArcAirPlayConsumer = Arc::new(VideoConsumer);
    let mserver = MServer::bind_with_addr(
        addr,
        ControlHandle::new(airplay_config, video_consumer.clone(), video_consumer),
    )
    .await;
    mserver.run().await?;
    Ok(())
}
