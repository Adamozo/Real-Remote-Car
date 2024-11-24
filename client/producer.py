import paho.mqtt.client as mqtt
from pynput import keyboard
import time
import json
from config import config

client = mqtt.Client()

client.username_pw_set(username=config.BROKER_USER,
                       password=config.BROKER_PASSWORD)
client.connect(config.BROKER_URL)


def on_press(key):
    try:
        key_char = key.char if hasattr(key, 'char') else str(key)

        send_time = time.time_ns()

        message = json.dumps({"key": key_char, "send_time": send_time})

        client.publish(config.MAIN_TOPIC, message)
        print(f"Sent key: {key_char} at {send_time}")

    except AttributeError:
        pass


with keyboard.Listener(on_press=on_press) as listener:
    listener.join()
