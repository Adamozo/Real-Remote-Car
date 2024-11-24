from typing import Optional
from pathlib import Path
from dotenv import load_dotenv
import os


class Config:
    BROKER_URL: str = None
    MAIN_TOPIC: str = None
    BROKER_USER: str = None
    BROKER_PASSWORD: str = None
    CONTROLLER_INTERFACE: str = None

    _instance = None
    _initialized = False

    def __new__(cls):
        if cls._instance is None:
            cls._instance = super().__new__(cls)
        return cls._instance

    def __init__(self):
        if not self._initialized:
            self._load_config()
            self._initialized = True

    def _load_config(self, path: str = '.env') -> None:
        env_path = Path(path)
        if not env_path.exists():
            raise FileNotFoundError(f"Unable to find: {path}")

        load_dotenv(env_path)

        class_attributes = [
            attr for attr in dir(self.__class__) if not attr.startswith('_')
            and not callable(getattr(self.__class__, attr))
        ]

        for attr_name in class_attributes:
            env_value = os.getenv(attr_name)
            if env_value is not None:
                setattr(self, attr_name, env_value)

    def _convert_value(self, value: str, target_type: type) -> any:
        if target_type == bool:
            return value.lower() in ('true', '1', 'yes', 'y', 'on')
        if target_type == int:
            return int(value)
        if target_type == float:
            return float(value)
        return value


config = Config()
