import paho.mqtt.client as mqtt
import time
import json
from config import config


def on_message(client, userdata, message):
    data = json.loads(message.payload.decode())
    key_char = data["key"]
    send_time = data["send_time"]

    receive_time = time.time_ns()
    latency_ms = (receive_time - send_time) / 1_000_000

    print(f"Received key: {key_char} with latency: {latency_ms:.3f} ms")


client = mqtt.Client()

client.username_pw_set(username=config.BROKER_USER,
                       password=config.BROKER_PASSWORD)
client.on_message = on_message
client.connect(config.BROKER_URL)

client.subscribe(config.MAIN_TOPIC)

client.loop_forever()
