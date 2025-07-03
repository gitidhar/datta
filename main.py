from pathlib import Path
from dotenv import load_dotenv
import os

# point load_dotenv() at your project-level .env  ──► defaults to current dir
env_path = Path(__file__).resolve().parent / ".env"
load_dotenv(env_path, override=False)   # override=True if you want .env to win

openai_api_key = os.getenv("OPENAI_API_KEY")   # returns None if missing

print("Hello world")