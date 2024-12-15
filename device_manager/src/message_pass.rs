use crate::config::AppConfig;
use crate::mqtt::consumer::MqttConsumer;
use std::sync::Arc;
use std::time::Duration;

pub fn message_consumer(config: Arc<AppConfig>) {
    let ports = serialport::available_ports().expect("No ports found!");
    let port_name = ports
        .iter()
        .find(|p| {
            println!("Found port: {:?}", p);
            p.port_name.contains("USB") || p.port_name.contains("ACM")
        })
        .map(|p| p.port_name.clone())
        .expect("No suitable USB port found!");

    println!("Using port: {}", port_name);

    let mut port = serialport::new(port_name, 57600)
        .timeout(Duration::from_millis(1))
        .open()
        .expect("Failed to open serial port");

    let mut consumer = MqttConsumer::new(config.mqtt.clone()).unwrap();
    consumer.subscribe("tasks").unwrap();

    loop {
        std::thread::sleep(Duration::from_millis(1));
        if let Some(msg) = consumer.try_next().unwrap() {
            let payload_str = String::from_utf8_lossy(&msg.payload);
            println!("Received raw message: {}", payload_str);

            let formatted_message = format!("{}\n", payload_str.trim());
            println!("Sending to Arduino: {:?}", formatted_message);

            if let Err(e) = port.clear(serialport::ClearBuffer::All) {
                eprintln!("Failed to clear buffer: {}", e);
            }

            match port.write_all(formatted_message.as_bytes()) {
                Ok(_) => {
                    if let Err(e) = port.flush() {
                        eprintln!("Failed to flush port: {}", e);
                    }
                }
                Err(e) => {
                    eprintln!("Failed to write to serial port: {}", e);
                    break;
                }
            }
        }
    }
}
